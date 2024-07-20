# Brief Introduction to Multithreading in Rust to Improve Render Performance

- July 11th, 2024. Yuxuan Wang

---

### What will we cover?

1. **Rust Language Feature for Multithreading**: Basics.

2. **Race Condition $\to$ Mutex / Atomic / Condition Variable**: Only minimum knowledge is necessary since the parallel computation here is rather simple.

3. **Job Partition And Task Strategy**: How to efficienctly parition render task into subtasks with better efficiency? - Elminate stragglers.

---
### Part 1: Rust Language Feature for Multithreading

#### I. std::thread/corssbream::thread

简单起见，不建议使用 std::thread，因为其会强行要求所有 move 进子线程的变量具有 "static" 的生命周期。

这对于采用引用传入的参数来说很麻烦，比如 `Camera::render(&self, world: &Object)`中的 self 和 world 都是无法移入子线程的，即使采用 Arc 也一样。

而 static 的生命周期很多时候是不必要的，比如我逻辑上保证在 rend 函数结束前所有的线程都会 join。

我们采用 crossbeam:\:thread::scope 来生成线程。
用法为：
```rust
crossbeam::thread::scope(move |thread_spawner| {
  ... // some instructions.
  for i in 0..8 {
    thread_spawner.spawn(move || { // invoke a thread.
    ... 
    });
  }
}).unwrap() // threads invoked within the scope will automatically join here.
```

所有 move 变量的生命周期只需要持续到 scope 结束就好了，且其中 spawn 出来的线程会在 scope 结束时自动 join，不需要手动管理。

#### II. std::sync::Arc

Arc 就是一个 thread-safe 的 Rc/shared_ptr。如果不想把每个参数都复制进子线程一次的话，请使用 Arc 传递。

用法大致如下：

```rust

pub fn render(&self, world: Object) -> RgbImage {
  let camera_wrapper = Arc::new(self); // Arc<&Camera>，注意内部包装的是 ref
  let world_wrapper = Arc::new(&world); // Arc<&Object>
  crossbeam::thread::scope(move |thread_spawner| {
    for i in 0..8 {
      let camera_wrapper = Arc::clone(world_wrapper); // 每一个子线程需要重新 clone 一个 Arc，相当于引用计数 + 1。
      let world_wrapper = Arc::clone(world_wrapper);
      thread_spawner.spawn(move || {
        camera_wrapper.render_sub(world_wrapper, ...); // world_wrapper (Arc<&Object>) 可以自动转化成 &Object
      });
    }
  }).unwrap() 
}
```

#### III. Send + Sync trait

可以参考：
https://course.rs/advance/concurrency-with-threads/send-sync.html

简单来说，这俩货只是告诉编译器：我是线程安全的，但是我怎么实现线程安全的你别管。

也就是说，如果你遇到了缺少 Send + Sync 的编译错误，可以写
```rust
unsafe impl Send for ImageTexture {} 
// ImageTexture 用了 opencv 有 raw pointer, 
// 所以没有默认实现的 Send 和 Sync.
unsafe impl Sync for ImageTexture {}
```

以及相信各位在实现 Rust 中 "多态" 的时候已经定义了类似
```rust 
pub type Object = Rc<dyn HittableTrait>
```
可以将其改造成多线程版本：
```rust
pub type Object = Arc<dyn HittableTrait + Send + Sync>
```
然后全局替换 Rc 为 Arc，这俩在语法上是完全可互换的。(最多出现一些 Send 和 Sync 缺失的编译错误)。

---
### Part 2: Race Condition $\to$ Mutex / Atomic / Condition Variable

#### I. Mutex
相信大家在 Trie 树项目中对于 C++ 的 mutex 已经有了一定了解。Rust 的 Mutex 和 C++ 类似，只是其一定包裹了一个数据对象，不是独立的锁。

使用例：
```rust
use std::sync::Mutex;
impl Camera {
  pub fn render() -> RbgImage {
    let mut img = RbgImage::new(...);
    let img_mtx = Arc::new(Mutex::new(&mut img)); // wrap with &mut
    cross:beam{
      ...
      let img_mtx = Arc::clone(img_mtx);
      thread_spawner.spawn(move || {
        camera.render_sub(..., img_mtx, ...);
      });
    }
  }

  pub fn render_sub(..., img_mtx: Mutex<&mut RgbImage>, ...) {
    ...
    let mut img: RbgImage = img_mtx.lock().unwrap(); // 相当于 lock_guard, 会自动就解锁。
    ...
  }
}
```

#### II. Atomic & Condition Variable
Atomic 无锁的数据共享(当然只基础一些基础类)，比锁快，但是比普通基础类慢。

Conodition Variable 信号通信量，用来 wait 和 notify。

具体函数问 GPT 吧。这里只需要在一个地方用到，可以直接借鉴我的。

---

### Part 3: Job Partition And Task Strategy

### I. Naive yet feasible strategy

定义：
```rust
const HEIGHT_PARTITION: usize = 4; // multithreading parameters
const WIDTH_PARTITION: usize = 4;
const THREAD_LIMIT: usize = 16;
```
分别表示纵向分割数，横向分割数。

然后直接划分为 HEIGHT_PARTION * WIDTH_PARTION 块，发给子线程去渲染。

```rust
  let chunk_height = (self.image_height + HEIGHT_PARTITION - 1) / HEIGHT_PARTITION;
  let chunk_width = (self.image_width + WIDTH_PARTITION - 1) / WIDTH_PARTITION;

  for i in 0...WIDTH_PARITION {
    for j in 0...HEIGHT_PARITION {
      ...
        thread.spawn( move || {
          camera.render_sub(
            ..., 
            i * chunk_width, (i + 1) * chunk_width,
            j * chunk_height, (j + 1) * chunk_height
          )
        }
        );
    }
  }
```

子线程的实现也非常 naive:
```rust
  pub fn render_sub(&self, 
    world: &Object, 
    img_mtx: &Mutex<&mut RgbImage>, 
    bar: &ProgressBar, 
    x_min: usize, x_max: usize, 
    y_min: usize, y_max: usize) {
    // Step1, 把 x_max 和 image_width, y_max 和 image_height 取个 min。
    // Step2, 渲染 [x_min, x_max] * [y_min, y_max] 之间的像素，写入临时 buff 中。
    // Step3, 渲染完后，获取 img 的锁，把 buff 写入相应区域。
  }
```

显然渲染一个像素就写入 img 会导致锁竞争太厉害，所以暂时写入 buff 最后合并。

另外，ProgressBar 似乎是线程安全的，但是速度有点慢，建议渲染完一行再更新一次。

**这个简单策略存在明显问题：不同区域的渲染任务计算量显著不同，整个程序会被最慢的线程(Straggler)拖慢。**

### II. Finer Granularity with Thread Number Control

由于难以在实际渲染前估计每个区域的工作量，一个简单的消除 Straggler 方法就是把子任务划分的更细。

修改：
```Rust
const HEIGHT_PARTITION: usize = 20; // multithreading parameters
const WIDTH_PARTITION: usize = 20;
const THREAD_LIMIT: usize = 16;
```

不过这样会有 400 个线程同时启动，效率其实并不好。

简单的策略是，在负责分发的 render 函数中，添加限制：
- 如果当前在运行的线程数 $\ge$ THREAD_LIMIT, 则先等待，不发放新任务。

实现起来也很简单：
- 增加一个 AtomicUsize thread_count, 在发布任务时 +1，子线程退出时 -1。
- 增加一个 CondVar thread_number_controler: 
  - 在 rend 发布新任务前若 thread_count $\ge$ THREAD_LIMIT 则 wait。
  - 在子线程退出时 notify。

可能...主要的困难在于 Rust 的语法问题，给大家借鉴一下核心控制代码大概是：
```rust
   
    thread::scope(move |thd_spawner|{
      let thread_count = Arc::new(AtomicUsize::new(0));
      let thread_number_controller = Arc::new(Condvar::new());

      ...// some Arc::new(&...)
      for j in 0..HEIGHT_PARTITION {
        for i in 0..WIDTH_PARTITION {
          // WAIT
          let lock_for_condv = Mutex::new(false);
          while !(thread_count.load(Ordering::SeqCst) < THREAD_LIMIT) { // outstanding thread number control
            thread_number_controller.wait(lock_for_condv.lock().unwrap()).unwrap();
          }
          ... // some Arc::clone(..._wrapper)          

          // move "thread_count++" out of child thread, so that it's sequential with thread number control code
          thread_count.fetch_add(1, Ordering::SeqCst);
          bar.set_message(format!("|{} threads outstanding|", thread_count.load(Ordering::SeqCst))); // set "thread_count" information to progress bar

          let _ = thd_spawner.spawn(move |_| {
            camera.render_sub(&world, &img_mtx, &bar, 
              i * chunk_width, (i + 1) * chunk_width, 
              j * chunk_height, (j + 1) * chunk_height);

            thread_count.fetch_sub(1, Ordering::SeqCst); // subtract first, then notify.
            bar.set_message(format!("|{} threads outstanding|", thread_count.load(Ordering::SeqCst)));
            // NOTIFY
            thread_number_controller.notify_one();
          });

        }
      }
    }).unwrap();

```

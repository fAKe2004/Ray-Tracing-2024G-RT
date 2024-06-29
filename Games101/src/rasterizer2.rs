use std::collections::HashMap;
use rand::Rng;

use chrono::{DateTime, Duration, Local};
use nalgebra::{Matrix4, Vector, Vector2, Vector3, Vector4};
use crate::triangle::{Aabb, Triangle};

#[allow(dead_code)]
pub enum Buffer {
    Color,
    Depth,
    Both,
}

#[allow(dead_code)]
pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Default, Clone)]
pub struct Rasterizer {
    model: Matrix4<f64>,
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    pos_buf: HashMap<usize, Vec<Vector3<f64>>>,
    ind_buf: HashMap<usize, Vec<Vector3<usize>>>,
    col_buf: HashMap<usize, Vec<Vector3<f64>>>,

    frame_buf: Vec<Vector3<f64>>,
    depth_buf: Vec<f64>,
    /*  You may need to uncomment here to implement the MSAA method  */

    // MSAA
    frame_sample: Vec<Vector3<f64>>,
    depth_sample: Vec<f64>,

    // TAA
    last_frame_buf:  Vec<Vector3<f64>>,
    first_render_flag: bool,
    
    width: u64,
    height: u64,
    next_id: usize,
}

#[derive(Clone, Copy)]
pub struct PosBufId(usize);

#[derive(Clone, Copy)]
pub struct IndBufId(usize);

#[derive(Clone, Copy)]
pub struct ColBufId(usize);

pub const antialiasing_method:&str = "NoAA";

impl Rasterizer {
    pub fn new(w: u64, h: u64) -> Self {
        let mut r = Rasterizer::default();
        r.width = w;
        r.height = h;
        
        match antialiasing_method {
            
            "MSAA" => {   
                r.frame_buf.resize((w * h) as usize, Vector3::zeros());
                r.frame_sample.resize((w * h * 4) as usize, Vector3::zeros());
                r.depth_sample.resize((w * h * 4) as usize, f64::MAX);
            },
        
            "TAA" => {
                r.frame_buf.resize((w * h) as usize, Vector3::zeros());
                r.depth_buf.resize((w * h) as usize, f64::MAX);                
                r.last_frame_buf.resize((w * h) as usize, Vector3::zeros());
                r.first_render_flag = true;
            },

            _ => {
                r.frame_buf.resize((w * h) as usize, Vector3::zeros());
                r.depth_buf.resize((w * h) as usize, f64::MAX);                
            }
        }
        r
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y as u64) * self.width + x as u64) as usize
    }
    
    fn get_index_MSAA(&self, x: usize, y:usize, sub_index: usize /*0 - 4*/) ->usize {
        4 * self.get_index(x, y) + sub_index
    }

    fn set_pixel(&mut self, point: &Vector3<f64>, color: &Vector3<f64>) {
        let ind = (self.height as f64 - 1.0 - point.y) * self.width as f64 + point.x;
        self.frame_buf[ind as usize] = *color;
    }

    pub fn clear(&mut self, buff: Buffer) {
        match buff {
            Buffer::Color => {
                match antialiasing_method {
                    "MSAA" => {
                        self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                        self.frame_sample.fill(Vector3::new(0.0, 0.0, 0.0));
                    },
                    "TAA" => {
                        std::mem::swap(&mut self.last_frame_buf, &mut self.frame_buf);
                        self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                        // self.first_render_flag = false;
                    },
                    _ => {
                        self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                    },
                }
            }
            Buffer::Depth => {
                self.depth_buf.fill(f64::MAX);
                match antialiasing_method {
                    "MSAA" => {
                        self.depth_sample.fill(f64::MAX);
                    },
                    "TAA" => {
                        self.depth_sample.fill(f64::MAX);
                        // self.first_render_flag = false;                 
                    }
                    _ => {
                        self.depth_buf.fill(f64::MAX);
                    },
                }
            }
            Buffer::Both => {
                
                match antialiasing_method {
                    "MSAA" => {
                        self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));                
                        self.frame_sample.fill(Vector3::new(0.0, 0.0, 0.0));
                        self.depth_sample.fill(f64::MAX);
                    },
                    "TAA" => {
                        std::mem::swap(&mut self.last_frame_buf, &mut self.frame_buf);
                        self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                        self.depth_buf.fill(f64::MAX);
                        // self.first_render_flag = false;
                    }
                    _ => {
                        self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                        self.depth_buf.fill(f64::MAX);
                    },
                }
                
            }
        }
    }

    pub fn set_model(&mut self, model: Matrix4<f64>) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f64>) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Matrix4<f64>) {
        self.projection = projection;
    }

    fn get_next_id(&mut self) -> usize {
        let res = self.next_id;
        self.next_id += 1;
        res
    }

    pub fn load_position(&mut self, positions: &Vec<Vector3<f64>>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.clone());
        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: &Vec<Vector3<usize>>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices.clone());
        IndBufId(id)
    }

    pub fn load_colors(&mut self, colors: &Vec<Vector3<f64>>) -> ColBufId {
        let id = self.get_next_id();
        self.col_buf.insert(id, colors.clone());
        ColBufId(id)
    }

    pub fn draw(&mut self, pos_buffer: PosBufId, ind_buffer: IndBufId, col_buffer: ColBufId, _typ: Primitive) {
        let beginning_time = Local::now();

        let buf = &self.clone().pos_buf[&pos_buffer.0];
        let ind: &Vec<Vector3<usize>> = &self.clone().ind_buf[&ind_buffer.0];
        let col = &self.clone().col_buf[&col_buffer.0];

        let f1 = (50.0 - 0.1) / 2.0;
        let f2 = (50.0 + 0.1) / 2.0;

        let mvp = self.projection * self.view * self.model;

        for i in ind {
            let mut t = Triangle::new();
            let mut v =
                vec![mvp * to_vec4(buf[i[0]], Some(1.0)), // homogeneous coordinates
                     mvp * to_vec4(buf[i[1]], Some(1.0)), 
                     mvp * to_vec4(buf[i[2]], Some(1.0))];
    
            for vec in v.iter_mut() {
                *vec = *vec / vec.w;
            }
            for vert in v.iter_mut() {
                vert.x = 0.5 * self.width as f64 * (vert.x + 1.0);
                vert.y = 0.5 * self.height as f64 * (vert.y + 1.0);
                vert.z = vert.z * f1 + f2;
            }
            for j in 0..3 {
                // t.set_vertex(j, Vector3::new(v[j].x, v[j].y, v[j].z));
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
            }
            let col_x = col[i[0]];
            let col_y = col[i[1]];
            let col_z = col[i[2]];
            t.set_color(0, col_x[0], col_x[1], col_x[2]);
            t.set_color(1, col_y[0], col_y[1], col_y[2]);
            t.set_color(2, col_z[0], col_z[1], col_z[2]);

            self.rasterize_triangle(&t);
        }

        match antialiasing_method {
            "TAA" => {
                if !self.first_render_flag {
                    for i in 0..self.width {
                        for j in 0..self.height {
                            let index = self.get_index(i as usize, j as usize);
                            self.frame_buf[index] = (self.frame_buf[index] + self.last_frame_buf[index]) / 2.0;
                        }
                    }
                    self.first_render_flag = false;
                }
            },
            _ => {
            },
        }

        let ending_time = Local::now();
        println!("Time consumed with {} : {}", antialiasing_method,ending_time.signed_duration_since(beginning_time).num_milliseconds());
    }

    pub fn rasterize_triangle(&mut self, t: &Triangle) {
        /*  implement your code here  */
        match antialiasing_method {
            "MSAA" => self.MSAA_sampling(t),
            "TAA" => self.TAA_sampling(t),
            _ => self.trivial_sampling(t),
        }
    }
        
    fn trivial_sampling(&mut self, t: &Triangle) {
        let aabb = Aabb::new(t);
                
        // println!("This is Aabb, with this {} {} {} {} \n", aabb.xmin, aabb.xmax, aabb.ymin, aabb.ymax);
        for x in aabb.xmin as i32..=aabb.xmax as i32{
            for y in aabb.ymin as i32..=aabb.ymax as i32{
                let index = self.get_index(x as usize, y as usize);
                if inside_triangle(x as f64 + 0.5, y as f64 + 0.5, &aabb.v) {
                    let z = compute_interpolate_depth(x as f64 + 0.5, y as f64 + 0.5, &aabb.v);
                    if z < self.depth_buf[index] {
                        // println!("This is pixel set, hello !!! at {} {} [COL : {}]", x, y, &(t.color[0]));
                        let color = t.color[0] * 255.0;
                        self.set_pixel(&Vector3::new(x as f64, y as f64, z), &color);
                        self.depth_buf[index] = z;
                    }
                }
            }
        }
    }

    fn MSAA_sampling(&mut self, t: &Triangle) {
        let aabb = Aabb::new(t);

        let dx = [0.0, 0.5, 0.0, 0.5];
        let dy = [0.0, 0.0, 0.5, 0.5];

        for x in aabb.xmin as i32..=aabb.xmax as i32{
            for y in aabb.ymin as i32..=aabb.ymax as i32{
                let mut modified = false;
                let index = self.get_index(x as usize, y as usize);
                for sub_index in 0..4 {
                    let (x_sub, y_sub) = (x as f64 + dx[sub_index], y as f64 + dy[sub_index]);
                    if inside_triangle(x_sub + 0.25, y_sub + 0.25, &aabb.v) {
                        let z = compute_interpolate_depth(x_sub + 0.25, y_sub + 0.25, &aabb.v);
                        let index_MSAA = self.get_index_MSAA(x as usize, y as usize, sub_index);
    
                        // println!("{} {}", self.depth_sample.len(), index_MSAA);
                        if z < self.depth_sample[index_MSAA] {
                            // println!("This is pixel set, hello !!! at {} {} [COL : {}]", x, y, &(t.color[0]));
                            let color = t.color[0] * 255.0;
                            self.frame_sample[index_MSAA] = color;
                            self.depth_sample[index_MSAA] = z;
                            modified = true;
                        }
                    }
                }

                if modified {
                    self.frame_buf[index] = Vector3::zeros();
                    for sub_index in 0..4 {
                        self.frame_buf[index] += self.frame_sample[index * 4 + sub_index] / 4.0
                    }
                }
            }
        }
    }

    fn TAA_sampling(&mut self, t: &Triangle) {
        let aabb = Aabb::new(t);
        
        let mut rng = rand::thread_rng();

        // println!("This is Aabb, with this {} {} {} {} \n", aabb.xmin, aabb.xmax, aabb.ymin, aabb.ymax);
        for x in aabb.xmin as i32..=aabb.xmax as i32{
            for y in aabb.ymin as i32..=aabb.ymax as i32{
                let (dx, dy): (f64, f64) = (rng.gen(), rng.gen());
                let index = self.get_index(x as usize, y as usize);
                if inside_triangle(x as f64 + dx, y as f64 + dy, &aabb.v) {
                    let z = compute_interpolate_depth(x as f64 + dx, y as f64 + dy, &aabb.v);
                    if z < self.depth_buf[index] {
                        // println!("This is pixel set, hello !!! at {} {} [COL : {}]", x, y, &(t.color[0]));
                        let color = t.color[0] * 255.0;
                        self.set_pixel(&Vector3::new(x as f64, y as f64, z), &color);
                        self.depth_buf[index] = z;
                    }
                }
            }
        }
    }

    pub fn frame_buffer(&self) -> &Vec<Vector3<f64>> {
        &self.frame_buf
    }
}

fn to_vec4(v3: Vector3<f64>, w: Option<f64>) -> Vector4<f64> {
    Vector4::new(v3.x, v3.y, v3.z, w.unwrap_or(1.0))
}

fn inside_triangle(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> bool {
    /*  implement your code here  */
    let point = Vector2::new(x, y);
    let mut p: [Vector2<f64>; 3] = [Vector2::new(0.0, 0.0); 3];
    let mut d: [f64; 3] = [0.0; 3];
    for i in 0..3 {
        p[i] = Vector2::new(v[i][0], v[i][1]);
    }
    for i in 0..3 {
        d[i] = (p[(i + 1) % 3] - p[i]).perp(&(point - p[i]));
    } 

    (d[0] > 0.0 && d[1] > 0.0 && d[2] > 0.0) ||  (d[0] < 0.0 && d[1] < 0.0 && d[2] < 0.0)
}

fn compute_barycentric2d(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> (f64, f64, f64) {
    let c1 = (x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * y + v[1].x * v[2].y - v[2].x * v[1].y)
        / (v[0].x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * v[0].y + v[1].x * v[2].y - v[2].x * v[1].y);
    let c2 = (x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * y + v[2].x * v[0].y - v[0].x * v[2].y)
        / (v[1].x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * v[1].y + v[2].x * v[0].y - v[0].x * v[2].y);
    let c3 = (x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * y + v[0].x * v[1].y - v[1].x * v[0].y)
        / (v[2].x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * v[2].y + v[0].x * v[1].y - v[1].x * v[0].y);
    (c1, c2, c3)
}

fn compute_interpolate_depth(x: f64, y: f64, v : &[Vector3<f64>; 3]) -> f64 {
    let (c1, c2, c3) = compute_barycentric2d(x, y, v);
    let point = c1 * v[0] + c2 * v[1] + c3 * v[2];
    return point.z;
}
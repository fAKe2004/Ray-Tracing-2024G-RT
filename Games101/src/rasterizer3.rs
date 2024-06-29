use std::rc::Rc;

use clap::builder::TypedValueParser;
use nalgebra::{Matrix4, Transform, Vector2, Vector3, Vector4};
// use crate::get_NDC_to_screen;
use crate::shader::{FragmentShaderPayload, VertexShaderPayload};
use crate::texture::Texture;
use crate::triangle::Triangle;
use crate::triangle::Aabb;

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

#[derive(Default)]
pub struct Rasterizer {
    model: Matrix4<f64>,
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    texture: Option<Texture>,

    vert_shader: Option<fn(&VertexShaderPayload) -> Vector3<f64>>,
    fragment_shader: Option<fn(&FragmentShaderPayload) -> Vector3<f64>>,
    frame_buf: Vec<Vector3<f64>>,
    depth_buf: Vec<f64>,
    width: u64,
    height: u64,
}

#[derive(Clone, Copy)]
pub struct PosBufId(usize);

#[derive(Clone, Copy)]
pub struct IndBufId(usize);

#[derive(Clone, Copy)]
pub struct ColBufId(usize);

impl Rasterizer {
    pub fn new(w: u64, h: u64) -> Self {
        let mut r = Rasterizer::default();
        r.width = w;
        r.height = h;
        r.frame_buf.resize((w * h) as usize, Vector3::zeros());
        r.depth_buf.resize((w * h) as usize, 0.0);
        r.texture = None;
        r
    }

    fn get_index(height: u64, width: u64, x: usize, y: usize) -> usize {
        ((height - 1 - y as u64) * width + x as u64) as usize
    }

    fn set_pixel(height: u64, width: u64, frame_buf: &mut Vec<Vector3<f64>>, point: &Vector3<f64>, color: &Vector3<f64>) {
        let ind = (height as f64 - 1.0 - point.y) * width as f64 + point.x;
        frame_buf[ind as usize] = *color;
    }

    pub fn clear(&mut self, buff: Buffer) {
        match buff {
            Buffer::Color =>
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0)),
            Buffer::Depth =>
                self.depth_buf.fill(f64::MAX),
            Buffer::Both => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                self.depth_buf.fill(f64::MAX);
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

    pub fn set_texture(&mut self, tex: Texture) { 
        self.texture = Some(tex); 
    }

    pub fn set_vertex_shader(&mut self, vert_shader: fn(&VertexShaderPayload) -> Vector3<f64>) {
        self.vert_shader = Some(vert_shader);
    }
    
    pub fn set_fragment_shader(&mut self, frag_shader: fn(&FragmentShaderPayload) -> Vector3<f64>) {
        self.fragment_shader = Some(frag_shader);
    }

    pub fn draw(&mut self, triangles: &Vec<Triangle>) {
        // println!("> This is ORIGIN rasterizer <");
        // println!("View {} Model {} Project {}", self.view, self.model, self.projection);

        let mvp = self.projection * self.view * self.model;

        // 遍历每个小三角形
        for triangle in triangles { 
            self.rasterize_triangle(&triangle, mvp); 
        }
    }

    pub fn rasterize_triangle(&mut self, triangle: &Triangle, mvp: Matrix4<f64>) {
        // println!("??? {} {} ???", self.view, self.model);
        /*  Implement your code here  */
        let (t, view_space_pos) = Self::get_new_tri(triangle, self.view, self.model, mvp, (self.height, self.width));

        // let proj_inv: Matrix4<f64> = (get_NDC_to_screen(self.height as usize, self.width as usize, 0.1, 50.0) * self.projection).try_inverse().unwrap(); // map Screen space back to view space

        // let view_space_pos_arr: [Vector4<f64>; 3] = [
        //     to_vec4(view_space_pos[0], None), 
        //     to_vec4(view_space_pos[1], None), 
        //     to_vec4(view_space_pos[2], None)];
        let aabb = Aabb::new(&t);

        
        for x in aabb.xmin as i32..=aabb.xmax as i32{
            for y in aabb.ymin as i32..=aabb.ymax as i32{
                // println!("DEBUG1 >>> {} {} {} {}", aabb.xmin, aabb.xmax, aabb.ymin, aabb.ymax);
                let index = Self::get_index(self.height, self.width, x as usize, y as usize);
                let x_pos = x as f64 + 0.5;
                let y_pos = y as f64 + 0.5;
                if inside_triangle(x_pos, y_pos, &t.v) {
                    // println!("DEBUG2 >>> {} {}", x_pos, y_pos);
                    let (alpha, beta, gamma) = compute_barycentric2d(x_pos, y_pos, &(t.v));
                    let coe = (alpha, beta, gamma);
                    let z = Self::interpolate_depth(coe, &(aabb.v));
                    if z < self.depth_buf[index] {
                        // let mut view_space_coord = proj_inv * Vector4::new(x_pos, y_pos, z, 1.0); // revert to view space
                        // view_space_coord.x /= view_space_coord.w; // 注意修正到齐次
                        // view_space_coord.y /= view_space_coord.w;
                        // view_space_coord.z /= view_space_coord.w;
                        // view_space_coord.w = 1.0; 
                        
                        // let coe = compute_barycentric2d(view_space_coord.x, view_space_coord.y, &view_space_pos_arr); // 好像修正回 view_space 没啥帮助。
                        
                        let basic_color = Self::interpolate_color(coe, &t.color);
                        let view_pos = Self::interpolate_view_pos(coe, &view_space_pos);
                        let normal = Self::interpolate_normal(coe, &(t.normal));
                        // println!("Normal {} ", normal);
                        let tex_coord = Self::interpolate_tex_coord(coe, &(t.tex_coords));
  
                        let mut fspl = FragmentShaderPayload::new(&basic_color, &normal, &tex_coord, self.texture.as_ref().map(|texture| Rc::new(texture)));
                        fspl.view_pos = view_pos;

                        if let Some(fragment_shader) = self.fragment_shader {
                            let color = fragment_shader(&fspl);
                            
                            Self::set_pixel(self.height, self.width, &mut self.frame_buf, &Vector3::new(x as f64, y as f64, z), &color);
                            self.depth_buf[index] = z;
                        } else {
                            // println!("no fragment_shader provided");
                        }
                    }
                }
            }
        }
    }


    fn interpolate_vec3(a: f64, b: f64, c: f64, vert1: Vector3<f64>, vert2: Vector3<f64>, vert3: Vector3<f64>, weight: f64) -> Vector3<f64> {
        (a * vert1 + b * vert2 + c * vert3) / weight
    }
    fn interpolate_vec2(a: f64, b: f64, c: f64, vert1: Vector2<f64>, vert2: Vector2<f64>, vert3: Vector2<f64>, weight: f64) -> Vector2<f64> {
        (a * vert1 + b * vert2 + c * vert3) / weight
    }

    fn interpolate_depth((c1, c2, c3): (f64, f64, f64) , v: &[Vector3<f64>; 3]) -> f64 {
        let point = Self::interpolate_vec3(c1, c2, c3, v[0], v[1], v[2], 1.0);
        return point.z;
    }
    
    fn interpolate_view_pos((c1, c2, c3): (f64, f64, f64), view_space_pos: &Vec<Vector3<f64>>) -> Vector3<f64> {
        Self::interpolate_vec3(c1, c2, c3, view_space_pos[0], view_space_pos[1], view_space_pos[2], 1.0)
    }
    

    fn interpolate_color((c1, c2, c3): (f64, f64, f64), col: &[Vector3<f64>; 3]) -> Vector3<f64> {
        Self::interpolate_vec3(c1, c2, c3, col[0], col[1], col[2], 1.0)
    }
    
    fn interpolate_tex_coord((c1, c2, c3): (f64, f64, f64), coord: &[Vector2<f64>; 3]) -> Vector2<f64> {
        Self::interpolate_vec2(c1, c2, c3, coord[0], coord[1], coord[2], 1.0)
    }

        
    fn interpolate_normal((c1, c2, c3): (f64, f64, f64), normal: &[Vector3<f64>; 3]) -> Vector3<f64> {
        let result = Self::interpolate_vec3(c1, c2, c3, normal[0], normal[1], normal[2], 1.0);
        result.normalize()
    }

    fn get_new_tri(t: &Triangle, view: Matrix4<f64>, model: Matrix4<f64>, mvp: Matrix4<f64>,
                    (width, height): (u64, u64)) -> (Triangle, Vec<Vector3<f64>>) {
        let f1 = (50.0 - 0.1) / 2.0; // zfar和znear距离的一半
        let f2 = (50.0 + 0.1) / 2.0; // zfar和znear的中心z坐标
        let mut new_tri = (*t).clone();
        let mm: Vec<Vector4<f64>> = (0..3).map(|i| view * model * t.v[i]).collect();

        // println!("MM {} >< {} -> {}", view * model, t.v[0], mm[0]);

        let view_space_pos: Vec<Vector3<f64>> = mm.iter().map(|v| v.xyz()).collect();
        let mut v: Vec<Vector4<f64>> = (0..3).map(|i| mvp * t.v[i]).collect();

        // 换算齐次坐标
        for vec in v.iter_mut() {
            vec.x /= vec.w;
            vec.y /= vec.w;
            vec.z /= vec.w;
            vec.w = 1.0; //... ADDED 我说怎么我的逆矩阵炸了， 原来这里齐次坐标没修正完
        }
        let inv_trans = (view * model).try_inverse().unwrap().transpose();
        let n: Vec<Vector4<f64>> = (0..3).map(|i| inv_trans * to_vec4(t.normal[i], Some(0.0))).collect();

        for vert in v.iter_mut() {
            vert.x = 0.5 * width as f64 * (vert.x + 1.0);
            vert.y = 0.5 * height as f64 * (vert.y + 1.0);
            vert.z = vert.z * f1 + f2;
        }

        // println!("ACTUAL {} ", v[0]);
        for i in 0..3 {
            new_tri.set_vertex(i, v[i]);
        }
        for i in 0..3 {
            new_tri.set_normal(i, n[i].xyz());
        }

        new_tri.set_color(0, 148.0, 121.0, 92.0);
        new_tri.set_color(1, 148.0, 121.0, 92.0);
        new_tri.set_color(2, 148.0, 121.0, 92.0);

        // println!("view_space_pos {} {} {}", view_space_pos[0], view_space_pos[1], view_space_pos[2]);
        (new_tri, view_space_pos)
    }

    pub fn frame_buffer(&self) -> &Vec<Vector3<f64>> {
        &self.frame_buf
    }

}

fn to_vec4(v3: Vector3<f64>, w: Option<f64>) -> Vector4<f64> {
    Vector4::new(v3.x, v3.y, v3.z, w.unwrap_or(1.0))
}

fn inside_triangle(x: f64, y: f64, v: &[Vector4<f64>; 3]) -> bool {
    let v = [
        Vector3::new(v[0].x, v[0].y, 1.0),
        Vector3::new(v[1].x, v[1].y, 1.0),
        Vector3::new(v[2].x, v[2].y, 1.0), ];

    let f0 = v[1].cross(&v[0]);
    let f1 = v[2].cross(&v[1]);
    let f2 = v[0].cross(&v[2]);
    let p = Vector3::new(x, y, 1.0);
    if (p.dot(&f0) * f0.dot(&v[2]) > 0.0) &&
        (p.dot(&f1) * f1.dot(&v[0]) > 0.0) &&
        (p.dot(&f2) * f2.dot(&v[1]) > 0.0) {
        true
    } else {
        false
    }
}

fn compute_barycentric2d(x: f64, y: f64, v: &[Vector4<f64>; 3]) -> (f64, f64, f64) {
    let c1 = (x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * y + v[1].x * v[2].y - v[2].x * v[1].y) / (v[0].x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * v[0].y + v[1].x * v[2].y - v[2].x * v[1].y);
    let c2 = (x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * y + v[2].x * v[0].y - v[0].x * v[2].y) / (v[1].x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * v[1].y + v[2].x * v[0].y - v[0].x * v[2].y);
    let c3 = (x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * y + v[0].x * v[1].y - v[1].x * v[0].y) / (v[2].x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * v[2].y + v[0].x * v[1].y - v[1].x * v[0].y);
    (c1, c2, c3)
}


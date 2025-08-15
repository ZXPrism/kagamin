use std::{cmp, default};

use log::{error, info};
use minifb::{Window, WindowOptions};
use nalgebra::{Matrix4, Vector4};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

#[derive(Clone, Copy)]
pub struct VertexShaderState {
    // nuance: if under the same module, the `pub` keyword is redundant: any codes within the same module can access to fields freely, even they're private
    builtin_primitive_id: usize,
    builtin_vertex_id: usize,
    builtin_position: Vector4<f64>,
}

impl VertexShaderState {
    pub fn new(primitive_id: usize) -> VertexShaderState {
        VertexShaderState {
            builtin_primitive_id: primitive_id,
            builtin_vertex_id: 0, // undetermined until the VS stage of the current primitive
            builtin_position: Vector4::<f64>::default(),
        }
    }

    pub fn builtin_primitive_id(&self) -> usize {
        self.builtin_primitive_id
    }

    pub fn builtin_vertex_id(&self) -> usize {
        self.builtin_vertex_id
    }

    pub fn builtin_position(&self) -> &Vector4<f64> {
        &self.builtin_position
    }

    pub fn builtin_position_mut(&mut self) -> &mut Vector4<f64> {
        &mut self.builtin_position
    }

    pub fn _set_vertex_id(&mut self, vertex_id: usize) {
        self.builtin_vertex_id = vertex_id
    }
}

pub struct FragmentShaderState {}

// primitive is triangle by default
fn draw(n_vertices: usize, framebuffer: &mut Vec<u32>, vertices: &Vec<f64>) {
    if n_vertices % 3 != 0 {
        error!(
            "bad draw call. for triangle primitives, the number of vertices to draw must be divisible by 3!"
        );
        return;
    }

    // make viewport matrix
    #[rustfmt::skip]
    let viewport_matrix = Matrix4::new(
        WIDTH as f64 / 2.0, 0.0, 0.0, WIDTH as f64 / 2.0,
        0.0, -(HEIGHT as f64 / 2.0), 0.0, HEIGHT as f64 / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    for i in 0..n_vertices / 3 {
        // VS stage
        // VS can output arbitrary positions based on their needs, and they have access to current vertex id (0..n_vertices)
        // specifically, the primitive id: n-th primitive currently being rendered
        // the vertex id: local offset for the primitive, for triangles, ranges in 0, 1, 2
        let mut vs_state = [VertexShaderState::new(i); 3];

        // process each vertex
        for j in 0..3 {
            vs_state[j]._set_vertex_id(j);

            // call VS and get the output position
            // ...

            // viewport transformation
            *vs_state[j].builtin_position_mut() = viewport_matrix
                * Vector4::<f64>::new(
                    vertices[(i * 3 + j) * 2],
                    vertices[(i * 3 + j) * 2 + 1],
                    0.0,
                    1.0,
                );
        }

        // calculate AABB
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;
        for j in 0..3 {
            x_min = cmp::min(x_min, vs_state[j].builtin_position()[0].floor() as i32);
            x_max = cmp::max(x_max, vs_state[j].builtin_position()[0].floor() as i32);
            y_min = cmp::min(y_min, vs_state[j].builtin_position()[1].floor() as i32);
            y_max = cmp::max(y_max, vs_state[j].builtin_position()[1].floor() as i32);
        }

        let in_triangle = |x: f64, y: f64| {
            let mut check = 0u8;

            for i in 0..3 {
                let x1 = vs_state[i].builtin_position()[0];
                let y1 = vs_state[i].builtin_position()[1];
                let x2 = vs_state[(i + 1) % 3].builtin_position()[0];
                let y2 = vs_state[(i + 1) % 3].builtin_position()[1];
                if (x1 - x) * (y2 - y1) >= (y1 - y) * (x2 - x1) {
                    check |= 1 << i;
                }
            }

            check == 0 || check == 7
        };

        // rasterization
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                // check if current pixel is in the triangle
                if in_triangle(x as f64 + 0.5, y as f64 + 0.5) {
                    // FS stage
                    framebuffer[y as usize * WIDTH + x as usize] =
                        (((y as f64 / (HEIGHT - 1) as f64 * 255.0) as u32) << 16)
                            + (((x as f64 / (WIDTH - 1) as f64 * 255.0) as u32) << 8);
                }
            }
        }
    }
}

fn main() {
    env_logger::init();

    // prepare vertices data
    #[rustfmt::skip]
    let vertices = vec![
        0.0, 0.5,
        -0.5, -0.5,
        0.5, -0.5
    ];

    // create window and framebuffer
    let mut window =
        Window::new("test", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("Failed to create window: {}", e);
        });
    let mut framebuffer = vec![0u32; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        // clear back buffer
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                framebuffer[y * WIDTH + x] = 0x00ffffff;
            }
        }

        // draw call
        draw(3, &mut framebuffer, &vertices);

        window
            .update_with_buffer(&framebuffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

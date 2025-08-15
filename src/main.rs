use std::{array::from_fn, cmp, sync::Arc};

use log::error;
use minifb::{Window, WindowOptions};
use nalgebra::{Matrix4, Vector4};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

pub struct BindGroup {
    buffer: Arc<Vec<f64>>,
}

impl BindGroup {
    pub fn get_data_from_location(&self, location: usize) -> Arc<Vec<f64>> {
        self.buffer.clone()
    }
}

pub trait VertexShader {
    fn vertex(&self, vs_state: &mut VertexShaderState);
}

pub trait FragmentShader {
    fn fragment(&self, fs_state: &mut FragmentShaderState);
}

#[derive(Clone)]
pub struct VertexShaderState {
    // nuance: if under the same module, the `pub` keyword is redundant: any codes within the same module can access to fields freely, even they're private
    builtin_primitive_id: usize,
    builtin_vertex_id: usize,
    builtin_position: Vector4<f64>, // similar to gl_Position
    bind_group: Arc<BindGroup>,
}

impl VertexShaderState {
    pub fn new(
        primitive_id: usize,
        vertex_id: usize,
        bind_group: &Arc<BindGroup>,
    ) -> VertexShaderState {
        VertexShaderState {
            builtin_primitive_id: primitive_id,
            builtin_vertex_id: vertex_id,
            builtin_position: Vector4::<f64>::default(),
            bind_group: bind_group.clone(),
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

    pub fn location(&self, location: usize) -> Arc<Vec<f64>> {
        self.bind_group.get_data_from_location(location)
    }

    pub fn _set_vertex_id(&mut self, vertex_id: usize) {
        self.builtin_vertex_id = vertex_id
    }
}

pub struct FragmentShaderState {
    builtin_position: Vector4<f64>, // similar to gl_FragCoord
    builtin_color: Vector4<f64>,
}

impl FragmentShaderState {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> FragmentShaderState {
        FragmentShaderState {
            builtin_position: Vector4::<f64>::new(x, y, z, w),
            builtin_color: Vector4::<f64>::new(1.0, 0.0, 1.0, 1.0), // purple, can be useful for debug
        }
    }

    pub fn builtin_position(&self) -> &Vector4<f64> {
        &self.builtin_position
    }

    pub fn builtin_color(&self) -> &Vector4<f64> {
        &self.builtin_color
    }

    pub fn builtin_color_mut(&mut self) -> &mut Vector4<f64> {
        &mut self.builtin_color
    }
}

pub struct VSBlinnPhong {}

impl VertexShader for VSBlinnPhong {
    fn vertex(&self, vs_state: &mut VertexShaderState) {
        let buffer = vs_state.location(0);
        let primitive_id = vs_state.builtin_primitive_id();
        let vertex_id = vs_state.builtin_vertex_id();
        let global_vertex_offset = primitive_id * 3 + vertex_id;

        *vs_state.builtin_position_mut() = Vector4::<f64>::new(
            buffer[global_vertex_offset * 2],
            buffer[global_vertex_offset * 2 + 1],
            0.0,
            1.0,
        );
    }
}

pub struct FSBlinnPhong {}

impl FragmentShader for FSBlinnPhong {
    fn fragment(&self, fs_state: &mut FragmentShaderState) {
        let position = fs_state.builtin_position();

        *fs_state.builtin_color_mut() = Vector4::new(
            position.y as f64 / HEIGHT as f64,
            position.x as f64 / WIDTH as f64,
            0.0,
            1.0,
        );
    }
}

fn vec4_to_0rgb(vec: &Vector4<f64>) -> u32 {
    (((vec.x * 255.0).round() as u32) << 16)
        + (((vec.y * 255.0).round() as u32) << 8)
        + ((vec.z * 255.0).round() as u32)
}

// primitive is triangle by default
fn draw(n_vertices: usize, framebuffer: &mut Vec<u32>, vertices: &Arc<Vec<f64>>) {
    if n_vertices % 3 != 0 {
        error!(
            "bad draw call. for triangle primitives, the number of vertices to draw must be divisible by 3!"
        );
        return;
    }

    // make viewport matrix (test only, should not be placed here)
    #[rustfmt::skip]
    let viewport_matrix = Matrix4::new(
        WIDTH as f64 / 2.0, 0.0, 0.0, WIDTH as f64 / 2.0,
        0.0, -(HEIGHT as f64 / 2.0), 0.0, HEIGHT as f64 / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    // make the shader (test only, should not be placed here)
    let vertex_shader = VSBlinnPhong {};
    let fragment_shader = FSBlinnPhong {};

    // make the bind group (test only, should not be placed here)
    let bind_group = Arc::<BindGroup>::new(BindGroup {
        buffer: vertices.clone(),
    });

    for i in 0..n_vertices / 3 {
        // VS stage
        // VS can output arbitrary positions based on their needs, and they have access to current vertex id (0..n_vertices)
        // specifically, the primitive id: n-th primitive currently being rendered
        // the vertex id: local offset for the primitive, for triangles, ranges in 0, 1, 2
        let mut vs_state: [_; 3] = from_fn(|x| VertexShaderState::new(i, x, &bind_group));

        // process each vertex
        for j in 0..3 {
            // call VS and get the output position
            vertex_shader.vertex(&mut vs_state[j]);
        }

        // TODO: clip, may emit new vertices

        // convert to NDC (but preseve w for correction?)
        for j in 0..3 {
            let w = vs_state[j].builtin_position().w;
            *vs_state[j].builtin_position_mut() /= w; // nuance: we can't write *vs_state[j].builtin_position_mut() /= *vs_state[j].builtin_position().w
        }

        for j in 0..3 {
            // viewport transformation
            *vs_state[j].builtin_position_mut() = viewport_matrix * vs_state[j].builtin_position(); // nuance: but we can use the whole! ..
            // ..the key is '/='
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
                let px_x = x as f64 + 0.5;
                let px_y = y as f64 + 0.5;

                // check if current pixel is in the triangle
                if in_triangle(px_x, px_y) {
                    // FS stage
                    let mut fs_state = FragmentShaderState::new(px_x, px_y, 0.0, 1.0); // TODO: should be interp and corrected
                    fragment_shader.fragment(&mut fs_state);

                    framebuffer[y as usize * WIDTH + x as usize] =
                        vec4_to_0rgb(&fs_state.builtin_color());
                }
            }
        }
    }
}

fn main() {
    env_logger::init();

    // prepare vertices data
    #[rustfmt::skip]
    let vertices = Arc::<Vec<f64>> ::new(vec![
        0.0, 0.5,
        -0.5, -0.5,
        0.5, -0.5
    ]);

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

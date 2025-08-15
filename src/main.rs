use std::cmp;

use log::info;
use minifb::{Window, WindowOptions};
use nalgebra::{Matrix4, Vector4};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    env_logger::init();

    // prepare vertices data
    #[rustfmt::skip]
    let vertices = [
        0.0, 0.5,
        -0.5, -0.5,
        0.5, -0.5
    ];

    // create window and framebuffer
    let mut window =
        Window::new("test", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("Failed to create window: {}", e);
        });
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // make viewport matrix
    #[rustfmt::skip]
    let viewport_matrix = Matrix4::new(
        WIDTH as f64 / 2.0, 0.0, 0.0, WIDTH as f64 / 2.0,
        0.0, -(HEIGHT as f64 / 2.0), 0.0, HEIGHT as f64 / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        // clear back buffer
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                buffer[y * WIDTH + x] = 0x00ffffff;
            }
        }

        // viewport transformation
        let mut position = [Vector4::<f64>::default(); 3];
        for i in 0..3 {
            position[i] = viewport_matrix
                * Vector4::<f64>::new(vertices[i * 2], vertices[i * 2 + 1], 0.0, 1.0);
        }

        // calculate AABB
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;
        for i in 0..3 {
            x_min = cmp::min(x_min, position[i][0].floor() as i32);
            x_max = cmp::max(x_max, position[i][0].floor() as i32);
            y_min = cmp::min(y_min, position[i][1].floor() as i32);
            y_max = cmp::max(y_max, position[i][1].floor() as i32);
        }

        let in_triangle = |x: f64, y: f64| {
            let mut check = 0u8;

            for i in 0..3 {
                let x1 = position[i][0];
                let y1 = position[i][1];
                let x2 = position[(i + 1) % 3][0];
                let y2 = position[(i + 1) % 3][1];
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
                    buffer[y as usize * WIDTH + x as usize] =
                        (((y as f64 / (HEIGHT - 1) as f64 * 255.0) as u32) << 16)
                            + (((x as f64 / (WIDTH - 1) as f64 * 255.0) as u32) << 8);
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

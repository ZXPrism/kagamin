use crate::bind_group::*;

use std::sync::Arc;

use nalgebra::Vector4;

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

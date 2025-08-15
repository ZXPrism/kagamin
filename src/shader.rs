use crate::shader_state::*;

pub trait VertexShader {
    fn vertex(&self, vs_state: &mut VertexShaderState);
}

pub trait FragmentShader {
    fn fragment(&self, fs_state: &mut FragmentShaderState);
}

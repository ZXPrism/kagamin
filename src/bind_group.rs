use std::sync::Arc;

pub struct BindGroup {
    pub buffer: Arc<Vec<f64>>,
}

pub struct BindGroupCreateInfo {
    entries: ;
}

pub struct BindGroupEntry{
    binding_point : ;
}

impl BindGroup {
    pub fn new(create_info: &BindGroupCreateInfo) -> BindGroup {

    }

    pub fn get_data_from_location(&self, location: usize) -> Arc<Vec<f64>> {
        self.buffer.clone()
    }
}

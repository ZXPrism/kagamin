use std::sync::Arc;

pub struct BindGroup {
    pub buffer: Arc<Vec<f64>>,
}

impl BindGroup {
    pub fn get_data_from_location(&self, location: usize) -> Arc<Vec<f64>> {
        self.buffer.clone()
    }
}

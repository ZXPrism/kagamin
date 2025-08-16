use std::{default, sync::Arc};

use log::error;

pub struct BindGroup {
    buffers: Vec<Arc<Vec<f64>>>,
}

pub struct BindGroupCreateInfo {
    pub entries: Vec<BindGroupEntry>,
}

pub struct BindGroupEntry {
    pub binding_point: usize,
    pub buffer: Arc<Vec<f64>>,
}

impl BindGroup {
    pub fn new(create_info: &BindGroupCreateInfo) -> BindGroup {
        let n = create_info.entries.len();

        // check if binding_points are out of range or duplicate
        let mut flag = vec![false; n];
        for i in 0..n {
            let binding_point = create_info.entries[i].binding_point;

            if binding_point >= n {
                error!("bad bind group create info: binding point value out of range!");
            } else if flag[binding_point] {
                error!("bad bind group create info: binding point value duplicates!");
            }

            flag[binding_point] = true;
        }

        let mut res = BindGroup {
            buffers: vec![Arc::<Vec<f64>>::default(); n],
        };

        // put each entry to its right position
        for i in 0..n {
            res.buffers[create_info.entries[i].binding_point] =
                create_info.entries[i].buffer.clone();
        }

        res
    }

    pub fn get_data_from_location(&self, location: usize) -> Arc<Vec<f64>> {
        self.buffers[location].clone()
    }
}

impl BindGroupCreateInfo {
    pub fn new() -> BindGroupCreateInfo {
        BindGroupCreateInfo {
            entries: Vec::<BindGroupEntry>::new(),
        }
    }
}

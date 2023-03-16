use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Entry {
    pub name: String,
    pub tag_index: usize,
    pub color_index: usize,
}

impl Entry {
    pub fn new(name: String, tag_index: usize, color_index: usize) -> Self {
        Self {
            name,
            tag_index,
            color_index,
        }
    }
}


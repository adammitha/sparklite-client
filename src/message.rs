use crate::dataset::FilterPredicate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Message {
    CreateDataset { id: String, data: Vec<u8> },
    LoadDataset { id: String },
    Transformation(Transformation),
}

impl Message {
    pub fn load_data(id: &str) -> Self {
        Self::LoadDataset {
            id: String::from(id),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Transformation {
    Map,
    Filter(FilterPredicate),
    Reduce,
}

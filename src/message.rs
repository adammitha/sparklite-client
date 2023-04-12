use crate::dataset::FilterPredicate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Message {
    CreateDataset { id: String, data: Vec<u8> },
    LoadDataset { id: String },
    Transformation(Transformation),
}

#[derive(Serialize, Deserialize)]
pub enum Transformation {
    Map,
    Filter(FilterPredicate),
    Reduce,
}

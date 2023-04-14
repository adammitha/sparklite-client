use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Message {
    CreateDataset { id: String, data: Vec<u8> },
    LoadDataset { id: String },
    Transformation(Transformation),
    GetDataset { id: String },
}

#[derive(Serialize, Deserialize)]
pub enum Transformation {
    Map,
    Filter(FilterPredicate),
    Reduce,
}

#[derive(Serialize, Deserialize)]
pub enum FilterPredicate {
    Eq(String),
    Lt(String),
    Lte(String),
    Gt(String),
    Gte(String),
    Ne(String),
}

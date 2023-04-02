use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Message<'a> {
    CreateDataset { id: &'a str, data: &'a [u8] },
    LoadDataset { id: &'a str },
    Transformation(Transformation),
}

#[derive(Serialize, Deserialize)]
pub enum Transformation {
    Map,
    Filter,
    Reduce,
}

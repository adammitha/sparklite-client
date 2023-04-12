use crate::dataset::FilterPredicate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Message<'a, T>
where
    T: PartialEq + PartialOrd,
{
    CreateDataset { id: &'a str, data: &'a [u8] },
    LoadDataset { id: &'a str },
    Transformation(Transformation<T>),
}

#[derive(Serialize, Deserialize)]
pub enum Transformation<T>
where
    T: PartialEq + PartialOrd,
{
    Map,
    Filter(FilterPredicate<T>),
    Reduce,
}

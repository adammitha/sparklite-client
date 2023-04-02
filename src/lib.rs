#![allow(dead_code, unused_variables)]
mod http;

use std::fs::File;
use std::net::SocketAddr;
pub use http::RetryingHttpClient;

pub struct Client {
    server: SocketAddr,
}

impl Client {
    pub fn new(server: SocketAddr) -> Self {
        Self {
            server,
        }
    }

    pub fn create_data(dataset_id: &str, datatset: &File) -> Result<(), Error> {
        todo!()
    }

    pub fn load_data(dataset_id: &str) -> Result<(), Error> {
        todo!()
    }

    pub fn map(dataset_id: &str) -> Result<(), Error> {
        todo!()
    }

    pub fn reduce(dataset_id: &str) -> Result<(), Error> {
        todo!()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
}

enum Message {
    CreateDataset,
    LoadDataset,
    Transformation(Transformation),
}

enum Transformation {
    Map,
    Filter,
    Reduce,
}

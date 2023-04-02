#![allow(dead_code, unused_variables)]
mod http;
mod message;

pub use http::RetryingHttpClient;
use std::fs::File;
use std::net::SocketAddr;

pub struct Client {
    server: SocketAddr,
}

impl Client {
    pub fn new(server: SocketAddr) -> Self {
        Self { server }
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
pub enum Error {}

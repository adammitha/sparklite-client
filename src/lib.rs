#![allow(dead_code, unused_variables)]
mod http;
mod message;

pub use http::RetryingHttpClient;
use hyper::client::connect::Connect;
use hyper::Uri;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub struct Client<C>
where
    C: Connect + Clone + Send + Sync + 'static,
{
    inner: RetryingHttpClient<C>,
    server: Uri,
}

impl<C> Client<C>
where
    C: Connect + Clone + Send + Sync,
{
    pub fn new(server: Uri, connector: C) -> Self {
        Self {
            inner: RetryingHttpClient::new(connector),
            server,
        }
    }

    pub async fn create_data(dataset_id: &str, dataset: File) -> Result<(), Error> {
        let stream = FramedRead::new(dataset, BytesCodec::new());
        Ok(())
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

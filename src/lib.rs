#![allow(dead_code, unused_variables)]
mod http;
mod message;

pub use http::RetryingHttpClient;
use hyper::client::connect::Connect;
use hyper::{Body, Response, Uri};
use tokio::fs::File;

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

    pub async fn create_data(&self, dataset_id: &str, dataset: &mut File) -> Result<Response<Body>, Error> {
        self.inner.post(&self.server, dataset).await.map_err(|err| { Error::HttpError(err) })
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
    #[error("Error with http client")]
    HttpError(http::Error),
}

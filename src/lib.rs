#![allow(dead_code, unused_variables)]
mod dataset;
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

    pub async fn create_data(
        &self,
        dataset_id: &str,
        dataset: &mut File,
    ) -> Result<Response<Body>, Error> {
        self.inner
            .post(&self.server, dataset)
            .await
            .map_err(|err| Error::HttpError(err))
    }

    pub async fn load_data(&self, dataset_id: &str) -> Result<Response<Body>, Error> {
        let mut parts = self.server.clone().into_parts();
        parts.path_and_query = Some(
            format!("/load_dataset?dataset_id={}", dataset_id)
                .try_into()
                .unwrap(),
        );
        self.inner.get(&Uri::from_parts(parts).unwrap()).await.map_err(|err| Error::HttpError(err))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error with http client")]
    HttpError(http::Error),
}

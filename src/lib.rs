#![allow(dead_code, unused_variables)]
mod dataset;
mod http;
mod message;

use crate::message::Message;
pub use dataset::Dataset;
pub use dataset::FilterPredicate;
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
        // TODO: needs to be revised if the server implements this function
        self.inner
            .post(&self.server, dataset)
            .await
            .map_err(|err| Error::HttpError(err))
    }

    pub async fn load_data(&self, dataset_id: &str) -> Result<String, Error> {
        let msg = Message::LoadDataset {
            id: dataset_id.into(),
        };
        let mut response = self
            .inner
            .get(
                &self.build_uri("/load_data"),
                Some(serde_json::to_string(&msg).unwrap()),
            )
            .await
            .map_err(|err| Error::HttpError(err))?;
        Ok(std::str::from_utf8(
            hyper::body::to_bytes(response.body_mut())
                .await
                .unwrap()
                .as_ref(),
        )
        .unwrap()
        .into())
    }

    pub async fn filter(
        &self,
        dataset_id: &str,
        predicate: FilterPredicate,
    ) -> Result<String, Error> {
        let msg = Message::Transformation(message::Transformation::Filter(predicate));
        let mut response = self
            .inner
            .get(
                &self.build_uri("/filter"),
                Some(serde_json::to_string(&msg).unwrap()),
            )
            .await
            .map_err(|err| Error::HttpError(err))?;
        Ok(std::str::from_utf8(
            hyper::body::to_bytes(response.body_mut())
                .await
                .unwrap()
                .as_ref(),
        )
        .unwrap()
        .into())
    }

    pub async fn get_dataset(&self, dataset_id: &str) -> Result<Response<Body>, Error> {
        let msg = Message::GetDataset {
            id: dataset_id.into(),
        };
        self.inner
            .get(
                &self.build_uri("/get_data"),
                Some(serde_json::to_string(&msg).unwrap()),
            )
            .await
            .map_err(|err| Error::HttpError(err))
    }

    fn build_uri(&self, path: &str) -> Uri {
        let mut parts = self.server.clone().into_parts();
        parts.path_and_query = Some(path.try_into().unwrap());
        Uri::from_parts(parts).unwrap()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error with http client")]
    HttpError(http::Error),
}

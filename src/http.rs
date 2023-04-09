use hyper::client::{connect::Connect, Client};
use hyper::{Body, Request, Response, Uri};
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::time::{sleep, timeout};
use tracing::debug;

pub struct RetryingHttpClient<C>
where
    C: Connect + Clone + Send + Sync + 'static,
{
    inner: Client<C>,
    num_retries: u8,
    timeout: Duration,
}

impl<C> RetryingHttpClient<C>
where
    C: Connect + Clone + Send + Sync,
{
    pub fn new(connector: C) -> Self {
        Self {
            inner: Client::builder().build(connector),
            num_retries: 4,
            timeout: Duration::from_millis(200),
        }
    }

    pub async fn get(&self, uri: &Uri) -> Result<Response<Body>, Error> {
        for i in 0..self.num_retries {
            let request = Request::get(uri).body(Body::empty()).unwrap();
            let timeout_duration = self.timeout * 2u32.pow(i as _);
            debug!("Sending {:?}, iteration: {}, timeout: {:?}", request, i, timeout_duration);
            let req_future = self.inner.request(request);
            match timeout(timeout_duration, req_future).await {
                Ok(result) => match result {
                    Ok(res) => return Ok(res),
                    Err(err) => return Err(Error::Hyper(err)),
                },
                Err(_) => (),
            }
        }
        Err(Error::Timeout)
    }

    pub async fn post(&self, uri: &Uri, body: &mut tokio::fs::File) -> Result<Response<Body>, Error> {
        let mut bytes = Vec::new();
        body.read_to_end(&mut bytes).await.unwrap();
        for i in 0..self.num_retries {
            let request = Request::post(uri).body(bytes.clone().into()).unwrap();
            let timeout_duration = self.timeout * 2u32.pow(i as _);
            debug!("Sending {:?}, iteration: {}, timeout: {:?}", request, i, timeout_duration);
            let req_future = self.inner.request(request);
            match timeout(self.timeout, req_future).await {
                Ok(result) => match result {
                    Ok(res) => return Ok(res),
                    Err(err) => return Err(Error::Hyper(err)),
                },
                Err(_) => (),
            }
        }
        Err(Error::Timeout)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Non-retryable http error")]
    Hyper(hyper::Error),
    #[error("Exhausted retries")]
    Timeout,
}

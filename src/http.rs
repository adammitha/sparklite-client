use hyper::client::{Client, connect::Connect};
use hyper::{Body, Request, Response};
use std::time::Duration;
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
            timeout: Duration::from_millis(100),
        }
    }

    pub async fn get(&self, url: &str) -> Result<Response<Body>, Error> {
        let uri: hyper::Uri = url.parse().unwrap();
        for i in 0..self.num_retries {
            let request = Request::builder()
                .method("GET")
                .uri(&uri)
                .body(Body::empty())
                .unwrap();
            debug!("Sending {:?}, iteration: {}", request, i);
            let req_future = self.inner.request(request);
            match timeout(self.timeout, req_future).await {
                Ok(result) => match result {
                    Ok(res) => return Ok(res),
                    Err(err) => return Err(Error::Hyper(err)),
                },
                Err(_) => (),
            }
            let sleep_duration = Duration::from_secs(2u64.pow(i as _));
            debug!("Sleeping for {:?} secs", sleep_duration);
            sleep(sleep_duration).await;
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

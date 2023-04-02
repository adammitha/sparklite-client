use std::time::Duration;
use hyper::client::Client;
use hyper::client::connect::Connect;
use hyper::Request;
use hyper::Body;
use hyper::Response;
use tokio::time::timeout;

pub struct RetryingHttpClient<C>
where
    C: Connect + Clone + Send + Sync
{
    inner: Client<C>,
    num_retries: u8,
    timeout: Duration,
}

impl<C> RetryingHttpClient<C>
where
    C: Connect + Clone + Send + Sync + 'static
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
            let req_future = self.inner.request(request);
            match timeout(self.timeout, req_future).await {
                Ok(result) => match result {
                    Ok(res) => return Ok(res),
                    Err(err) => return Err(Error::Hyper(err)),
                }
                Err(_) => continue
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

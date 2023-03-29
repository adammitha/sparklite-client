use reqwest::Response;
use std::time::Duration;

pub struct RetryingHttpClient {
    inner: reqwest::Client,
    num_retries: u8,
    timeout: Duration,
}

impl RetryingHttpClient {
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
            num_retries: 4,
            timeout: Duration::from_secs(1),
        }
    }

    pub async fn get(&self, url: &str) -> Result<Response, Error> {
        for i in 0..self.num_retries {
            let result = self.inner.get(url).timeout(self.timeout).send().await;
            match result {
                Ok(res) => return Ok(res),
                Err(err) => {
                    if !err.is_timeout() {
                        return Err(Error::Reqwest(err));
                    }
                }
            }
            tokio::time::sleep(Duration::from_secs(2u64.pow(i as _))).await;
        }
        Err(Error::Timeout)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Non-retryable http error")]
    Reqwest(reqwest::Error),
    #[error("Exhausted retries")]
    Timeout,
}

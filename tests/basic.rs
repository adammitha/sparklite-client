#![allow(dead_code)]
use sparklite_client::RetryingHttpClient;
use tracing::info;
mod connector;

#[tokio::test]
async fn basic_test() {
    tracing_subscriber::fmt::init();
    let client = RetryingHttpClient::new(hyper::client::HttpConnector::new());
    info!("{:?}", client.get("http://localhost:8000/".parse().unwrap()).await.unwrap());
}

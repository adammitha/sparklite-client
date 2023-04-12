#![allow(dead_code)]
use sparklite_client::RetryingHttpClient;
mod connector;

#[tokio::test]
async fn get() {
    let client = RetryingHttpClient::new(hyper::client::HttpConnector::new());
    println!(
        "{:?}",
        client
            .get(&"http://localhost:8000/".parse().unwrap(), None)
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn post() {
    let client = RetryingHttpClient::new(hyper::client::HttpConnector::new());
    println!(
        "{:?}",
        client
            .post(
                &"http://localhost:8000/".parse().unwrap(),
                &mut tokio::fs::File::open("Cargo.toml").await.unwrap()
            )
            .await
            .unwrap()
    );
}

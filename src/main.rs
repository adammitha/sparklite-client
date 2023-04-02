#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let client = sparklite_client::RetryingHttpClient::new(hyper::client::HttpConnector::new());
    println!("{:?}", client.get("http://localhost:8000/").await.unwrap());
}

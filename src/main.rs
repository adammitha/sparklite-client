#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let client = sparklite_client::Client::new(
        "http://localhost:8000".parse().unwrap(),
        hyper::client::HttpConnector::new(),
    );
    println!("{:?}", client.load_data("123").await.unwrap());
}

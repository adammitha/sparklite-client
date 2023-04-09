#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    println!("Hello, world!");
    let client = sparklite_client::RetryingHttpClient::new(hyper::client::HttpConnector::new());
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

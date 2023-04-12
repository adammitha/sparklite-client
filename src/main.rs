#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let client = sparklite_client::Client::new(
        "http://localhost:8000".parse().unwrap(),
        hyper::client::HttpConnector::new(),
    );
    let res = client.load_data("test_data").await.unwrap();
    println!("{:?}", res);
    // println!("{:?}", client.filter("test_data", sparklite_client::FilterPredicate::Eq("abc".into())).await.unwrap());
    // println!("{:?}", client.load_data("123").await.unwrap());
}

use algolia_monitoring_rs::AlgoliaMonitoring;

#[tokio::main]
async fn main() {
    let monitoring = AlgoliaMonitoring::new(Some("api_key".to_owned()), Some("app_id".to_owned()));
    let result = monitoring.get_status(None).await;
    println!("{:?}", result);
    println!("Hello, world!");
}

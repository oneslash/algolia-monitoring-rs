# Algolia Monitoring API Client

The Algolia Monitoring API Client is a library that enables developers to interact with the Algolia Monitoring REST API programmatically. This client makes it easy to fetch data and monitor the status, incidents, inventory, latency, and reachability of Algolia's services. By providing a simple interface to interact with the API, the Algolia Monitoring API Client abstracts away the complexity of making HTTP requests, handling authentication, and parsing JSON responses.

The repository is not affiliated with Algolia company and all the trademarks to Algolia name is theirs.

Key features of the Algolia Monitoring API Client:

1. Simple Interface: The client offers an easy-to-use interface to interact with the API, with intuitive methods to access various endpoints such as `get_status`, `get_incidents`, `get_inventory`, `get_latency`, and `get_reachability`.
2. Asynchronous Support: The API client supports asynchronous requests, allowing developers to make non-blocking calls to the Algolia Monitoring API, improving the performance and responsiveness of applications.
3. Error Handling: The client provides clear error messages and handles common HTTP errors, allowing developers to focus on building their applications rather than debugging API requests.



Usage of the Algolia Monitoring API Client typically involves the following steps:

1. Install the client library in your project.
2. (Optional) Create an instance of the client using your Algolia API key and application ID.
3. Call the appropriate methods, such as `get_status`, `get_incidents`, or `get_latency`, to fetch data from the Algolia Monitoring API.
4. Process the returned data as needed by your application.

By using the Algolia Monitoring API Client, developers can easily monitor their Algolia services, ensuring optimal performance, and quickly addressing any incidents or issues that arise.

Simple example

```rust
use algolia_monitoring_rs::AlgoliaMonitoring;

#[tokio::main]
async fn main() {
    let monitoring = AlgoliaMonitoring::new("api_key".to_owned(), "app_id".to_owned());
    let result = monitoring.get_status(None).await;
    println!("{:?}", result);
    println!("Hello, world!");
}
```

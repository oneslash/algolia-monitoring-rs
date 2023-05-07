use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct DataPoint {
    t: u64,
    v: u32,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    reason: String,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    status: HashMap<String, String>
}

#[derive(Debug, Deserialize)]
pub struct Incident {
    t: i64,
    v: IncidentDetails,
}

#[derive(Debug, Deserialize)]
pub struct IncidentDetails {
    title: String,
    body: String,
    status: String,
}

#[derive(Debug, Deserialize)]
pub struct Incidents {
    incidents: HashMap<String, Vec<Incident>>,
}

#[derive(Debug, Deserialize)]
pub struct InventoryItem {
    name: String,
    region: String,
    is_replica: bool,
    cluster: String,
}

#[derive(Debug, Deserialize)]
pub struct Inventory {
    inventory: Vec<InventoryItem>,
}

#[derive(Debug, Deserialize)]
pub struct MetricsGroup {
    #[serde(default)]
    latency: Option<HashMap<String, Vec<DataPoint>>>,
    #[serde(default)]
    indexing: Option<HashMap<String, Vec<DataPoint>>>,
    #[serde(default)]
    avg_build_time: Option<HashMap<String, Vec<DataPoint>>>,
    #[serde(default)]
    ssd_usage: Option<HashMap<String, Vec<DataPoint>>>,
    #[serde(default)]
    ram_search_usage: Option<HashMap<String, Vec<DataPoint>>>,
    #[serde(default)]
    ram_indexing_usage: Option<HashMap<String, Vec<DataPoint>>>,
    #[serde(default)]
    cpu_usage: Option<HashMap<String, Vec<DataPoint>>>,
}

#[derive(Debug, Deserialize)]
pub struct Metrics {
    metrics: MetricsGroup,
}

pub struct AlgoliaMonitoring {
    api_key: Option<String>,
    app_id: Option<String>,
}

impl AlgoliaMonitoring {
    pub fn new(api_key: Option<String>, app_id: Option<String>) -> Self {
        AlgoliaMonitoring { api_key, app_id }
    }

    /// Get the status of the Algolia servers
    /// `servers` is an optional list of servers to get the status of, if None, all servers are returned
    pub async fn get_status(&self, servers: Option<Vec<String>>) -> Result<Status, Error> {
        let servers = match servers.map(|s| s.join(",")) {
            Some(s) => format!("/{}", s),
            None => "".to_owned(),
        };

        let path = format!("status{}", servers);
        self.fetch_data::<Status>(path.as_str()).await
    }

    /// Get the incidents of the Algolia servers
    /// `servers` is an optional list of servers to get the incidents of, if None, all servers are returned
    pub async fn get_incidents(&self, servers: Option<Vec<String>>) -> Result<Incidents, Error> {
        let servers = match servers.map(|s| s.join(",")) {
            Some(s) => format!("/{}", s),
            None => "".to_owned(),
        };
        let path = format!("incidents/{}", servers);
        self.fetch_data::<Incidents>(path.as_str()).await
    }

    /// Get the inventory of the Algolia servers
    pub async fn get_inventory(&self) -> Result<Inventory, Error> {
        self.fetch_data::<Inventory>("inventory").await
    }

    /// Get the latency of the Algolia servers
    /// `servers` is a list of servers to get the latency of
    pub async fn get_latency(&self, servers: Vec<String>) -> Result<Metrics, Error> {
        let servers = servers.join(",");
        let path = format!("latency/{}", servers);
        self.fetch_data::<Metrics>(path.as_str()).await
    }

    /// This method gets the reachability for the servers passed in the URL
    /// `servers` is a list of servers to get the reachability of
    pub async fn get_reachability(&self, servers: Vec<String>) -> Result<Metrics, Error> {
        let servers = servers.join(",");
        let path = format!("reachability/{}/probes", servers);
        self.fetch_data::<Metrics>(path.as_str()).await
    }

    /// This method gets a metric over a period of time
    /// `metric` is the metric to get
    /// - `avg_build_time`: Average build time of the indices in seconds
    /// - `ssd_usage`: proportion of SSD vs RAM usage in % (0% means no SSD utilization, 32 GB storage used on 64 GB RAM system is 50%)
    /// - `ram_search_usage`: RAM usage for the search in MB
    /// - `ram_indexing_usage`: RAM usage for the indexing in MB
    /// - `cpu_usage`: proportion of CPU idleness in % (0% means the CPU isn’t idle)
    /// - `*`: All of the above
    /// `period` is the period of time to get the metric over
    /// - `minute`: 1 minute ago, 1 point per 10 seconds (10 points)
    /// - `hour`: 1 hour ago, 1 point per 1 minute (60 points)
    /// - `day`: 1 day ago, 1 point per 10 minutes (144 points)
    /// - `week`: 1 week ago, 1 point per 1 hour (168 points)
    /// - `month`: 1 month ago, 1 point per 1 day (30 points)
    pub async fn get_infrastructure_metrics(&self, metric: String, period: String) -> Result<Metrics, Error> {
        let path = format!("infrastructure/{}/period/{}", metric, period);
        self.fetch_data::<Metrics>(path.as_str()).await
    }

    fn get_http_client(&self) -> Result<Client, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        if !self.api_key.is_some() && !self.app_id.is_some() {
            headers.insert("X-Algolia-API-Key", self.api_key.as_ref().unwrap().parse().unwrap());
            headers.insert("X-Algolia-Application-Id", self.app_id.as_ref().unwrap().parse().unwrap());
        }
        Client::builder().default_headers(headers).build()
    }

    fn get_endpoint(&self) -> String {
        "https://status.algolia.com/1".to_owned()
    }

    async fn fetch_data<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let client = self.get_http_client().map_err(|e| Error {
            reason: e.to_string(),
        })?;

        let url = format!("{}/{}", self.get_endpoint(), path);
        let response = client.get(&url).send().await.map_err(|e| Error {
            reason: e.to_string(),
        })?;

        response.json::<T>().await.map_err(|e| Error {
            reason: e.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_algolia_monitoring() -> AlgoliaMonitoring {
        AlgoliaMonitoring::new(Some("your_api_key".to_string()), Some("your_app_id".to_string()))
    }

    #[test]
    fn test_new() {
        let algolia_monitoring = create_algolia_monitoring();
        assert_eq!(algolia_monitoring.api_key, "your_api_key");
        assert_eq!(algolia_monitoring.app_id, "your_app_id");
    }
}

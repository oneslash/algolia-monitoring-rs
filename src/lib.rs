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
}

#[derive(Debug, Deserialize)]
pub struct Metrics {
    metrics: MetricsGroup,
}

pub struct AlgoliaMonitoring {
    api_key: String,
    app_id: String,
}

impl AlgoliaMonitoring {
    pub fn new(api_key: String, app_id: String) -> Self {
        AlgoliaMonitoring { api_key, app_id }
    }

    /// Get the status of the Algolia servers
    pub async fn get_status(&self, servers: Option<Vec<String>>) -> Result<Status, Error> {
        let servers = match servers.map(|s| s.join(",")) {
            Some(s) => format!("/{}", s),
            None => "".to_owned(),
        };

        let path = format!("status{}", servers);
        self.fetch_data::<Status>(path.as_str()).await
    }

    pub async fn get_incidents(&self, servers: Option<Vec<String>>) -> Result<Incidents, Error> {
        let servers = match servers.map(|s| s.join(",")) {
            Some(s) => format!("/{}", s),
            None => "".to_owned(),
        };
        let path = format!("incidents/{}", servers);
        self.fetch_data::<Incidents>(path.as_str()).await
    }

    pub async fn get_inventory(&self) -> Result<Inventory, Error> {
        self.fetch_data::<Inventory>("inventory").await
    }

    pub async fn get_latency(&self, servers: Vec<String>) -> Result<Metrics, Error> {
        let servers = servers.join(",");
        let path = format!("latency/{}", servers);
        self.fetch_data::<Metrics>(path.as_str()).await
    }

    pub async fn get_reachability(&self, servers: Vec<String>) -> Result<Metrics, Error> {
        let servers = servers.join(",");
        let path = format!("reachability/{}/probes", servers);
        self.fetch_data::<Metrics>(path.as_str()).await
    }

    fn get_http_client(&self) -> Result<Client, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-Algolia-API-Key", self.api_key.parse().unwrap());
        headers.insert("X-Algolia-Application-Id", self.app_id.parse().unwrap());
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
        AlgoliaMonitoring::new("your_api_key".to_string(), "your_app_id".to_string())
    }

    #[test]
    fn test_new() {
        let algolia_monitoring = create_algolia_monitoring();
        assert_eq!(algolia_monitoring.api_key, "your_api_key");
        assert_eq!(algolia_monitoring.app_id, "your_app_id");
    }
}

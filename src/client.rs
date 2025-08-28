use reqwest::{Client, Url};
use crate::error::WalrusError;

pub struct WalrusClient {
    aggregator_url: Url,
    publisher_url: Url,
    http_client: Client,
}

impl WalrusClient {
    pub fn new(aggregator_url: &str, publisher_url: &str) -> Result<Self, WalrusError> {
        let aggregator_url = Url::parse(aggregator_url)
            .map_err(|e| WalrusError::InvalidUrl(format!("Invalid aggregator URL: {}", e)))?;
        let publisher_url = Url::parse(publisher_url)
            .map_err(|e| WalrusError::InvalidUrl(format!("Invalid publisher URL: {}", e)))?;

        Ok(Self {
            aggregator_url,
            publisher_url,
            http_client: Client::new(),
        })
    }

    pub fn aggregator_url(&self) -> &Url {
        &self.aggregator_url
    }

    pub fn publisher_url(&self) -> &Url {
        &self.publisher_url
    }

    pub fn http_client(&self) -> &Client {
        &self.http_client
    }
}
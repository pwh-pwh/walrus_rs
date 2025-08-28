use crate::client::WalrusClient;
use crate::error::WalrusError;
use crate::models::BlobStoreResult;

pub struct BlobClient<'a> {
    client: &'a WalrusClient,
}

impl<'a> BlobClient<'a> {
    pub fn new(client: &'a WalrusClient) -> Self {
        Self { client }
    }

    pub async fn store_blob(
        &self,
        data: impl Into<reqwest::Body>,
        epochs: Option<u64>,
        deletable: Option<bool>,
        permanent: Option<bool>,
        send_object_to: Option<&str>,
    ) -> Result<BlobStoreResult, WalrusError> {
        let mut url = self.client.publisher_url().join("v1/blobs")
            .map_err(|e| WalrusError::InvalidUrl(format!("Failed to build URL: {}", e)))?;

        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(e) = epochs {
                query_pairs.append_pair("epochs", &e.to_string());
            }
            if let Some(d) = deletable {
                query_pairs.append_pair("deletable", &d.to_string());
            }
            if let Some(p) = permanent {
                query_pairs.append_pair("permanent", &p.to_string());
            }
            if let Some(s) = send_object_to {
                query_pairs.append_pair("send_object_to", s);
            }
        }

        let response = self.client.http_client().put(url)
            .body(data)
            .send()
            .await?
            .error_for_status()?;

        let result: BlobStoreResult = response.json().await
            .map_err(|e| WalrusError::ParseError(format!("Failed to parse BlobStoreResult: {}", e)))?;

        Ok(result)
    }

    pub async fn read_blob_by_id(&self, blob_id: &str) -> Result<Vec<u8>, WalrusError> {
        let url = self.client.aggregator_url().join(&format!("v1/blobs/{}", blob_id))
            .map_err(|e| WalrusError::InvalidUrl(format!("Failed to build URL: {}", e)))?;

        let response = self.client.http_client().get(url)
            .send()
            .await?
            .error_for_status()?;

        let bytes = response.bytes().await
            .map_err(|e| WalrusError::ParseError(format!("Failed to read blob bytes: {}", e)))?;

        Ok(bytes.to_vec())
    }

    pub async fn read_blob_by_object_id(&self, object_id: &str) -> Result<Vec<u8>, WalrusError> {
        let url = self.client.aggregator_url().join(&format!("v1/blobs/by-object-id/{}", object_id))
            .map_err(|e| WalrusError::InvalidUrl(format!("Failed to build URL: {}", e)))?;

        let response = self.client.http_client().get(url)
            .send()
            .await?
            .error_for_status()?;

        let bytes = response.bytes().await
            .map_err(|e| WalrusError::ParseError(format!("Failed to read blob bytes: {}", e)))?;

        Ok(bytes.to_vec())
    }
}
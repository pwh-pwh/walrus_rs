use reqwest::{multipart::{Form, Part}};
use serde_json::to_string;
use crate::client::WalrusClient;
use crate::error::WalrusError;
use crate::models::{QuiltStoreResponse, QuiltMetadata};

pub struct QuiltClient<'a> {
    client: &'a WalrusClient,
}

impl<'a> QuiltClient<'a> {
    pub fn new(client: &'a WalrusClient) -> Self {
        Self { client }
    }

    pub async fn store_quilt(
        &self,
        files: Vec<(&str, Vec<u8>)>,
        metadata: Option<Vec<QuiltMetadata>>,
        epochs: Option<u64>,
        deletable: Option<bool>,
        permanent: Option<bool>,
        send_object_to: Option<&str>,
    ) -> Result<QuiltStoreResponse, WalrusError> {
        let mut url = self.client.publisher_url().join("v1/quilts")
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

        let mut form = Form::new();
        for (identifier, data) in files {
            form = form.part(identifier.to_string(), Part::bytes(data));
        }

        if let Some(meta) = metadata {
            let metadata_json = to_string(&meta)
                .map_err(|e| WalrusError::ParseError(format!("Failed to serialize metadata: {}", e)))?;
            form = form.part("_metadata", Part::text(metadata_json));
        }

        let response = self.client.http_client().put(url)
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        let result: QuiltStoreResponse = response.json().await
            .map_err(|e| WalrusError::ParseError(format!("Failed to parse QuiltStoreResponse: {}", e)))?;

        Ok(result)
    }

    pub async fn read_quilt_blob_by_patch_id(&self, quilt_patch_id: &str) -> Result<Vec<u8>, WalrusError> {
        let url = self.client.aggregator_url().join(&format!("v1/blobs/by-quilt-patch-id/{}", quilt_patch_id))
            .map_err(|e| WalrusError::InvalidUrl(format!("Failed to build URL: {}", e)))?;

        let response = self.client.http_client().get(url)
            .send()
            .await?
            .error_for_status()?;

        let bytes = response.bytes().await
            .map_err(|e| WalrusError::ParseError(format!("Failed to read quilt blob bytes: {}", e)))?;

        Ok(bytes.to_vec())
    }

    pub async fn read_quilt_blob_by_quilt_id_and_identifier(
        &self,
        quilt_id: &str,
        identifier: &str,
    ) -> Result<Vec<u8>, WalrusError> {
        let url = self.client.aggregator_url().join(&format!("v1/blobs/by-quilt-id/{}/{}", quilt_id, identifier))
            .map_err(|e| WalrusError::InvalidUrl(format!("Failed to build URL: {}", e)))?;

        let response = self.client.http_client().get(url)
            .send()
            .await?
            .error_for_status()?;

        let bytes = response.bytes().await
            .map_err(|e| WalrusError::ParseError(format!("Failed to read quilt blob bytes: {}", e)))?;

        Ok(bytes.to_vec())
    }
}
use reqwest::{
    Client, Url,
    multipart::{Form, Part},
};
use serde_json::to_string;

use crate::error::WalrusError;
use crate::models::{BlobMetadata, BlobStoreResult, QuiltMetadata, QuiltStoreResponse};

pub struct WalrusClient {
    aggregator_url: Url,
    publisher_url: Url,
    http_client: Client,
}

impl WalrusClient {
    pub fn new(aggregator_url: &str, publisher_url: &str) -> Result<Self, WalrusError> {
        let aggregator_url = Url::parse(aggregator_url)
            .map_err(|e| WalrusError::InvalidUrl(format!("Invalid aggregator URL: {e}")))?;
        let publisher_url = Url::parse(publisher_url)
            .map_err(|e| WalrusError::InvalidUrl(format!("Invalid publisher URL: {e}")))?;

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

    pub async fn store_blob(
        &self,
        data: impl Into<reqwest::Body>,
        epochs: Option<u64>,
        deletable: Option<bool>,
        permanent: Option<bool>,
        send_object_to: Option<&str>,
    ) -> Result<BlobStoreResult, WalrusError> {
        let mut url = self
            .publisher_url()
            .join("v1/blobs")
            .map_err(|e| WalrusError::InvalidUrl(format!("Failed to build URL: {e}")))?;

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

        let response = self
            .http_client()
            .put(url)
            .body(data)
            .send()
            .await?
            .error_for_status()?;

        let result: BlobStoreResult = response.json().await.map_err(|e| {
            WalrusError::ParseError(format!("Failed to parse BlobStoreResult: {e}"))
        })?;

        Ok(result)
    }

    pub async fn read_blob_by_id(&self, blob_id: &str) -> Result<Vec<u8>, WalrusError> {
        let url = self
            .aggregator_url()
            .join(&format!("v1/blobs/{blob_id}"))
            .map_err(|e| WalrusError::InvalidUrl(format!("Failed to build URL: {e}")))?;

        let response = self
            .http_client()
            .get(url)
            .send()
            .await?
            .error_for_status()?;

        let bytes = response
            .bytes()
            .await
            .map_err(|e| WalrusError::ParseError(format!("Failed to read blob bytes: {e}")))?;

        Ok(bytes.to_vec())
    }

    pub async fn read_blob_by_object_id(&self, object_id: &str) -> Result<Vec<u8>, WalrusError> {
        let url = self
            .aggregator_url()
            .join(&format!("v1/blobs/by-object-id/{object_id}"))
            .map_err(|e| WalrusError::InvalidUrl(format!("Failed to build URL: {e}")))?;

        let response = self
            .http_client()
            .get(url)
            .send()
            .await?
            .error_for_status()?;

        let bytes = response
            .bytes()
            .await
            .map_err(|e| WalrusError::ParseError(format!("Failed to read blob bytes: {e}")))?;

        Ok(bytes.to_vec())
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
        let mut url = self
            .publisher_url()
            .join("v1/quilts")
            .map_err(|e| WalrusError::InvalidUrl(format!("Failed to build URL: {e}")))?;

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
            let metadata_json = to_string(&meta).map_err(|e| {
                WalrusError::ParseError(format!("Failed to serialize metadata: {e}"))
            })?;
            form = form.part("_metadata", Part::text(metadata_json));
        }

        let response = self
            .http_client()
            .put(url)
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        let result: QuiltStoreResponse = response.json().await.map_err(|e| {
            WalrusError::ParseError(format!("Failed to parse QuiltStoreResponse: {e}"))
        })?;

        Ok(result)
    }

    pub async fn read_quilt_blob_by_patch_id(
        &self,
        quilt_patch_id: &str,
    ) -> Result<Vec<u8>, WalrusError> {
        let url = self
            .aggregator_url()
            .join(&format!("v1/blobs/by-quilt-patch-id/{quilt_patch_id}"))
            .map_err(|e| WalrusError::InvalidUrl(format!("Failed to build URL: {e}")))?;

        let response = self
            .http_client()
            .get(url)
            .send()
            .await?
            .error_for_status()?;

        let bytes = response.bytes().await.map_err(|e| {
            WalrusError::ParseError(format!("Failed to read quilt blob bytes: {e}"))
        })?;

        Ok(bytes.to_vec())
    }

    pub async fn read_quilt_blob_by_quilt_id_and_identifier(
        &self,
        quilt_id: &str,
        identifier: &str,
    ) -> Result<Vec<u8>, WalrusError> {
        let url = self
            .aggregator_url()
            .join(&format!("v1/blobs/by-quilt-id/{quilt_id}/{identifier}"))
            .map_err(|e| WalrusError::InvalidUrl(format!("Failed to build URL: {e}")))?;

        let response = self
            .http_client()
            .get(url)
            .send()
            .await?
            .error_for_status()?;

        let bytes = response.bytes().await.map_err(|e| {
            WalrusError::ParseError(format!("Failed to read quilt blob bytes: {e}"))
        })?;

        Ok(bytes.to_vec())
    }

    pub async fn get_blob_metadata(&self, blob_id: &str) -> Result<BlobMetadata, WalrusError> {
        let url = self
            .aggregator_url()
            .join(&format!("v1/blobs/{blob_id}"))
            .map_err(|e| WalrusError::InvalidUrl(format!("Failed to build URL: {e}")))?;

        let response = self
            .http_client()
            .head(url)
            .send()
            .await?
            .error_for_status()?;

        // Helper function to extract header value
        fn get_header_value(
            headers: &reqwest::header::HeaderMap,
            key: &str,
        ) -> Result<String, WalrusError> {
            headers
                .get(key)
                .ok_or_else(|| WalrusError::ParseError(format!("Missing header: {key}")))?
                .to_str()
                .map_err(|e| WalrusError::ParseError(format!("Failed to parse header {key}: {e}")))
                .map(|s| s.to_owned())
        }

        let content_length = get_header_value(response.headers(), "content-length")?
            .parse::<u64>()
            .map_err(|e| WalrusError::ParseError(format!("Failed to parse content-length: {e}")))?;
        let content_type = get_header_value(response.headers(), "content-type")?;
        let etag = get_header_value(response.headers(), "etag")?;

        Ok(BlobMetadata {
            content_length,
            content_type,
            etag,
        })
    }
}

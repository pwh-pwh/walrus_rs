use reqwest::{
    Client, Url,
    multipart::{Form, Part},
};
use serde_json::to_string;

use crate::error::WalrusError;
use crate::models::{BlobMetadata, BlobStoreResult, QuiltMetadata, QuiltStoreResponse};

/// `WalrusClient` is an asynchronous Walrus API client.
/// It encapsulates all logic for interacting with the Walrus Aggregator and Publisher services.
pub struct WalrusClient {
    aggregator_url: Url,
    publisher_url: Url,
    http_client: Client,
}

impl WalrusClient {
    /// Creates a new `WalrusClient` instance.
    ///
    /// # Arguments
    /// - `aggregator_url`: The URL string for the Walrus Aggregator service.
    /// - `publisher_url`: The URL string for the Walrus Publisher service.
    ///
    /// # Returns
    /// - `Ok(WalrusClient)`: Successfully created a client instance.
    /// - `Err(WalrusError::InvalidUrl)`: If the provided URL is invalid.
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

    /// Returns the URL of the Aggregator service.
    pub fn aggregator_url(&self) -> &Url {
        &self.aggregator_url
    }

    /// Returns the URL of the Publisher service.
    pub fn publisher_url(&self) -> &Url {
        &self.publisher_url
    }

    /// Returns a reference to the internal `reqwest::Client` instance.
    pub fn http_client(&self) -> &Client {
        &self.http_client
    }

    /// Stores a Blob to the Walrus Publisher service.
    ///
    /// # Arguments
    /// - `data`: The Blob data to store, can be any type convertible to `reqwest::Body`.
    /// - `epochs`: Optional, the number of epochs for the Blob's lifecycle.
    /// - `deletable`: Optional, indicates if the Blob is deletable.
    /// - `permanent`: Optional, indicates if the Blob is permanently stored.
    /// - `send_object_to`: Optional, specifies where to send the object.
    ///
    /// # Returns
    /// - `Ok(BlobStoreResult)`: Successfully stored the Blob and returned the result.
    /// - `Err(WalrusError)`: If storing failed, possibly due to invalid URL, network error, or response parsing failure.
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

    /// Reads Blob data by Blob ID from the Walrus Aggregator service.
    ///
    /// # Arguments
    /// - `blob_id`: The unique identifier of the Blob.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)`: Successfully read the Blob data.
    /// - `Err(WalrusError)`: If reading failed, possibly due to invalid URL, network error, or data parsing failure.
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

    /// Reads Blob data by object ID from the Walrus Aggregator service.
    ///
    /// # Arguments
    /// - `object_id`: The unique identifier of the object.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)`: Successfully read the Blob data.
    /// - `Err(WalrusError)`: If reading failed, possibly due to invalid URL, network error, or data parsing failure.
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

    /// Stores a Quilt (multiple files) to the Walrus Publisher service.
    ///
    /// # Arguments
    /// - `files`: A vector of tuples containing the filename and file content.
    /// - `metadata`: Optional, metadata for the Quilt.
    /// - `epochs`: Optional, the number of epochs for the Quilt's lifecycle.
    /// - `deletable`: Optional, indicates if the Quilt is deletable.
    /// - `permanent`: Optional, indicates if the Quilt is permanently stored.
    /// - `send_object_to`: Optional, specifies where to send the object.
    ///
    /// # Returns
    /// - `Ok(QuiltStoreResponse)`: Successfully stored the Quilt and returned the result.
    /// - `Err(WalrusError)`: If storing failed, possibly due to invalid URL, network error, metadata serialization failure, or response parsing failure.
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

    /// Reads Quilt Blob data by Quilt Patch ID from the Walrus Aggregator service.
    ///
    /// # Arguments
    /// - `quilt_patch_id`: The unique identifier of the Quilt Patch.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)`: Successfully read the Quilt Blob data.
    /// - `Err(WalrusError)`: If reading failed, possibly due to invalid URL, network error, or data parsing failure.
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

    /// Reads Quilt Blob data by Quilt ID and identifier from the Walrus Aggregator service.
    ///
    /// # Arguments
    /// - `quilt_id`: The unique identifier of the Quilt.
    /// - `identifier`: The identifier of the Blob within the Quilt.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)`: Successfully read the Quilt Blob data.
    /// - `Err(WalrusError)`: If reading failed, possibly due to invalid URL, network error, or data parsing failure.
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

    /// Retrieves metadata for a Blob by its Blob ID from the Walrus Aggregator service.
    ///
    /// # Arguments
    /// - `blob_id`: The unique identifier of the Blob.
    ///
    /// # Returns
    /// - `Ok(BlobMetadata)`: Successfully retrieved the Blob metadata.
    /// - `Err(WalrusError)`: If retrieval failed, possibly due to invalid URL, network error, or response header parsing failure.
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

        /// Helper function to extract a header value from HTTP response headers.
        ///
        /// # Arguments
        /// - `headers`: The HTTP response headers.
        /// - `key`: The key of the header to extract.
        ///
        /// # Returns
        /// - `Ok(String)`: The successfully extracted header value.
        /// - `Err(WalrusError::ParseError)`: If the header is missing or its value cannot be parsed.
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

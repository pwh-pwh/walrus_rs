use crate::client::WalrusClient;
use crate::error::WalrusError;
use crate::models::{BlobMetadata, BlobStoreResult, QuiltMetadata, QuiltStoreResponse};
use tokio::runtime::Runtime;

/// `BlockingWalrusClient` is a blocking Walrus API client.
/// It provides a synchronous interface by internally using an asynchronous `WalrusClient` and blocking the current thread.
pub struct BlockingWalrusClient {
    async_client: WalrusClient,
    runtime: Runtime,
}

impl BlockingWalrusClient {
    /// Creates a new `BlockingWalrusClient` instance.
    ///
    /// # Arguments
    /// - `aggregator_url`: The URL string for the Walrus Aggregator service.
    /// - `publisher_url`: The URL string for the Walrus Publisher service.
    ///
    /// # Returns
    /// - `Ok(BlockingWalrusClient)`: Successfully created a client instance.
    /// - `Err(WalrusError)`: If the provided URL is invalid or the Tokio runtime creation fails.
    pub fn new(aggregator_url: &str, publisher_url: &str) -> Result<Self, WalrusError> {
        let async_client = WalrusClient::new(aggregator_url, publisher_url)?;
        let runtime = Runtime::new().map_err(|e| WalrusError::Other(e.to_string()))?;
        Ok(Self {
            async_client,
            runtime,
        })
    }

    /// Stores a Blob to the Walrus Publisher service (blocking version).
    ///
    /// This method blocks the current thread until the Blob storage operation is complete.
    ///
    /// # Arguments
    /// - `data`: The Blob data to store.
    /// - `epochs`: Optional, the number of epochs for the Blob's lifecycle.
    /// - `deletable`: Optional, indicates if the Blob is deletable.
    /// - `permanent`: Optional, indicates if the Blob is permanently stored.
    /// - `send_object_to`: Optional, specifies where to send the object.
    ///
    /// # Returns
    /// - `Ok(BlobStoreResult)`: Successfully stored the Blob and returned the result.
    /// - `Err(WalrusError)`: If storing failed.
    pub fn store_blob(
        &self,
        data: impl Into<reqwest::Body> + Send,
        epochs: Option<u64>,
        deletable: Option<bool>,
        permanent: Option<bool>,
        send_object_to: Option<&str>,
    ) -> Result<BlobStoreResult, WalrusError> {
        self.runtime.block_on(self.async_client.store_blob(
            data,
            epochs,
            deletable,
            permanent,
            send_object_to,
        ))
    }

    /// Reads Blob data by Blob ID from the Walrus Aggregator service (blocking version).
    ///
    /// This method blocks the current thread until the Blob read operation is complete.
    ///
    /// # Arguments
    /// - `blob_id`: The unique identifier of the Blob.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)`: Successfully read the Blob data.
    /// - `Err(WalrusError)`: If reading failed.
    pub fn read_blob_by_id(&self, blob_id: &str) -> Result<Vec<u8>, WalrusError> {
        self.runtime
            .block_on(self.async_client.read_blob_by_id(blob_id))
    }

    /// Reads Blob data by object ID from the Walrus Aggregator service (blocking version).
    ///
    /// This method blocks the current thread until the Blob read operation is complete.
    ///
    /// # Arguments
    /// - `object_id`: The unique identifier of the object.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)`: Successfully read the Blob data.
    /// - `Err(WalrusError)`: If reading failed.
    pub fn read_blob_by_object_id(&self, object_id: &str) -> Result<Vec<u8>, WalrusError> {
        self.runtime
            .block_on(self.async_client.read_blob_by_object_id(object_id))
    }

    /// Stores a Quilt (multiple files) to the Walrus Publisher service (blocking version).
    ///
    /// This method blocks the current thread until the Quilt storage operation is complete.
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
    /// - `Err(WalrusError)`: If storing failed.
    pub fn store_quilt(
        &self,
        files: Vec<(&str, Vec<u8>)>,
        metadata: Option<Vec<QuiltMetadata>>,
        epochs: Option<u64>,
        deletable: Option<bool>,
        permanent: Option<bool>,
        send_object_to: Option<&str>,
    ) -> Result<QuiltStoreResponse, WalrusError> {
        self.runtime.block_on(self.async_client.store_quilt(
            files,
            metadata,
            epochs,
            deletable,
            permanent,
            send_object_to,
        ))
    }

    /// Reads Quilt Blob data by Quilt Patch ID from the Walrus Aggregator service (blocking version).
    ///
    /// This method blocks the current thread until the Quilt Blob read operation is complete.
    ///
    /// # Arguments
    /// - `quilt_patch_id`: The unique identifier of the Quilt Patch.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)`: Successfully read the Quilt Blob data.
    /// - `Err(WalrusError)`: If reading failed.
    pub fn read_quilt_blob_by_patch_id(
        &self,
        quilt_patch_id: &str,
    ) -> Result<Vec<u8>, WalrusError> {
        self.runtime.block_on(
            self.async_client
                .read_quilt_blob_by_patch_id(quilt_patch_id),
        )
    }

    /// Reads Quilt Blob data by Quilt ID and identifier from the Walrus Aggregator service (blocking version).
    ///
    /// This method blocks the current thread until the Quilt Blob read operation is complete.
    ///
    /// # Arguments
    /// - `quilt_id`: The unique identifier of the Quilt.
    /// - `identifier`: The identifier of the Blob within the Quilt.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)`: Successfully read the Quilt Blob data.
    /// - `Err(WalrusError)`: If reading failed.
    pub fn read_quilt_blob_by_quilt_id_and_identifier(
        &self,
        quilt_id: &str,
        identifier: &str,
    ) -> Result<Vec<u8>, WalrusError> {
        self.runtime.block_on(
            self.async_client
                .read_quilt_blob_by_quilt_id_and_identifier(quilt_id, identifier),
        )
    }

    /// Retrieves metadata for a Blob by its Blob ID from the Walrus Aggregator service (blocking version).
    ///
    /// This method blocks the current thread until the Blob metadata retrieval operation is complete.
    ///
    /// # Arguments
    /// - `blob_id`: The unique identifier of the Blob.
    ///
    /// # Returns
    /// - `Ok(BlobMetadata)`: Successfully retrieved the Blob metadata.
    /// - `Err(WalrusError)`: If retrieval failed.
    pub fn get_blob_metadata(&self, blob_id: &str) -> Result<BlobMetadata, WalrusError> {
        self.runtime
            .block_on(self.async_client.get_blob_metadata(blob_id))
    }
}

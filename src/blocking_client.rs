use crate::client::WalrusClient;
use crate::error::WalrusError;
use crate::models::{BlobMetadata, BlobStoreResult, QuiltMetadata, QuiltStoreResponse};
use tokio::runtime::Runtime;

pub struct BlockingWalrusClient {
    async_client: WalrusClient,
    runtime: Runtime,
}

impl BlockingWalrusClient {
    pub fn new(aggregator_url: &str, publisher_url: &str) -> Result<Self, WalrusError> {
        let async_client = WalrusClient::new(aggregator_url, publisher_url)?;
        let runtime = Runtime::new().map_err(|e| WalrusError::Other(e.to_string()))?;
        Ok(Self {
            async_client,
            runtime,
        })
    }

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

    pub fn read_blob_by_id(&self, blob_id: &str) -> Result<Vec<u8>, WalrusError> {
        self.runtime
            .block_on(self.async_client.read_blob_by_id(blob_id))
    }

    pub fn read_blob_by_object_id(&self, object_id: &str) -> Result<Vec<u8>, WalrusError> {
        self.runtime
            .block_on(self.async_client.read_blob_by_object_id(object_id))
    }

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

    pub fn read_quilt_blob_by_patch_id(
        &self,
        quilt_patch_id: &str,
    ) -> Result<Vec<u8>, WalrusError> {
        self.runtime.block_on(
            self.async_client
                .read_quilt_blob_by_patch_id(quilt_patch_id),
        )
    }

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

    pub fn get_blob_metadata(&self, blob_id: &str) -> Result<BlobMetadata, WalrusError> {
        self.runtime
            .block_on(self.async_client.get_blob_metadata(blob_id))
    }
}

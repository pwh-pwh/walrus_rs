use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Blob object in the Walrus API.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobObject {
    /// The unique ID of the Blob.
    pub id: String,
    /// The epoch at which the Blob was registered.
    pub registered_epoch: u64,
    /// The ID of the Blob.
    pub blob_id: String,
    /// The size of the Blob.
    pub size: u64,
    /// The encoding type of the Blob.
    pub encoding_type: String,
    /// The epoch at which the Blob was certified (if applicable).
    pub certified_epoch: Option<u64>,
    /// Storage information for the Blob.
    pub storage: StorageInfo,
    /// Indicates if the Blob is deletable.
    pub deletable: bool,
}

/// Represents storage information for a Blob.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageInfo {
    /// The storage ID.
    pub id: String,
    /// The starting epoch of the storage.
    pub start_epoch: u64,
    /// The ending epoch of the storage.
    pub end_epoch: u64,
    /// The size of the storage.
    pub storage_size: u64,
}

/// Represents a resource operation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceOperation {
    /// Details for a register from scratch operation (if applicable).
    pub register_from_scratch: Option<RegisterFromScratch>,
}

/// Represents details for a register from scratch operation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterFromScratch {
    /// The encoded length.
    pub encoded_length: u64,
    /// The number of epochs ahead.
    pub epochs_ahead: u64,
}

/// Represents information about a newly created Blob.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewlyCreated {
    /// The Blob object.
    pub blob_object: BlobObject,
    /// The resource operation.
    pub resource_operation: ResourceOperation,
    /// The cost.
    pub cost: u64,
}

/// Represents an event in the Walrus API.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// The transaction digest.
    pub tx_digest: String,
    /// The event sequence.
    pub event_seq: String,
}

/// Represents information about an already certified Blob.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlreadyCertified {
    /// The ID of the Blob.
    pub blob_id: String,
    /// The associated event.
    pub event: Event,
    /// The ending epoch.
    pub end_epoch: u64,
}

/// Represents the result of a Blob storage operation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobStoreResult {
    /// Information about a newly created Blob (if applicable).
    pub newly_created: Option<NewlyCreated>,
    /// Information about an already certified Blob (if applicable).
    pub already_certified: Option<AlreadyCertified>,
}

/// Represents a stored Quilt Blob.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredQuiltBlob {
    /// The identifier.
    pub identifier: String,
    /// The Quilt Patch ID.
    pub quilt_patch_id: String,
}

/// Represents the response from a Quilt storage operation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuiltStoreResponse {
    /// The Blob store result.
    pub blob_store_result: BlobStoreResult,
    /// A list of stored Quilt Blobs.
    pub stored_quilt_blobs: Vec<StoredQuiltBlob>,
}

/// Represents metadata for a Quilt.
#[derive(Debug, Serialize, Deserialize)]
pub struct QuiltMetadata {
    /// The identifier.
    pub identifier: String,
    /// Tags associated with the Quilt.
    pub tags: HashMap<String, String>,
}

/// Represents metadata for a Blob.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlobMetadata {
    /// The content length.
    pub content_length: u64,
    /// The content type.
    pub content_type: String,
    /// The ETag.
    pub etag: String,
}

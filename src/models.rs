use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobObject {
    pub id: String,
    pub registered_epoch: u64,
    pub blob_id: String,
    pub size: u64,
    pub encoding_type: String,
    pub certified_epoch: Option<u64>,
    pub storage: StorageInfo,
    pub deletable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageInfo {
    pub id: String,
    pub start_epoch: u64,
    pub end_epoch: u64,
    pub storage_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceOperation {
    pub register_from_scratch: Option<RegisterFromScratch>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterFromScratch {
    pub encoded_length: u64,
    pub epochs_ahead: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewlyCreated {
    pub blob_object: BlobObject,
    pub resource_operation: ResourceOperation,
    pub cost: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub tx_digest: String,
    pub event_seq: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlreadyCertified {
    pub blob_id: String,
    pub event: Event,
    pub end_epoch: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobStoreResult {
    pub newly_created: Option<NewlyCreated>,
    pub already_certified: Option<AlreadyCertified>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredQuiltBlob {
    pub identifier: String,
    pub quilt_patch_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuiltStoreResponse {
    pub blob_store_result: BlobStoreResult,
    pub stored_quilt_blobs: Vec<StoredQuiltBlob>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuiltMetadata {
    pub identifier: String,
    pub tags: HashMap<String, String>,
}
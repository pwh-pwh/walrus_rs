# walrus_rs

`walrus_rs` is a Rust client library for interacting with the Walrus API. It provides convenient interfaces for storing and reading blob and quilt data.

## Installation

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
walrus_rs = "0.1.4" # Replace with the latest version
reqwest = { version = "0.12", features = ["json", "multipart"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
url = "2"
thiserror = "1"
async-trait = "0.1"
```

## Async Example

Here's a simple example demonstrating how to use the async `WalrusClient` to store and read blob and quilt data:

```rust
use walrus_rs::{WalrusClient, WalrusError};

#[tokio::main]
async fn main() -> Result<(), WalrusError> {
    let aggregator_url = std::env::var("AGGREGATOR")
        .unwrap_or_else(|_| "https://aggregator.testnet.walrus.atalma.io".to_string());
    let publisher_url = std::env::var("PUBLISHER")
        .unwrap_or_else(|_| "https://publisher.walrus-01.tududes.com".to_string());

    let client = WalrusClient::new(&aggregator_url, &publisher_url)?;

    // Example: Store a blob
    println!("Storing a blob...");
    let data = "some string from Rust SDK".as_bytes().to_vec();
    let store_result = client.store_blob(data, Some(1), None, None, None).await?;
    println!("Blob store result: {:?}", store_result);

    if let Some(newly_created) = store_result.newly_created {
        let blob_id = newly_created.blob_object.blob_id;
        println!("Newly created blob ID: {}", blob_id);

        // Example: Read a blob by ID
        println!("Reading blob by ID: {}", blob_id);
        let read_data = client.read_blob_by_id(&blob_id).await?;
        println!("Read blob data: {}", String::from_utf8_lossy(&read_data));
    }

    // Example: Store a quilt
    println!("\nStoring a quilt...");
    let file1_data = "content of file 1".as_bytes().to_vec();
    let file2_data = "content of file 2".as_bytes().to_vec();
    let files = vec![
        ("file1.txt", file1_data),
        ("file2.txt", file2_data),
    ];
    let quilt_store_result = client.store_quilt(files, None, Some(1), None, None, None).await?;
    println!("Quilt store result: {:?}", quilt_store_result);

    if let Some(newly_created) = quilt_store_result.blob_store_result.newly_created {
        let quilt_id = newly_created.blob_object.blob_id;
        println!("Newly created quilt ID: {}", quilt_id);

        if let Some(stored_quilt_blob) = quilt_store_result.stored_quilt_blobs.get(0) {
            let quilt_patch_id = &stored_quilt_blob.quilt_patch_id;
            println!("First quilt patch ID: {}", quilt_patch_id);

            // Example: Read a quilt blob by patch ID
            println!("Reading quilt blob by patch ID: {}", quilt_patch_id);
            let read_quilt_data = client.read_quilt_blob_by_patch_id(quilt_patch_id).await?;
            println!("Read quilt blob data: {}", String::from_utf8_lossy(&read_quilt_data));
        }
    }

    Ok(())
}
```

## Blocking (Sync) Example

For environments where you can't use async, `walrus_rs` also provides a blocking client.

```rust
use walrus_rs::{BlockingWalrusClient, WalrusError};

fn main() -> Result<(), WalrusError> {
    let aggregator_url = std::env::var("AGGREGATOR")
        .unwrap_or_else(|_| "https://aggregator.testnet.walrus.atalma.io".to_string());
    let publisher_url = std::env::var("PUBLISHER")
        .unwrap_or_else(|_| "https://publisher.walrus-01.tududes.com".to_string());

    let client = BlockingWalrusClient::new(&aggregator_url, &publisher_url)?;

    // Example: Store a blob
    println!("Storing a blob...");
    let data = "some string from Rust SDK (blocking)".as_bytes().to_vec();
    let store_result = client.store_blob(data, Some(1), None, None, None)?;
    println!("Blob store result: {:?}", store_result);

    if let Some(newly_created) = store_result.newly_created {
        let blob_id = newly_created.blob_object.blob_id;
        println!("Newly created blob ID: {}", blob_id);

        // Example: Read a blob by ID
        println!("Reading blob by ID: {}", blob_id);
        let read_data = client.read_blob_by_id(&blob_id)?;
        println!("Read blob data: {}", String::from_utf8_lossy(&read_data));
    }
    Ok(())
}
```

## Running the Examples

To run the examples, ensure you have the `AGGREGATOR` and `PUBLISHER` environment variables set, or provide default values in the code.

To run the async example:
```bash
cargo run --example simple_usage
```

To run the blocking example:
```bash
cargo run --example blocking_usage
```

## License

This project is licensed under the MIT License.
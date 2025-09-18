# walrus_rs

`walrus_rs` 是一个 Rust 客户端库，用于与 Walrus API 进行交互。它提供了方便的接口来存储和读取 blob 和 quilt 数据。

## 安装

在你的 `Cargo.toml` 文件中添加以下依赖：

```toml
[dependencies]
walrus_rs = "0.1.3" # 替换为最新版本
reqwest = { version = "0.12", features = ["json", "multipart"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
url = "2"
thiserror = "1"
async-trait = "0.1"
```

## 异步示例

以下是一个简单的示例，演示如何使用异步的 `WalrusClient` 存储和读取 blob 和 quilt 数据：

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

## 阻塞 (同步) 示例

对于无法使用异步的环境，`walrus_rs` 也提供了一个阻塞客户端。

```rust
use walrus_rs::{BlockingWalrusClient, WalrusError};

fn main() -> Result<(), WalrusError> {
    let aggregator_url = std::env::var("AGGREGATOR")
        .unwrap_or_else(|_| "https://aggregator.testnet.walrus.atalma.io".to_string());
    let publisher_url = std::env::var("PUBLISHER")
        .unwrap_or_else(|_| "https://publisher.walrus-01.tududes.com".to_string());

    let client = BlockingWalrusClient::new(&aggregator_url, &publisher_url)?;

    // 示例：存储 blob
    println!("Storing a blob...");
    let data = "some string from Rust SDK (blocking)".as_bytes().to_vec();
    let store_result = client.store_blob(data, Some(1), None, None, None)?;
    println!("Blob store result: {:?}", store_result);

    if let Some(newly_created) = store_result.newly_created {
        let blob_id = newly_created.blob_object.blob_id;
        println!("Newly created blob ID: {}", blob_id);

        // 示例：通过 ID 读取 blob
        println!("Reading blob by ID: {}", blob_id);
        let read_data = client.read_blob_by_id(&blob_id)?;
        println!("Read blob data: {}", String::from_utf8_lossy(&read_data));
    }
    Ok(())
}
```

## 运行示例

要运行示例，请确保已设置 `AGGREGATOR` 和 `PUBLISHER` 环境变量，或者在代码中提供默认值。

运行异步示例：
```bash
cargo run --example simple_usage
```

运行阻塞示例：
```bash
cargo run --example blocking_usage
```

## 许可证

本项目根据 MIT 许可证发布。
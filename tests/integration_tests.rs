#[cfg(test)]
mod tests {
    use tokio;
    use walrus_rs::{WalrusClient, WalrusError};

    #[tokio::test]
    async fn test_walrus_client_new() {
        let aggregator_url = "http://localhost:8080";
        let publisher_url = "http://localhost:8081";
        let client = WalrusClient::new(aggregator_url, publisher_url).unwrap();

        assert_eq!(client.aggregator_url().as_str(), "http://localhost:8080/");
        assert_eq!(client.publisher_url().as_str(), "http://localhost:8081/");
    }

    #[tokio::test]
    async fn test_walrus_client_invalid_url() {
        let aggregator_url = "invalid-url";
        let publisher_url = "http://localhost:8081";
        let client_result = WalrusClient::new(aggregator_url, publisher_url);
        assert!(client_result.is_err());
        if let Err(WalrusError::InvalidUrl(msg)) = client_result {
            assert!(msg.contains("Invalid aggregator URL"));
        } else {
            panic!("Expected InvalidUrl error");
        }
    }
}
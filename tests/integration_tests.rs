#[cfg(test)]
mod tests {
    use tokio;
    use walrus_rs::{BlockingWalrusClient, WalrusClient, WalrusError};

    #[tokio::test]
    async fn test_walrus_client_new() {
        let aggregator_url = "https://aggregator.testnet.walrus.atalma.io/";
        let publisher_url = "https://publisher.walrus-01.tududes.com/";
        let client = WalrusClient::new(aggregator_url, publisher_url).unwrap();

        assert_eq!(
            client.aggregator_url().as_str(),
            "https://aggregator.testnet.walrus.atalma.io/"
        );
        assert_eq!(
            client.publisher_url().as_str(),
            "https://publisher.walrus-01.tududes.com/"
        );
    }

    #[tokio::test]
    async fn test_walrus_client_invalid_url() {
        let aggregator_url = "invalid-url";
        let publisher_url = "invalid-url";
        let client_result = WalrusClient::new(aggregator_url, publisher_url);
        assert!(client_result.is_err());
        if let Err(WalrusError::InvalidUrl(msg)) = client_result {
            assert!(msg.contains("Invalid aggregator URL"));
        } else {
            panic!("Expected InvalidUrl error");
        }
    }
    #[test]
    fn test_blocking_walrus_client_new() {
        let aggregator_url = "https://aggregator.testnet.walrus.atalma.io";
        let publisher_url = "https://publisher.walrus-01.tududes.com";
        let _client = BlockingWalrusClient::new(aggregator_url, publisher_url).unwrap();
        // We can't directly access the URLs in the blocking client,
        // but we can check that it was created successfully.
        // To properly test, we would need to mock the async client's methods
        // or expose the URLs, but for now, we'll just check for creation.
        assert!(true);
    }

    #[test]
    fn test_blocking_walrus_client_invalid_url() {
        let aggregator_url = "invalid-url";
        let publisher_url = "invalid-url";
        let client_result = BlockingWalrusClient::new(aggregator_url, publisher_url);
        assert!(client_result.is_err());
        if let Err(WalrusError::InvalidUrl(msg)) = client_result {
            assert!(msg.contains("Invalid aggregator URL"));
        } else {
            panic!("Expected InvalidUrl error");
        }
    }

    #[test]
    fn test_get_blob_metadata() {
        let aggregator_url = "https://aggregator.testnet.walrus.atalma.io";
        let publisher_url = "https://publisher.walrus-01.tududes.com";
        let client = BlockingWalrusClient::new(aggregator_url, publisher_url).unwrap();
        let metadata = client.get_blob_metadata("jUtX26C8c9csndZOUSrYmyLKlL_4CPfH1M4fnTI_kjY");
        assert!(metadata.is_ok());
    }
}

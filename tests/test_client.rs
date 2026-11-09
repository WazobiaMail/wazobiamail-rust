use std::time::Duration;
use wazobiamail::{WazobiaError, WazobiaMail};

mod common;

#[tokio::test]
async fn test_client_initialization_with_api_key() {
    let client = WazobiaMail::new("wzbm_live_testkey123456").unwrap();
    assert_eq!(
        client.config().api_key.as_deref(),
        Some("wzbm_live_testkey123456")
    );
    assert_eq!(
        client.config().base_url,
        "https://app.wazobiamail.com/api/v1"
    );
}

#[tokio::test]
async fn test_client_builder_custom_options() {
    let client = WazobiaMail::builder()
        .api_key("wzbm_custom_key")
        .base_url("https://custom.api.wazobiamail.com/api/v1/")
        .timeout(Duration::from_secs(45))
        .build()
        .unwrap();

    assert_eq!(client.config().api_key.as_deref(), Some("wzbm_custom_key"));
    assert_eq!(
        client.config().base_url,
        "https://custom.api.wazobiamail.com/api/v1"
    );
    assert_eq!(client.config().timeout, Duration::from_secs(45));
}

#[tokio::test]
async fn test_client_masks_secret_in_debug() {
    let client = WazobiaMail::new("wzbm_live_supersecretsecret9999").unwrap();
    let debug_str = format!("{client:?}");

    assert!(!debug_str.contains("supersecretsecret"));
    assert!(debug_str.contains("wzbm_...9999"));
}

#[tokio::test]
async fn test_client_fails_when_no_api_key() {
    std::env::remove_var("WAZOBIA_API_KEY");
    std::env::remove_var("WAZOBIAMAIL_API_KEY");

    let result = WazobiaMail::builder().build();
    assert!(matches!(result, Err(WazobiaError::Authentication(_))));
}

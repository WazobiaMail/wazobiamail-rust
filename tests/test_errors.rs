use wazobiamail::WazobiaError;

mod common;
use common::MockServer;

#[tokio::test]
async fn test_401_authentication_error() {
    let server = MockServer::start().await;
    server.enqueue_error(401, "Invalid API key provided", false, None);

    let client = server.client();
    let result = client.domains().list().await;

    match result {
        Err(WazobiaError::Authentication(msg)) => {
            assert!(msg.contains("Invalid API key"));
        }
        other => panic!("Expected Authentication error, got {other:?}"),
    }
}

#[tokio::test]
async fn test_403_forbidden_with_upgrade_notice() {
    let server = MockServer::start().await;
    server.enqueue_error(
        403,
        "Feature requires Business or Enterprise plan",
        true,
        None,
    );

    let client = server.client();
    let result = client.mailboxes().get_storage().await;

    match result {
        Err(WazobiaError::Forbidden {
            message,
            upgrade_required,
        }) => {
            assert!(message.contains("Feature requires"));
            assert!(upgrade_required);
        }
        other => panic!("Expected Forbidden error, got {other:?}"),
    }
}

#[tokio::test]
async fn test_404_not_found_error() {
    let server = MockServer::start().await;
    server.enqueue_error(404, "Domain dom_999 not found", false, None);

    let client = server.client();
    let result = client.domains().get("dom_999").await;

    match result {
        Err(WazobiaError::NotFound(msg)) => {
            assert!(msg.contains("dom_999"));
        }
        other => panic!("Expected NotFound error, got {other:?}"),
    }
}

#[tokio::test]
async fn test_409_conflict_error() {
    let server = MockServer::start().await;
    server.enqueue_error(409, "Domain already registered", false, None);

    let client = server.client();
    let result = client.domains().create("example.com").await;

    match result {
        Err(WazobiaError::Conflict(msg)) => {
            assert!(msg.contains("already registered"));
        }
        other => panic!("Expected Conflict error, got {other:?}"),
    }
}

#[tokio::test]
async fn test_422_validation_error() {
    let server = MockServer::start().await;
    let field_errors = serde_json::json!({
        "to": "Invalid email address format",
        "subject": "Subject is required"
    });
    server.enqueue_error(422, "Input validation failed", false, Some(field_errors));

    let client = server.client();
    let result = client.domains().create("").await;

    match result {
        Err(WazobiaError::Validation { message, errors }) => {
            assert!(message.contains("validation failed"));
            assert_eq!(errors.get("to").unwrap(), "Invalid email address format");
            assert_eq!(errors.get("subject").unwrap(), "Subject is required");
        }
        other => panic!("Expected Validation error, got {other:?}"),
    }
}

#[tokio::test]
async fn test_429_rate_limit_with_retry_after() {
    let server = MockServer::start().await;
    server.enqueue_error(429, "Too many requests", false, None);

    let client = server.client();
    let result = client.domains().list().await;

    match result {
        Err(WazobiaError::RateLimit {
            message,
            retry_after,
        }) => {
            assert!(message.contains("Too many requests"));
            assert_eq!(retry_after, Some(30));
        }
        other => panic!("Expected RateLimit error, got {other:?}"),
    }
}

#[tokio::test]
async fn test_500_server_error() {
    let server = MockServer::start().await;
    server.enqueue_error(500, "Internal gateway error", false, None);

    let client = server.client();
    let result = client.domains().list().await;

    match result {
        Err(WazobiaError::Server { status, message }) => {
            assert_eq!(status, 500);
            assert!(message.contains("Internal gateway"));
        }
        other => panic!("Expected Server error, got {other:?}"),
    }
}

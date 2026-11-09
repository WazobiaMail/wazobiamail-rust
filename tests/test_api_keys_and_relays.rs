use wazobiamail::models::api_keys::{CreateApiKeyRequest, CreateSmtpRelayRequest};

mod common;
use common::MockServer;

#[tokio::test]
async fn test_api_keys_lifecycle() {
    let server = MockServer::start().await;
    server.enqueue_json(
        201,
        serde_json::json!({
            "id": "key_101",
            "name": "Production Mailer",
            "scope": "sending",
            "keyPrefix": "wzbm_live",
            "apiKey": "wzbm_live_plainsecret_12345"
        }),
    );
    server.enqueue_json(
        200,
        serde_json::json!({
            "id": "key_101",
            "name": "Production Mailer",
            "scope": "sending",
            "keyPrefix": "wzbm_live",
            "apiKey": "wzbm_live_newsecret_99999"
        }),
    );
    server.enqueue_json(200, serde_json::Value::Null);

    let client = server.client();
    let req = CreateApiKeyRequest {
        name: "Production Mailer".to_string(),
        scope: "sending".to_string(),
        domain_id: None,
        expires_in_days: Some(90),
    };

    let created = client.api_keys().create(req).await.unwrap();
    assert_eq!(created.data.api_key, "wzbm_live_plainsecret_12345");

    let regenerated = client.api_keys().regenerate("key_101").await.unwrap();
    assert_eq!(regenerated.data.api_key, "wzbm_live_newsecret_99999");

    let revoked = client.api_keys().revoke("key_101").await.unwrap();
    assert!(revoked.success);

    let reqs = server.requests();
    assert_eq!(reqs[0].path, "/api/v1/api-keys");
    assert_eq!(reqs[1].path, "/api/v1/api-keys/key_101/regenerate");
    assert_eq!(reqs[2].path, "/api/v1/api-keys/key_101");
    assert_eq!(reqs[2].method, "DELETE");
}

#[tokio::test]
async fn test_smtp_relays_crud() {
    let server = MockServer::start().await;
    server.enqueue_json(
        201,
        serde_json::json!({
            "id": "relay_1",
            "name": "SendGrid Relay",
            "smtpHost": "smtp.sendgrid.net",
            "smtpPort": 587,
            "username": "apikey",
            "tlsMode": "starttls",
            "isActive": true
        }),
    );

    let client = server.client();
    let req = CreateSmtpRelayRequest {
        name: "SendGrid Relay".to_string(),
        smtp_host: "smtp.sendgrid.net".to_string(),
        smtp_port: 587,
        username: "apikey".to_string(),
        password: Some("SG.secret_key".to_string()),
        tls_mode: Some("starttls".to_string()),
    };

    let res = client.smtp_relays().create(req).await.unwrap();
    assert_eq!(res.data.id, "relay_1");
    assert_eq!(res.data.smtp_host, "smtp.sendgrid.net");
    assert_eq!(res.data.smtp_port, 587);

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/sending/smtp-relays");
}

#[tokio::test]
async fn test_smtp_access() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!({
            "enabled": true,
            "enforcementLevel": "mandatory"
        }),
    );

    let client = server.client();
    let res = client.smtp_access().update(true).await.unwrap();

    assert!(res.data.enabled);
    assert_eq!(res.data.enforcement_level.as_deref(), Some("mandatory"));

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/account/smtp-access");
    assert_eq!(req.method, "PUT");
}

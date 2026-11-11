use serde_json::json;
use wazobiamail::models::api_keys::CreateApiKeyRequest;
use wazobiamail::models::smtp::{CreateSmtpRelayRequest, UpdateSmtpRelayRequest};

mod common;
use common::MockServer;

const KEY_ID: &str = "k3Yq9xT2mN8pL4vR7wZa";
const RELAY_ID: &str = "R2mP8xQ4nT6vL1wZ9aBc";

#[tokio::test]
async fn test_api_key_list_uses_items_envelope() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "items": [
                {
                    "id": KEY_ID,
                    "name": "Production Mailer",
                    "scope": "general",
                    "domainId": null,
                    "domain": null,
                    "keyPrefix": "wzbm_4f8c2a1b9d",
                    "lastUsedAt": "2026-11-10T09:15:00",
                    "expiresAt": "2027-11-10T00:00:00",
                    "revokedAt": null,
                    "createdAt": "2026-11-09T12:00:00"
                }
            ]
        }),
    );

    let client = server.client();
    let res = client.api_keys().list().await.unwrap();

    assert_eq!(res.data.items.len(), 1);
    let key = &res.data.items[0];
    assert_eq!(key.id, KEY_ID);
    assert_eq!(key.scope, "general");
    assert_eq!(key.key_prefix, "wzbm_4f8c2a1b9d");
    assert_eq!(key.expires_at.as_deref(), Some("2027-11-10T00:00:00"));

    let req = server.last_request();
    assert_eq!(req.method, "GET");
    assert_eq!(req.path, "/api/v1/sending/api-keys");
}

#[tokio::test]
async fn test_api_key_create_regenerate_and_revoke_routes() {
    let server = MockServer::start().await;
    server.enqueue_json(
        201,
        json!({
            "apiKey": {
                "id": KEY_ID,
                "name": "Production Mailer",
                "scope": "general",
                "domainId": null,
                "keyPrefix": "wzbm_4f8c2a1b9d",
                "apiKey": "wzbm_4f8c2a1b9d0e7f6a5b4c3d2e1f0099887766",
                "expiresAt": "2027-11-10T00:00:00"
            }
        }),
    );
    server.enqueue_json(
        200,
        json!({
            "apiKey": {
                "id": KEY_ID,
                "name": "Production Mailer",
                "scope": "general",
                "domainId": null,
                "keyPrefix": "wzbm_9a8b7c6d5e",
                "apiKey": "wzbm_9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d",
                "expiresAt": "2027-11-10T00:00:00"
            }
        }),
    );
    server.enqueue_json(200, json!({ "keyId": KEY_ID }));

    let client = server.client();
    let request = CreateApiKeyRequest {
        name: "Production Mailer".to_string(),
        scope: Some("general".to_string()),
        domain_id: None,
        expires_at: Some("2027-11-10T00:00:00".to_string()),
    };

    let created = client.api_keys().create(request).await.unwrap();
    assert_eq!(
        created.data.api_key.api_key,
        "wzbm_4f8c2a1b9d0e7f6a5b4c3d2e1f0099887766"
    );
    assert_eq!(created.data.api_key.key_prefix, "wzbm_4f8c2a1b9d");

    let regenerated = client.api_keys().regenerate(KEY_ID).await.unwrap();
    assert_eq!(
        regenerated.data.api_key.api_key,
        "wzbm_9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d"
    );

    let revoked = client.api_keys().revoke(KEY_ID).await.unwrap();
    assert_eq!(revoked.data.key_id, KEY_ID);

    let reqs = server.requests();
    assert_eq!(reqs[0].path, "/api/v1/sending/api-keys");
    assert_eq!(reqs[0].method, "POST");
    assert_eq!(
        reqs[1].path,
        format!("/api/v1/sending/api-keys/{KEY_ID}/regenerate")
    );
    assert_eq!(reqs[1].method, "POST");
    assert_eq!(
        reqs[2].path,
        format!("/api/v1/sending/api-keys/{KEY_ID}/revoke")
    );
    assert_eq!(reqs[2].method, "POST");

    let body: serde_json::Value = reqs[0].json().unwrap();
    assert_eq!(body["name"], "Production Mailer");
    assert_eq!(body["scope"], "general");
    assert_eq!(body["expiresAt"], "2027-11-10T00:00:00");
    assert!(body.get("expiresInDays").is_none());
}

#[tokio::test]
async fn test_smtp_relay_list_and_create_use_backend_field_names() {
    let server = MockServer::start().await;
    let relay = json!({
        "id": RELAY_ID,
        "accountId": 42,
        "name": "SendGrid Relay",
        "smtpHost": "smtp.sendgrid.net",
        "smtpPort": 587,
        "smtpUsername": "apikey",
        "smtpSecurity": "starttls",
        "status": "active",
        "isDefault": true,
        "hasPassword": true,
        "createdAt": "2026-11-09T12:00:00",
        "updatedAt": "2026-11-09T12:00:00"
    });
    server.enqueue_json(200, json!({ "smtpRelays": [relay.clone()] }));
    server.enqueue_json(201, json!({ "smtpRelay": relay }));

    let client = server.client();
    let list = client.smtp_relays().list().await.unwrap();
    assert_eq!(list.data.smtp_relays.len(), 1);
    assert_eq!(list.data.smtp_relays[0].smtp_username, "apikey");
    assert!(list.data.smtp_relays[0].has_password);

    let created = client
        .smtp_relays()
        .create(CreateSmtpRelayRequest {
            name: "SendGrid Relay".to_string(),
            smtp_host: "smtp.sendgrid.net".to_string(),
            smtp_port: 587,
            smtp_username: "apikey".to_string(),
            smtp_password: "SG.secret_key".to_string(),
            smtp_security: "starttls".to_string(),
            status: Some("active".to_string()),
            is_default: Some(true),
        })
        .await
        .unwrap();

    assert_eq!(created.data.smtp_relay.id, RELAY_ID);
    assert_eq!(created.data.smtp_relay.smtp_port, 587);

    let reqs = server.requests();
    assert_eq!(reqs[0].path, "/api/v1/smtp-relays");
    assert_eq!(reqs[1].path, "/api/v1/smtp-relays");
    assert_eq!(reqs[1].method, "POST");

    let body: serde_json::Value = reqs[1].json().unwrap();
    assert_eq!(body["smtp_host"], "smtp.sendgrid.net");
    assert_eq!(body["smtp_port"], 587);
    assert_eq!(body["smtp_username"], "apikey");
    assert_eq!(body["smtp_password"], "SG.secret_key");
    assert_eq!(body["smtp_security"], "starttls");
    assert_eq!(body["is_default"], true);
    assert_eq!(body["status"], "active");
    assert!(body.get("username").is_none());
    assert!(body.get("password").is_none());
    assert!(body.get("tlsMode").is_none());
    assert!(body.get("smtpHost").is_none());
}

#[tokio::test]
async fn test_smtp_relay_update_and_state_transitions() {
    let server = MockServer::start().await;
    let relay = json!({
        "id": RELAY_ID,
        "accountId": 42,
        "name": "SendGrid Relay",
        "smtpHost": "smtp.sendgrid.net",
        "smtpPort": 587,
        "smtpUsername": "apikey",
        "smtpSecurity": "starttls",
        "status": "inactive",
        "isDefault": false,
        "hasPassword": true,
        "createdAt": "2026-11-09T12:00:00",
        "updatedAt": "2026-11-10T12:00:00"
    });
    server.enqueue_json(200, json!({ "smtpRelay": relay.clone() }));
    let mut active = relay.clone();
    active["status"] = json!("active");
    server.enqueue_json(200, json!({ "smtpRelay": active }));
    let mut inactive = relay.clone();
    inactive["status"] = json!("inactive");
    server.enqueue_json(200, json!({ "smtpRelay": inactive }));
    server.enqueue_json(200, json!({ "relayId": RELAY_ID }));

    let client = server.client();
    let updated = client
        .smtp_relays()
        .update(
            RELAY_ID,
            UpdateSmtpRelayRequest {
                status: Some("inactive".to_string()),
                ..UpdateSmtpRelayRequest::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.data.smtp_relay.status, "inactive");

    let activated = client.smtp_relays().activate(RELAY_ID).await.unwrap();
    assert_eq!(activated.data.smtp_relay.status, "active");

    let deactivated = client.smtp_relays().deactivate(RELAY_ID).await.unwrap();
    assert_eq!(deactivated.data.smtp_relay.status, "inactive");

    let deleted = client.smtp_relays().delete(RELAY_ID).await.unwrap();
    assert_eq!(deleted.data.relay_id, RELAY_ID);

    let reqs = server.requests();
    assert_eq!(reqs[0].method, "PATCH");
    assert_eq!(reqs[0].path, format!("/api/v1/smtp-relays/{RELAY_ID}"));
    let body: serde_json::Value = reqs[0].json().unwrap();
    assert_eq!(body["status"], "inactive");
    assert!(body.get("name").is_none());

    assert_eq!(reqs[1].method, "POST");
    assert_eq!(
        reqs[1].path,
        format!("/api/v1/smtp-relays/{RELAY_ID}/activate")
    );
    assert_eq!(reqs[2].method, "POST");
    assert_eq!(
        reqs[2].path,
        format!("/api/v1/smtp-relays/{RELAY_ID}/deactivate")
    );
    assert_eq!(reqs[3].method, "DELETE");
    assert_eq!(reqs[3].path, format!("/api/v1/smtp-relays/{RELAY_ID}"));
}

#[tokio::test]
async fn test_smtp_access_uses_sending_route_and_real_shape() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "paid": true,
            "eligible": true,
            "enabled": true,
            "persona": "company",
            "configured": true,
            "connection": {
                "host": "smtp.wazobiamail.com",
                "port": 587,
                "security": "STARTTLS",
                "usernames": ["support@brand.com"]
            }
        }),
    );
    server.enqueue_json(200, json!({ "enabled": false }));

    let client = server.client();
    let status = client.smtp_access().get().await.unwrap();
    assert!(status.data.paid);
    assert!(status.data.enabled);
    let connection = status.data.connection.unwrap();
    assert_eq!(connection.port, 587);
    assert_eq!(connection.usernames, vec!["support@brand.com"]);

    let updated = client.smtp_access().update(false).await.unwrap();
    assert!(!updated.data.enabled);

    let reqs = server.requests();
    assert_eq!(reqs[0].method, "GET");
    assert_eq!(reqs[0].path, "/api/v1/sending/smtp-access");
    assert_eq!(reqs[1].method, "PUT");
    assert_eq!(reqs[1].path, "/api/v1/sending/smtp-access");
    let body: serde_json::Value = reqs[1].json().unwrap();
    assert_eq!(body, json!({ "enabled": false }));
}

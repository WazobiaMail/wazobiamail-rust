use serde_json::json;

mod common;
use common::MockServer;

const MAILBOX_ID: &str = "m4iL8xQ2tR6pL1wZ9aBc";
const CREDENTIAL_ID: &str = "c7eD2xQ6tP4mL1wZ9aBc";

fn mailbox_fixture() -> serde_json::Value {
    json!({
        "id": MAILBOX_ID,
        "domainId": "d4mN8xQ2tR6pL1wZ9aBc",
        "address": "support@mail.brand.com",
        "localPart": "support",
        "displayName": "Support Team",
        "status": "active",
        "isPlatformSubdomain": false,
        "autoProvisioned": false,
        "role": "general",
        "createdAt": "2026-11-09T00:00:00",
        "updatedAt": "2026-11-09T00:00:00"
    })
}

fn credential_fixture() -> serde_json::Value {
    json!({
        "id": CREDENTIAL_ID,
        "mailboxId": MAILBOX_ID,
        "username": "support@mail.brand.com",
        "expiresAt": null,
        "revokedAt": null,
        "createdAt": "2026-11-09T00:00:00"
    })
}

#[tokio::test]
async fn test_list_mailboxes_returns_bare_array() {
    let server = MockServer::start().await;
    server.enqueue_json(200, json!([mailbox_fixture()]));

    let client = server.client();
    let res = client.mailboxes().list().await.unwrap();

    assert_eq!(res.data.len(), 1);
    assert_eq!(res.data[0].address, "support@mail.brand.com");
    assert_eq!(res.data[0].email(), "support@mail.brand.com");
    assert_eq!(res.data[0].role, "general");

    let req = server.last_request();
    assert_eq!(req.method, "GET");
    assert_eq!(req.path, "/api/v1/mailboxes");
}

#[tokio::test]
async fn test_create_mailbox_returns_mailbox_and_credential() {
    let server = MockServer::start().await;
    server.enqueue_json(
        201,
        json!({
            "mailbox": mailbox_fixture(),
            "credential": {
                "credential": credential_fixture(),
                "password": "GeneratedPassword123!"
            }
        }),
    );

    let client = server.client();
    let res = client
        .mailboxes()
        .create("d4mN8xQ2tR6pL1wZ9aBc", "support", Some("Support Team"))
        .await
        .unwrap();

    assert_eq!(res.data.mailbox.id, MAILBOX_ID);
    assert_eq!(res.data.mailbox.address, "support@mail.brand.com");
    assert_eq!(res.data.credential.password, "GeneratedPassword123!");
    assert_eq!(
        res.data.credential.credential.username,
        "support@mail.brand.com"
    );

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/mailboxes");
    let body: serde_json::Value = req.json().unwrap();
    assert_eq!(body["domainId"], "d4mN8xQ2tR6pL1wZ9aBc");
    assert_eq!(body["localPart"], "support");
    assert_eq!(body["displayName"], "Support Team");
    assert!(body.get("password").is_none());
}

#[tokio::test]
async fn test_get_update_and_delete_mailbox_envelopes() {
    let server = MockServer::start().await;
    server.enqueue_json(200, json!({ "mailbox": mailbox_fixture() }));
    let mut renamed = mailbox_fixture();
    renamed["displayName"] = json!("Customer Success");
    server.enqueue_json(200, json!({ "mailbox": renamed }));
    server.enqueue_json(200, json!({ "mailboxId": MAILBOX_ID }));

    let client = server.client();
    let fetched = client.mailboxes().get(MAILBOX_ID).await.unwrap();
    assert_eq!(fetched.data.mailbox.local_part, "support");

    let updated = client
        .mailboxes()
        .update(MAILBOX_ID, Some("Customer Success"))
        .await
        .unwrap();
    assert_eq!(
        updated.data.mailbox.display_name.as_deref(),
        Some("Customer Success")
    );

    let deleted = client.mailboxes().delete(MAILBOX_ID).await.unwrap();
    assert_eq!(deleted.data.mailbox_id, MAILBOX_ID);

    let reqs = server.requests();
    assert_eq!(reqs[0].path, format!("/api/v1/mailboxes/{MAILBOX_ID}"));
    assert_eq!(reqs[0].method, "GET");
    assert_eq!(reqs[1].path, format!("/api/v1/mailboxes/{MAILBOX_ID}"));
    assert_eq!(reqs[1].method, "PATCH");
    let body: serde_json::Value = reqs[1].json().unwrap();
    assert_eq!(body, json!({ "displayName": "Customer Success" }));
    assert_eq!(reqs[2].method, "DELETE");
}

#[tokio::test]
async fn test_mailbox_storage_real_shape() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "mailboxes": [
                {
                    "id": MAILBOX_ID,
                    "accountId": 42,
                    "address": "support@mail.brand.com",
                    "usedBytes": 524288000,
                    "quotaBytes": 1073741824,
                    "remainingBytes": 549453824,
                    "measuredAt": "2026-11-10T12:00:00"
                }
            ],
            "customers": [
                {
                    "accountId": 42,
                    "usedBytes": 524288000,
                    "allocatedBytes": 1073741824,
                    "quotaBytes": 10737418240i64,
                    "mailboxes": 1,
                    "measuredAt": "2026-11-10T12:00:00"
                }
            ],
            "domains": [
                {
                    "domainId": "d4mN8xQ2tR6pL1wZ9aBc",
                    "mailboxes": 1,
                    "includedMailboxes": 0,
                    "usedBytes": 524288000,
                    "allocatedBytes": 1073741824,
                    "measuredAt": "2026-11-10T12:00:00"
                }
            ],
            "measurement": "logical mailbox bytes",
            "plan": {
                "mailboxLimit": 50,
                "baseMailboxLimit": 25,
                "additionalMailboxLimit": 25,
                "mailboxesUsed": 1,
                "mailboxRemaining": 49,
                "storageTotalBytes": 53687091200i64,
                "storageAllocatedBytes": 1073741824,
                "storageRemainingBytes": 52613349376i64
            }
        }),
    );

    let client = server.client();
    let res = client.mailboxes().get_storage().await.unwrap();

    assert_eq!(res.data.measurement, "logical mailbox bytes");
    assert_eq!(res.data.mailboxes.len(), 1);
    assert_eq!(res.data.mailboxes[0].used_bytes, Some(524288000));
    assert_eq!(res.data.customers[0].mailboxes, 1);
    let plan = res.data.plan.unwrap();
    assert_eq!(plan.mailbox_limit, 50);
    assert_eq!(plan.mailboxes_used, 1);
    assert_eq!(plan.storage_remaining_bytes, Some(52613349376));

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/mailboxes/storage");
}

#[tokio::test]
async fn test_create_and_revoke_mailbox_credential() {
    let server = MockServer::start().await;
    server.enqueue_json(
        201,
        json!({
            "credential": {
                "credential": credential_fixture(),
                "password": "AnotherGeneratedPassword456!"
            }
        }),
    );
    server.enqueue_json(
        200,
        json!({ "mailboxId": MAILBOX_ID, "credentialId": CREDENTIAL_ID }),
    );

    let client = server.client();
    let created = client
        .mailboxes()
        .create_credential(MAILBOX_ID)
        .await
        .unwrap();
    assert_eq!(
        created.data.credential.password,
        "AnotherGeneratedPassword456!"
    );
    assert_eq!(created.data.credential.credential.id, CREDENTIAL_ID);

    let revoked = client
        .mailboxes()
        .revoke_credential(MAILBOX_ID, CREDENTIAL_ID)
        .await
        .unwrap();
    assert_eq!(revoked.data.mailbox_id, MAILBOX_ID);
    assert_eq!(revoked.data.credential_id, CREDENTIAL_ID);

    let reqs = server.requests();
    assert_eq!(
        reqs[0].path,
        format!("/api/v1/mailboxes/{MAILBOX_ID}/credentials")
    );
    assert_eq!(reqs[0].method, "POST");
    let body: serde_json::Value = reqs[0].json().unwrap();
    assert_eq!(body, json!({}));
    assert!(body.get("password").is_none());

    assert_eq!(
        reqs[1].path,
        format!("/api/v1/mailboxes/{MAILBOX_ID}/credentials/{CREDENTIAL_ID}/revoke")
    );
    assert_eq!(reqs[1].method, "POST");
}

#[tokio::test]
async fn test_headless_inbox_messages_list_and_get() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "items": [
                {
                    "id": "g7mS2xQ6tP4mL1wZ9aBc",
                    "threadId": "t3hR8xQ2tP6mL1wZ9aBc",
                    "from": [{ "name": "Alice", "email": "alice@client.com" }],
                    "to": [{ "name": "Support", "email": "support@mail.brand.com" }],
                    "subject": "Customer Inquiry #1024",
                    "preview": "Need help with billing invoice",
                    "receivedAt": "2026-11-09T00:00:00Z",
                    "isUnread": true,
                    "hasAttachments": true,
                    "sizeBytes": 15420
                }
            ],
            "total": 1,
            "page": 1,
            "limit": 25
        }),
    );
    server.enqueue_json(
        200,
        json!({
            "id": "g7mS2xQ6tP4mL1wZ9aBc",
            "threadId": "t3hR8xQ2tP6mL1wZ9aBc",
            "from": [{ "name": "Alice", "email": "alice@client.com" }],
            "to": [{ "name": "Support", "email": "support@mail.brand.com" }],
            "cc": [],
            "bcc": [],
            "replyTo": [{ "name": null, "email": "alice@client.com" }],
            "subject": "Customer Inquiry #1024",
            "preview": "Need help with billing invoice",
            "textBody": "Need help with billing invoice",
            "htmlBody": "<p>Need help with billing invoice</p>",
            "attachments": [
                {
                    "blobId": "b1lB8xQ2tP6mL1wZ9aBc",
                    "name": "receipt.pdf",
                    "type": "application/pdf",
                    "size": 14000
                }
            ],
            "receivedAt": "2026-11-09T00:00:00Z",
            "isUnread": false,
            "sizeBytes": 15420
        }),
    );

    let client = server.client();
    let list_res = client
        .mailboxes()
        .messages()
        .list_with_params(MAILBOX_ID, Some(true), Some(1), Some(25), Some("invoice"))
        .await
        .unwrap();

    assert_eq!(list_res.data.items.len(), 1);
    assert_eq!(list_res.data.total, 1);
    assert_eq!(list_res.data.page, 1);
    assert_eq!(list_res.data.limit, 25);
    assert_eq!(
        list_res.data.items[0].subject.as_deref(),
        Some("Customer Inquiry #1024")
    );
    assert_eq!(list_res.data.items[0].from[0].email, "alice@client.com");
    assert!(list_res.data.items[0].is_unread);

    let detail_res = client
        .mailboxes()
        .messages()
        .get(MAILBOX_ID, "g7mS2xQ6tP4mL1wZ9aBc")
        .await
        .unwrap();

    assert_eq!(detail_res.data.id, "g7mS2xQ6tP4mL1wZ9aBc");
    assert_eq!(
        detail_res.data.html_body.as_deref(),
        Some("<p>Need help with billing invoice</p>")
    );
    assert_eq!(detail_res.data.attachments.len(), 1);
    assert_eq!(detail_res.data.attachments[0].name, "receipt.pdf");
    assert_eq!(
        detail_res.data.attachments[0].content_type,
        "application/pdf"
    );

    let reqs = server.requests();
    assert_eq!(
        reqs[0].path,
        format!(
            "/api/v1/mailboxes/{MAILBOX_ID}/messages?unreadOnly=true&page=1&limit=25&search=invoice"
        )
    );
    assert_eq!(
        reqs[1].path,
        format!("/api/v1/mailboxes/{MAILBOX_ID}/messages/g7mS2xQ6tP4mL1wZ9aBc")
    );
}

#[tokio::test]
async fn test_headless_inbox_message_delete() {
    let server = MockServer::start().await;
    server.enqueue_json(200, json!({ "messageId": "g7mS2xQ6tP4mL1wZ9aBc" }));

    let client = server.client();
    let res = client
        .mailboxes()
        .messages()
        .delete(MAILBOX_ID, "g7mS2xQ6tP4mL1wZ9aBc")
        .await
        .unwrap();

    assert_eq!(res.data.message_id, "g7mS2xQ6tP4mL1wZ9aBc");

    let req = server.last_request();
    assert_eq!(req.method, "DELETE");
    assert_eq!(
        req.path,
        format!("/api/v1/mailboxes/{MAILBOX_ID}/messages/g7mS2xQ6tP4mL1wZ9aBc")
    );
}

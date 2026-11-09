mod common;
use common::MockServer;

#[tokio::test]
async fn test_create_mailbox_and_credential() {
    let server = MockServer::start().await;
    server.enqueue_json(
        201,
        serde_json::json!({
            "mailbox": {
                "id": "mbx_123",
                "domainId": "dom_123",
                "email": "support@brand.com",
                "localPart": "support",
                "displayName": "Support Team",
                "status": "active",
                "storageUsageBytes": 0,
                "createdAt": "2026-11-09T00:00:00Z"
            },
            "credential": {
                "id": "cred_1",
                "username": "support@brand.com",
                "password": "GeneratedPassword123!",
                "createdAt": "2026-11-09T00:00:00Z"
            }
        }),
    );

    let client = server.client();
    let res = client
        .mailboxes()
        .create("dom_123", "support", Some("Support Team"))
        .await
        .unwrap();

    assert_eq!(res.data.mailbox.email, "support@brand.com");
    assert_eq!(
        res.data.credential.as_ref().unwrap().password.as_deref(),
        Some("GeneratedPassword123!")
    );

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/mailboxes");
    assert_eq!(req.method, "POST");
}

#[tokio::test]
async fn test_get_storage_quotas() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!({
            "totalBytes": 524288000,
            "quotaBytes": 10737418240u64,
            "percentUsed": 4.88,
            "mailboxesCount": 3
        }),
    );

    let client = server.client();
    let res = client.mailboxes().get_storage().await.unwrap();

    assert_eq!(res.data.mailboxes_count, 3);
    assert_eq!(res.data.percent_used, 4.88);

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/mailboxes/storage");
}

#[tokio::test]
async fn test_headless_inbox_messages_list_and_get() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!({
            "items": [
                {
                    "id": "msg_001",
                    "messageId": "<inq@client.com>",
                    "subject": "Customer Inquiry #1024",
                    "from": {
                        "name": "Alice",
                        "email": "alice@client.com"
                    },
                    "to": ["support@brand.com"],
                    "date": "2026-11-09T00:00:00Z",
                    "isRead": false,
                    "hasAttachments": true,
                    "sizeBytes": 15420
                }
            ],
            "pagination": {
                "currentPage": 1,
                "pageSize": 25,
                "totalItems": 1,
                "totalPages": 1,
                "hasNext": false,
                "hasPrev": false
            }
        }),
    );
    server.enqueue_json(
        200,
        serde_json::json!({
            "id": "msg_001",
            "messageId": "<inq@client.com>",
            "subject": "Customer Inquiry #1024",
            "from": {
                "name": "Alice",
                "email": "alice@client.com"
            },
            "to": ["support@brand.com"],
            "date": "2026-11-09T00:00:00Z",
            "isRead": true,
            "hasAttachments": true,
            "sizeBytes": 15420,
            "htmlBody": "<p>Need help with billing invoice</p>",
            "textBody": "Need help with billing invoice",
            "headers": {
                "x-priority": "1"
            },
            "attachments": [
                {
                    "id": "att_1",
                    "fileName": "receipt.pdf",
                    "contentType": "application/pdf",
                    "sizeBytes": 14000
                }
            ]
        }),
    );

    let client = server.client();
    let list_res = client
        .mailboxes()
        .messages()
        .list("mbx_123", Some(true))
        .await
        .unwrap();

    assert_eq!(list_res.data.items.len(), 1);
    assert_eq!(list_res.data.items[0].subject, "Customer Inquiry #1024");
    assert!(!list_res.data.items[0].is_read);

    let detail_res = client
        .mailboxes()
        .messages()
        .get("mbx_123", "msg_001")
        .await
        .unwrap();

    assert_eq!(detail_res.data.id, "msg_001");
    assert_eq!(
        detail_res.data.html_body.as_deref(),
        Some("<p>Need help with billing invoice</p>")
    );
    assert_eq!(detail_res.data.attachments.len(), 1);
    assert_eq!(detail_res.data.attachments[0].file_name, "receipt.pdf");

    let reqs = server.requests();
    assert!(reqs[0]
        .path
        .starts_with("/api/v1/mailboxes/mbx_123/messages"));
    assert!(reqs[0].path.contains("unreadOnly=true"));
    assert_eq!(reqs[1].path, "/api/v1/mailboxes/mbx_123/messages/msg_001");
}

#[tokio::test]
async fn test_headless_inbox_message_delete() {
    let server = MockServer::start().await;
    server.enqueue_json(200, serde_json::Value::Null);

    let client = server.client();
    let res = client
        .mailboxes()
        .messages()
        .delete("mbx_123", "msg_del_99")
        .await
        .unwrap();

    assert!(res.success);

    let req = server.last_request();
    assert_eq!(req.method, "DELETE");
    assert_eq!(req.path, "/api/v1/mailboxes/mbx_123/messages/msg_del_99");
}

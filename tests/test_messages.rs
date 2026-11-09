use wazobiamail::models::{AttachmentFile, SendMessageRequest, SpamCheckRequest, UrlAttachment};

mod common;
use common::MockServer;

#[tokio::test]
async fn test_send_email_wire_contract_and_receipt() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!({
            "status": "queued",
            "trackingId": "trk_abc123456",
            "recipients": 1
        }),
    );

    let client = server.client();
    let request = SendMessageRequest::builder()
        .from("Acme Store <hello@store.com>")
        .to("customer@example.com")
        .subject("Welcome to WazobiaMail")
        .html("<p>Welcome aboard!</p>")
        .text("Welcome aboard!")
        .reply_to("support@store.com")
        .attachment(UrlAttachment::new(
            "https://cdn.example.com/guide.pdf",
            "guide.pdf",
        ))
        .build()
        .unwrap();

    let res = client.send(request).await.unwrap();

    assert!(res.success);
    assert_eq!(res.data.tracking_id, "trk_abc123456");
    assert_eq!(res.data.status, "queued");
    assert_eq!(res.data.recipients, 1);

    let req = server.last_request();
    assert_eq!(req.method, "POST");
    assert_eq!(req.path, "/api/v1/sending/messages");
    assert_eq!(req.header("X-API-Key"), Some("wzbm_mock_key_123456789"));
    assert_eq!(req.header("User-Agent"), Some("wazobiamail-rust/0.1.0"));

    let json: serde_json::Value = req.json().unwrap();
    assert_eq!(json["to"][0], "customer@example.com");
    assert_eq!(json["content"]["subject"], "Welcome to WazobiaMail");
    assert_eq!(json["content"]["htmlBody"], "<p>Welcome aboard!</p>");
    assert_eq!(json["content"]["replyTo"], "support@store.com");
    assert_eq!(json["attachments"][0]["fileName"], "guide.pdf");
}

#[tokio::test]
async fn test_send_with_idempotency_key() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!({
            "status": "queued",
            "trackingId": "trk_idemp_1"
        }),
    );

    let client = server.client();
    let request = SendMessageRequest::builder()
        .to("idempotent@example.com")
        .subject("Idempotent Send")
        .text("Hello")
        .build()
        .unwrap();

    let res = client
        .send_with_idempotency(request, "idemp_key_987654")
        .await
        .unwrap();

    assert!(res.success);
    let req = server.last_request();
    assert_eq!(req.header("Idempotency-Key"), Some("idemp_key_987654"));
}

#[tokio::test]
async fn test_spam_check_and_findings() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!({
            "score": 0.5,
            "threshold": 5.0,
            "deliverabilityScore": 95,
            "findings": [
                {
                    "rule": "SUBJ_ALL_CAPS",
                    "score": 0.5,
                    "description": "Subject line is all capital letters"
                }
            ],
            "recommendations": ["Avoid uppercase words in subject line"]
        }),
    );

    let client = server.client();
    let request = SpamCheckRequest::builder()
        .subject("CLAIM YOUR PRIZE")
        .html("<p>Click here</p>")
        .build()
        .unwrap();

    let res = client.messages().spam_check(request).await.unwrap();

    assert_eq!(res.data.deliverability_score, 95);
    assert_eq!(res.data.findings.len(), 1);
    assert_eq!(res.data.findings[0].rule, "SUBJ_ALL_CAPS");
    assert_eq!(
        res.data.recommendations[0],
        "Avoid uppercase words in subject line"
    );

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/sending/spam-check");
}

#[tokio::test]
async fn test_multipart_send_with_attachments() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!({
            "status": "queued",
            "trackingId": "trk_multi_1"
        }),
    );

    let client = server.client();
    let request = SendMessageRequest::builder()
        .to("recipient@example.com")
        .subject("Files Attached")
        .html("<p>Here is your receipt</p>")
        .build()
        .unwrap();

    let files = vec![AttachmentFile::from_bytes(
        "receipt.txt",
        b"Payment Confirmed",
        "text/plain",
    )];

    let res = client
        .messages()
        .send_with_attachments(request, files)
        .await
        .unwrap();

    assert!(res.success);
    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/sending/messages/with-attachments");
    assert!(req
        .header("Content-Type")
        .unwrap()
        .starts_with("multipart/form-data"));
    assert!(req.text().contains("receipt.txt"));
    assert!(req.text().contains("Payment Confirmed"));
}

#[tokio::test]
async fn test_upload_attachment() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!({
            "fileName": "logo.png",
            "contentType": "image/png",
            "sizeBytes": 1024,
            "publicUrl": "https://cdn.wazobiamail.com/logo.png"
        }),
    );

    let client = server.client();
    let file = AttachmentFile::from_bytes("logo.png", b"fake png data", "image/png");
    let res = client.messages().upload_attachment(file).await.unwrap();

    assert_eq!(res.data.file_name, "logo.png");
    assert_eq!(res.data.public_url, "https://cdn.wazobiamail.com/logo.png");

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/sending/attachments/upload");
}

#[tokio::test]
async fn test_list_tracking_messages() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!({
            "items": [
                {
                    "trackingId": "trk_101",
                    "subject": "Monthly Statement",
                    "from": "billing@store.com",
                    "recipientsCount": 1,
                    "opensCount": 3,
                    "clicksCount": 1,
                    "status": "delivered",
                    "sentAt": "2026-11-09T00:00:00Z"
                }
            ],
            "total": 1
        }),
    );

    let client = server.client();
    let res = client
        .messages()
        .list_tracking(Some(10), None, Some("delivered"))
        .await
        .unwrap();

    assert_eq!(res.data.items.len(), 1);
    assert_eq!(res.data.items[0].tracking_id, "trk_101");
    assert_eq!(res.data.items[0].opens_count, 3);

    let req = server.last_request();
    assert!(req.path.starts_with("/api/v1/sending/tracking/messages"));
    assert!(req.path.contains("limit=10"));
    assert!(req.path.contains("status=delivered"));
    assert!(req.header("Host").is_some());
}

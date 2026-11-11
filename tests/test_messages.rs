use serde_json::json;
use wazobiamail::models::{AttachmentFile, SendMessageRequest, SpamCheckRequest, UrlAttachment};

mod common;
use common::MockServer;

#[tokio::test]
async fn test_send_email_wire_contract_and_receipt() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "result": {
                "status": "queued",
                "trackingId": "trk_abc123456",
                "recipients": 1
            }
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
    assert_eq!(res.data.result.tracking_id, "trk_abc123456");
    assert_eq!(res.data.result.status, "queued");
    assert_eq!(res.data.result.recipients, 1);

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
        json!({
            "result": {
                "status": "queued",
                "trackingId": "trk_idemp_1",
                "recipients": 1
            }
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

    assert_eq!(res.data.result.tracking_id, "trk_idemp_1");
    let req = server.last_request();
    assert_eq!(req.header("Idempotency-Key"), Some("idemp_key_987654"));
}

#[tokio::test]
async fn test_spam_check_real_response_shape() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "score": 2.5,
            "threshold": 5.0,
            "isSpam": false,
            "rating": "Low risk",
            "badgeColor": "green",
            "deliverabilityScore": 92,
            "summary": "2 of 9 checks triggered",
            "rules": [
                {
                    "code": "SUBJ_ALL_CAPS",
                    "points": 1.5,
                    "title": "Subject is all capital letters",
                    "description": "All-caps subjects are a common spam signal.",
                    "recommendation": "Avoid uppercase words in the subject line"
                }
            ]
        }),
    );

    let client = server.client();
    let request = SpamCheckRequest::builder()
        .subject("CLAIM YOUR PRIZE")
        .html("<p>Click here</p>")
        .build()
        .unwrap();

    let res = client.messages().spam_check(request).await.unwrap();

    assert_eq!(res.data.deliverability_score, 92);
    assert!(!res.data.is_spam);
    assert_eq!(res.data.rating, "Low risk");
    assert_eq!(res.data.rules.len(), 1);
    assert_eq!(res.data.rules[0].code, "SUBJ_ALL_CAPS");
    assert_eq!(res.data.rules[0].points, 1.5);
    assert_eq!(
        res.data.rules[0].recommendation,
        "Avoid uppercase words in the subject line"
    );

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/sending/spam-check");
    let body: serde_json::Value = req.json().unwrap();
    assert_eq!(body["subject"], "CLAIM YOUR PRIZE");
    assert_eq!(body["html"], "<p>Click here</p>");
}

#[tokio::test]
async fn test_multipart_send_with_attachments() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "result": {
                "status": "queued",
                "trackingId": "trk_multi_1",
                "recipients": 1
            }
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

    assert_eq!(res.data.result.tracking_id, "trk_multi_1");
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
async fn test_upload_attachment_returns_file_envelope() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "file": {
                "fileName": "logo.png",
                "contentType": "image/png",
                "sizeBytes": 1024,
                "publicUrl": "https://cdn.wazobiamail.com/acct-42/uploads/logo.png"
            }
        }),
    );

    let client = server.client();
    let file = AttachmentFile::from_bytes("logo.png", b"fake png data", "image/png");
    let res = client.messages().upload_attachment(file).await.unwrap();

    assert_eq!(res.data.file.file_name, "logo.png");
    assert_eq!(
        res.data.file.public_url.as_deref(),
        Some("https://cdn.wazobiamail.com/acct-42/uploads/logo.png")
    );

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/sending/attachments/upload");
}

#[tokio::test]
async fn test_list_tracking_messages_uses_messages_and_summary() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "messages": [
                {
                    "trackingId": "trk_101",
                    "senderDomain": "mail.brand.com",
                    "subject": "Monthly Statement",
                    "recipientsCount": 1,
                    "acceptedCount": 1,
                    "deliveredCount": 1,
                    "bouncedCount": 0,
                    "openCount": 3,
                    "clickCount": 1,
                    "status": "delivered",
                    "createdAt": "2026-11-09T00:00:00",
                    "lastOpenedAt": "2026-11-09T08:00:00",
                    "lastClickedAt": "2026-11-09T08:05:00",
                    "lastDeliveryEventAt": "2026-11-09T00:02:00"
                }
            ],
            "summary": {
                "messages": 1,
                "recipients": 1,
                "accepted": 1,
                "delivered": 1,
                "bounced": 0,
                "opens": 3,
                "clicks": 1
            }
        }),
    );

    let client = server.client();
    let res = client
        .messages()
        .list_tracking(Some(10), None, Some("delivered"))
        .await
        .unwrap();

    assert_eq!(res.data.messages.len(), 1);
    assert_eq!(res.data.messages[0].tracking_id, "trk_101");
    assert_eq!(res.data.messages[0].open_count, 3);
    assert_eq!(res.data.messages[0].click_count, 1);
    assert_eq!(res.data.summary.messages, 1);
    assert_eq!(res.data.summary.opens, 3);

    let req = server.last_request();
    assert!(req.path.starts_with("/api/v1/sending/tracking/messages"));
    assert!(req.path.contains("limit=10"));
    assert!(req.path.contains("status=delivered"));
}

#[tokio::test]
async fn test_get_tracking_detail_wraps_message_and_events() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "message": {
                "message": {
                    "trackingId": "trk_101",
                    "senderDomain": "mail.brand.com",
                    "subject": "Monthly Statement",
                    "recipientsCount": 1,
                    "acceptedCount": 1,
                    "deliveredCount": 1,
                    "bouncedCount": 0,
                    "openCount": 3,
                    "clickCount": 1,
                    "status": "delivered",
                    "createdAt": "2026-11-09T00:00:00",
                    "lastOpenedAt": "2026-11-09T08:00:00",
                    "lastClickedAt": "2026-11-09T08:05:00",
                    "lastDeliveryEventAt": "2026-11-09T00:02:00"
                },
                "events": [
                    {
                        "id": "e5vT2xQ6tP4mL1wZ9aBc",
                        "eventType": "click",
                        "recipientEmail": "customer@example.com",
                        "clickUrl": "https://brand.com/invoices",
                        "occurredAt": "2026-11-09T08:05:00",
                        "engagement": { "userAgent": "Mozilla/5.0" }
                    }
                ]
            }
        }),
    );

    let client = server.client();
    let res = client.messages().get_tracking("trk_101").await.unwrap();

    let detail = &res.data.message;
    assert_eq!(detail.message.tracking_id, "trk_101");
    assert_eq!(detail.message.status, "delivered");
    assert_eq!(detail.events.len(), 1);
    assert_eq!(detail.events[0].event_type, "click");
    assert_eq!(
        detail.events[0].click_url.as_deref(),
        Some("https://brand.com/invoices")
    );

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/sending/tracking/messages/trk_101");
}

#[tokio::test]
async fn test_list_email_logs_real_shape() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "available": true,
            "messages": [
                {
                    "accountId": 42,
                    "trackingId": "trk_101",
                    "subject": "Monthly Statement",
                    "senderDomain": "mail.brand.com",
                    "from": "billing@mail.brand.com",
                    "fromName": "Billing",
                    "method": "api",
                    "to": ["customer@example.com"],
                    "cc": [],
                    "receivedAt": "2026-11-09T00:00:00.000000Z",
                    "status": "delivered",
                    "recipientsCount": 1,
                    "deliveredCount": 1,
                    "bouncedCount": 0,
                    "diagnostic": null,
                    "events": []
                }
            ],
            "hasMore": false,
            "eventDetailsAvailable": false
        }),
    );

    let client = server.client();
    let res = client.messages().list_logs(Some(25), None).await.unwrap();

    assert!(res.data.available);
    assert!(!res.data.has_more);
    assert_eq!(res.data.messages.len(), 1);
    assert_eq!(res.data.messages[0].tracking_id, "trk_101");
    assert_eq!(res.data.messages[0].method.as_deref(), Some("api"));
    assert_eq!(res.data.messages[0].to, vec!["customer@example.com"]);

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/account/email-logs?limit=25");
}

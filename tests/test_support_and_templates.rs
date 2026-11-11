use serde_json::json;
use wazobiamail::models::support::{
    CreateSupportTicketRequest, ReplySupportRequest, SupportUpload,
};
use wazobiamail::models::templates::{
    CreateTemplateRequest, TemplateListQuery, UpdateTemplateRequest,
};

mod common;
use common::MockServer;

const TICKET_ID: &str = "t8cK2xQ6tP4mL1wZ9aBc";
const TEMPLATE_ID: &str = "t3mP8xQ2tR6pL1wZ9aBc";

#[tokio::test]
async fn test_support_list_my_tickets_uses_support_my_and_ticket_page() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "items": [
                {
                    "id": TICKET_ID,
                    "accountId": 42,
                    "name": "Ada Obi",
                    "email": "ada@brand.com",
                    "subject": "Cannot verify domain",
                    "domain": "brand.com",
                    "category": "technical",
                    "priority": "high",
                    "status": "open",
                    "messages": [],
                    "customerReadAt": null,
                    "staffReadAt": null,
                    "createdAt": "2026-11-09T10:00:00",
                    "updatedAt": "2026-11-09T10:00:00"
                }
            ],
            "total": 1,
            "page": 1,
            "pages": 1
        }),
    );

    server.enqueue_json(
        200,
        json!({
            "items": [],
            "total": 0,
            "page": 1,
            "pages": 0
        }),
    );

    let client = server.client();
    let res = client.support().list().await.unwrap();

    assert_eq!(res.data.items.len(), 1);
    assert_eq!(res.data.items[0].subject, "Cannot verify domain");
    assert_eq!(res.data.items[0].status, "open");
    assert_eq!(res.data.total, 1);
    assert_eq!(res.data.page, 1);
    assert_eq!(res.data.pages, 1);

    let req = server.last_request();
    assert_eq!(req.method, "GET");
    assert_eq!(req.path, "/api/v1/support/my");

    client
        .support()
        .list_with_params(Some("open"), Some(1), Some(20))
        .await
        .unwrap();
    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/support/my?status=open&page=1&limit=20");
}

#[tokio::test]
async fn test_support_create_uses_support_route_and_full_body() {
    let server = MockServer::start().await;
    server.enqueue_json(201, json!({ "id": TICKET_ID, "ticketId": TICKET_ID }));

    let client = server.client();
    let res = client
        .support()
        .create(CreateSupportTicketRequest {
            name: "Ada Obi".to_string(),
            email: "ada@brand.com".to_string(),
            subject: "Cannot verify domain".to_string(),
            domain: Some("brand.com".to_string()),
            category: "technical".to_string(),
            priority: "high".to_string(),
            message: "The DKIM record still shows as pending.".to_string(),
            content_type: "text".to_string(),
            attachments: vec![SupportUpload {
                filename: "dig-output.txt".to_string(),
                data: "diB2ZXJpZmljYXRpb24=".to_string(),
            }],
        })
        .await
        .unwrap();

    assert_eq!(res.data.id, TICKET_ID);
    assert_eq!(res.data.ticket_id, TICKET_ID);

    let req = server.last_request();
    assert_eq!(req.method, "POST");
    assert_eq!(req.path, "/api/v1/support");
    let body: serde_json::Value = req.json().unwrap();
    assert_eq!(body["name"], "Ada Obi");
    assert_eq!(body["email"], "ada@brand.com");
    assert_eq!(body["category"], "technical");
    assert_eq!(body["priority"], "high");
    assert_eq!(body["contentType"], "text");
    assert_eq!(body["attachments"][0]["filename"], "dig-output.txt");
}

#[tokio::test]
async fn test_support_get_ticket_and_reply() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "id": TICKET_ID,
            "accountId": 42,
            "name": "Ada Obi",
            "email": "ada@brand.com",
            "subject": "Cannot verify domain",
            "domain": "brand.com",
            "category": "technical",
            "priority": "high",
            "status": "open",
            "messages": [
                {
                    "id": "m5sG2xQ6tP4mL1wZ9aBc",
                    "ticketId": TICKET_ID,
                    "senderName": "Ada Obi",
                    "senderEmail": "ada@brand.com",
                    "message": "The DKIM record still shows as pending.",
                    "isFromStaff": false,
                    "contentType": "text",
                    "attachments": [
                        {
                            "id": "a1tT8xQ2tP6mL1wZ9aBc",
                            "filename": "dig-output.txt",
                            "sizeBytes": 18
                        }
                    ],
                    "createdAt": "2026-11-09T10:00:00"
                }
            ],
            "customerReadAt": "2026-11-09T10:05:00",
            "staffReadAt": null,
            "createdAt": "2026-11-09T10:00:00",
            "updatedAt": "2026-11-09T10:05:00"
        }),
    );
    server.enqueue_json(200, json!({}));

    let client = server.client();
    let ticket = client.support().get(TICKET_ID).await.unwrap();
    assert_eq!(ticket.data.messages.len(), 1);
    assert_eq!(ticket.data.messages[0].sender_email, "ada@brand.com");
    assert_eq!(
        ticket.data.messages[0].attachments[0].filename,
        "dig-output.txt"
    );
    assert!(!ticket.data.messages[0].is_from_staff);

    let replied = client
        .support()
        .reply_with(
            TICKET_ID,
            ReplySupportRequest::text("Thanks, checking now."),
        )
        .await
        .unwrap();
    assert!(replied.success);

    let reqs = server.requests();
    assert_eq!(reqs[0].path, format!("/api/v1/support/tickets/{TICKET_ID}"));
    assert_eq!(reqs[0].method, "GET");
    assert_eq!(
        reqs[1].path,
        format!("/api/v1/support/tickets/{TICKET_ID}/messages")
    );
    assert_eq!(reqs[1].method, "POST");
    let body: serde_json::Value = reqs[1].json().unwrap();
    assert_eq!(body["message"], "Thanks, checking now.");
    assert_eq!(body["contentType"], "text");
    assert_eq!(body["attachments"], json!([]));
}

fn template_fixture() -> serde_json::Value {
    json!({
        "id": TEMPLATE_ID,
        "accountId": 42,
        "name": "Order Confirmation",
        "slug": "order_confirmation",
        "channel": "email",
        "category": "transactional",
        "status": "active",
        "rejectionReason": null,
        "subject": "Order {{order_id}} confirmed",
        "body": "<p>Thanks {{name}}</p>",
        "textBody": "Thanks {{name}}",
        "metaTemplateId": null,
        "metaTemplateName": null,
        "language": "en_US",
        "components": [],
        "variables": ["name", "order_id"],
        "design": null,
        "previewImageUrl": null,
        "isSystem": false,
        "revision": 1,
        "createdAt": "2026-11-09T10:00:00Z",
        "updatedAt": "2026-11-09T10:00:00Z"
    })
}

#[tokio::test]
async fn test_templates_list_uses_items_total_limit_offset() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "items": [template_fixture()],
            "total": 1,
            "limit": 50,
            "offset": 0
        }),
    );
    server.enqueue_json(
        200,
        json!({
            "items": [template_fixture()],
            "total": 1,
            "limit": 10,
            "offset": 20
        }),
    );

    let client = server.client();
    let res = client.templates().list().await.unwrap();

    assert_eq!(res.data.items.len(), 1);
    assert_eq!(res.data.items[0].slug, "order_confirmation");
    assert_eq!(res.data.items[0].body, "<p>Thanks {{name}}</p>");
    assert_eq!(res.data.total, 1);
    assert_eq!(res.data.limit, 50);

    client
        .templates()
        .list_with_query(TemplateListQuery {
            channel: Some("email".to_string()),
            category: Some("transactional".to_string()),
            status: Some("active".to_string()),
            search: Some("order".to_string()),
            limit: Some(10),
            offset: Some(20),
        })
        .await
        .unwrap();

    let reqs = server.requests();
    assert_eq!(reqs[0].path, "/api/v1/templates");
    assert_eq!(
        reqs[1].path,
        "/api/v1/templates?channel=email&category=transactional&status=active&search=order&limit=10&offset=20"
    );
}

#[tokio::test]
async fn test_templates_create_update_and_preview_bodies() {
    let server = MockServer::start().await;
    server.enqueue_json(200, template_fixture());
    let mut updated = template_fixture();
    updated["subject"] = json!("Order {{order_id}} is on its way");
    server.enqueue_json(200, updated);
    server.enqueue_json(
        200,
        json!({
            "channel": "email",
            "subject": "Order 1042 is on its way",
            "renderedBody": "<p>Thanks Ada</p>",
            "renderedText": "Thanks Ada",
            "status": "active",
            "canSend": true
        }),
    );

    let client = server.client();
    let created = client
        .templates()
        .create(CreateTemplateRequest {
            name: "Order Confirmation".to_string(),
            channel: "email".to_string(),
            category: "transactional".to_string(),
            subject: "Order {{order_id}} confirmed".to_string(),
            body: "<p>Thanks {{name}}</p>".to_string(),
            text_body: "Thanks {{name}}".to_string(),
            language: "en_US".to_string(),
            components: Some(json!([])),
            variables: Some(json!(["name", "order_id"])),
            design: None,
            auto_submit_meta: false,
            is_system: false,
        })
        .await
        .unwrap();
    assert_eq!(created.data.id, TEMPLATE_ID);

    let updated = client
        .templates()
        .update(
            TEMPLATE_ID,
            UpdateTemplateRequest {
                subject: Some("Order {{order_id}} is on its way".to_string()),
                ..UpdateTemplateRequest::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.data.subject, "Order {{order_id}} is on its way");

    let preview = client
        .templates()
        .preview(
            TEMPLATE_ID,
            Some(json!({ "name": "Ada", "order_id": "1042" })),
        )
        .await
        .unwrap();
    assert_eq!(preview.data.rendered_body, "<p>Thanks Ada</p>");
    assert!(preview.data.can_send);

    let reqs = server.requests();
    assert_eq!(reqs[0].path, "/api/v1/templates");
    assert_eq!(reqs[0].method, "POST");
    let body: serde_json::Value = reqs[0].json().unwrap();
    assert_eq!(body["name"], "Order Confirmation");
    assert_eq!(body["body"], "<p>Thanks {{name}}</p>");
    assert_eq!(body["textBody"], "Thanks {{name}}");
    assert_eq!(body["channel"], "email");
    assert!(body.get("html").is_none());
    assert!(body.get("text").is_none());

    assert_eq!(reqs[1].path, format!("/api/v1/templates/{TEMPLATE_ID}"));
    assert_eq!(reqs[1].method, "PUT");
    let body: serde_json::Value = reqs[1].json().unwrap();
    assert_eq!(body["subject"], "Order {{order_id}} is on its way");
    assert!(body.get("name").is_none());

    assert_eq!(
        reqs[2].path,
        format!("/api/v1/templates/{TEMPLATE_ID}/preview")
    );
    assert_eq!(reqs[2].method, "POST");
    let body: serde_json::Value = reqs[2].json().unwrap();
    assert_eq!(body["sampleVariables"]["name"], "Ada");
    assert!(body.get("name").is_none());
}

#[tokio::test]
async fn test_template_delete_accepts_null_data() {
    let server = MockServer::start().await;
    server.enqueue_json(200, serde_json::Value::Null);

    let client = server.client();
    let res = client.templates().delete(TEMPLATE_ID).await.unwrap();
    assert!(res.success);

    let req = server.last_request();
    assert_eq!(req.method, "DELETE");
    assert_eq!(req.path, format!("/api/v1/templates/{TEMPLATE_ID}"));
}

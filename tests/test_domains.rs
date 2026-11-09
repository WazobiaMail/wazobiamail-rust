mod common;
use common::MockServer;

#[tokio::test]
async fn test_list_domains() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!([
            {
                "id": "dom_1",
                "domain": "example.com",
                "isDefault": true,
                "verified": true,
                "status": "active",
                "createdAt": "2026-11-01T00:00:00Z"
            }
        ]),
    );

    let client = server.client();
    let res = client.domains().list().await.unwrap();

    assert_eq!(res.data.len(), 1);
    assert_eq!(res.data[0].domain, "example.com");
    assert!(res.data[0].verified);
    assert!(res.data[0].is_default);

    let req = server.last_request();
    assert_eq!(req.method, "GET");
    assert_eq!(req.path, "/api/v1/sending/domains");
}

#[tokio::test]
async fn test_create_and_delete_domain() {
    let server = MockServer::start().await;
    server.enqueue_json(
        201,
        serde_json::json!({
            "id": "dom_new_9",
            "domain": "updates.example.com",
            "isDefault": false,
            "verified": false,
            "status": "pending"
        }),
    );
    server.enqueue_json(200, serde_json::Value::Null);

    let client = server.client();
    let created = client
        .domains()
        .create("updates.example.com")
        .await
        .unwrap();

    assert_eq!(created.data.id, "dom_new_9");
    assert_eq!(created.data.domain, "updates.example.com");

    let deleted = client.domains().delete("dom_new_9").await.unwrap();
    assert!(deleted.success);

    let reqs = server.requests();
    assert_eq!(reqs[0].path, "/api/v1/sending/domains");
    assert_eq!(reqs[1].path, "/api/v1/sending/domains/dom_new_9");
    assert_eq!(reqs[1].method, "DELETE");
}

#[tokio::test]
async fn test_verify_domain_dns() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!({
            "verified": true,
            "records": [
                {
                    "id": "dns_1",
                    "recordType": "TXT",
                    "host": "wazobia._domainkey.example.com",
                    "value": "v=DKIM1; k=rsa; p=MIGfMA0GCSqGSIb3...",
                    "ttl": 3600,
                    "status": "valid"
                }
            ]
        }),
    );

    let client = server.client();
    let res = client.domains().verify("dom_1").await.unwrap();

    assert!(res.data.verified);
    assert_eq!(res.data.records.len(), 1);
    assert_eq!(res.data.records[0].record_type, "TXT");

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/sending/domains/dom_1/verify");
}

#[tokio::test]
async fn test_cloudflare_integration() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!({
            "connected": true,
            "zoneId": "cf_zone_123",
            "autoSync": true
        }),
    );

    let client = server.client();
    let res = client
        .domains()
        .cloudflare("dom_1")
        .connect("tok_cf", "zone_cf")
        .await
        .unwrap();

    assert!(res.data.connected);
    assert_eq!(res.data.zone_id.as_deref(), Some("cf_zone_123"));

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/sending/domains/dom_1/cloudflare");
    assert_eq!(req.method, "POST");
}

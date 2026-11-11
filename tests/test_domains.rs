use serde_json::json;
use wazobiamail::models::domains::UpdateDomainRequest;

mod common;
use common::MockServer;

const DOMAIN_ID: &str = "d4mN8xQ2tR6pL1wZ9aBc";

fn domain_fixture() -> serde_json::Value {
    json!({
        "id": DOMAIN_ID,
        "accountId": 42,
        "domain": "mail.brand.com",
        "requestedDomain": "brand.com",
        "verificationStatus": "verified",
        "isDefault": true,
        "domainRole": "primary",
        "parentDomainId": null,
        "createDefaultInboxes": true,
        "mailMode": "hosted",
        "createdAt": "2026-11-01T00:00:00"
    })
}

#[tokio::test]
async fn test_list_domains_uses_items_envelope_and_real_fields() {
    let server = MockServer::start().await;
    server.enqueue_json(200, json!({ "items": [domain_fixture()] }));

    let client = server.client();
    let res = client.domains().list().await.unwrap();

    assert_eq!(res.data.items.len(), 1);
    let domain = &res.data.items[0];
    assert_eq!(domain.domain, "mail.brand.com");
    assert_eq!(domain.verification_status, "verified");
    assert!(domain.is_verified());
    assert!(domain.is_default);
    assert_eq!(domain.mail_mode, "hosted");

    let req = server.last_request();
    assert_eq!(req.method, "GET");
    assert_eq!(req.path, "/api/v1/sending/domains");
}

#[tokio::test]
async fn test_create_update_and_delete_domain() {
    let server = MockServer::start().await;
    let mut updated = domain_fixture();
    updated["isDefault"] = json!(false);
    server.enqueue_json(201, json!({ "domain": domain_fixture() }));
    server.enqueue_json(200, json!({ "domain": updated }));
    server.enqueue_json(200, json!({ "domainId": DOMAIN_ID }));

    let client = server.client();
    let created = client.domains().create("mail.brand.com").await.unwrap();
    assert_eq!(created.data.domain.id, DOMAIN_ID);
    assert_eq!(created.data.domain.domain, "mail.brand.com");

    let updated = client
        .domains()
        .update(
            DOMAIN_ID,
            UpdateDomainRequest {
                is_default: Some(false),
                ..UpdateDomainRequest::default()
            },
        )
        .await
        .unwrap();
    assert!(!updated.data.domain.is_default);

    let deleted = client.domains().delete(DOMAIN_ID).await.unwrap();
    assert_eq!(deleted.data.domain_id, DOMAIN_ID);

    let reqs = server.requests();
    assert_eq!(reqs[0].path, "/api/v1/sending/domains");
    assert_eq!(reqs[0].method, "POST");
    let body: serde_json::Value = reqs[0].json().unwrap();
    assert_eq!(body, json!({ "domain": "mail.brand.com" }));

    assert_eq!(reqs[1].path, format!("/api/v1/sending/domains/{DOMAIN_ID}"));
    assert_eq!(reqs[1].method, "PATCH");
    let body: serde_json::Value = reqs[1].json().unwrap();
    assert_eq!(body, json!({ "isDefault": false }));

    assert_eq!(reqs[2].path, format!("/api/v1/sending/domains/{DOMAIN_ID}"));
    assert_eq!(reqs[2].method, "DELETE");
}

#[tokio::test]
async fn test_verify_domain_posts_force_body_and_reads_verification_envelope() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "verification": {
                "domainId": DOMAIN_ID,
                "domain": "mail.brand.com",
                "verificationStatus": "verified",
                "verifiedCount": 4,
                "pendingCount": 0,
                "checks": [
                    {
                        "recordType": "TXT",
                        "host": "wzbm._domainkey.mail.brand.com",
                        "expectedValue": "v=DKIM1; k=rsa; p=MIGfMA0GCSqGSIb3...",
                        "matched": true,
                        "observedValues": ["v=DKIM1; k=rsa; p=MIGfMA0GCSqGSIb3..."]
                    }
                ]
            }
        }),
    );

    let client = server.client();
    let res = client.domains().verify(DOMAIN_ID).await.unwrap();

    assert_eq!(res.data.verification.verification_status, "verified");
    assert_eq!(res.data.verification.verified_count, 4);
    assert_eq!(res.data.verification.checks.len(), 1);
    assert!(res.data.verification.checks[0].matched);

    let req = server.last_request();
    assert_eq!(
        req.path,
        format!("/api/v1/sending/domains/{DOMAIN_ID}/verify")
    );
    assert_eq!(req.method, "POST");
    let body: serde_json::Value = req.json().unwrap();
    assert_eq!(body, json!({}));
}

#[tokio::test]
async fn test_domain_dns_records_route_and_envelope() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "domain": {
                "provisioning": false,
                "domainId": DOMAIN_ID,
                "domain": "mail.brand.com",
                "verificationStatus": "verified",
                "records": [
                    {
                        "id": "r8cD2xQ6tP4mL1wZ9aBc",
                        "recordType": "TXT",
                        "host": "mail.brand.com",
                        "value": "v=spf1 include:_spf.wazobiamail.com ~all",
                        "ttl": 3600,
                        "status": "verified",
                        "lastCheckedAt": "2026-11-10T12:00:00"
                    }
                ],
                "copyBlock": "mail.brand.com. 3600 IN TXT \"v=spf1 include:_spf.wazobiamail.com ~all\""
            }
        }),
    );

    let client = server.client();
    let res = client.domains().get_dns_records(DOMAIN_ID).await.unwrap();

    assert_eq!(res.data.domain.domain, "mail.brand.com");
    assert_eq!(res.data.domain.records.len(), 1);
    assert_eq!(res.data.domain.records[0].record_type, "TXT");
    assert_eq!(res.data.domain.records[0].ttl, 3600);

    let req = server.last_request();
    assert_eq!(
        req.path,
        format!("/api/v1/sending/domains/{DOMAIN_ID}/dns-records")
    );
}

#[tokio::test]
async fn test_cloudflare_account_level_routes() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "configured": true,
            "managedEligible": true,
            "oauthConfigured": true,
            "domainConnectAvailable": true,
            "connectionSource": "customerOAuth",
            "connected": true,
            "needsReconnect": false,
            "email": "owner@brand.com",
            "scopes": "zone:read dns:edit"
        }),
    );
    server.enqueue_json(
        200,
        json!({
            "authorizationUrl": "https://dash.cloudflare.com/oauth2/auth?client_id=wzbm&state=abc"
        }),
    );
    server.enqueue_json(
        200,
        json!({
            "zone": "brand.com",
            "added": 5,
            "updated": 1,
            "records": 6,
            "verificationStatus": "verified",
            "receivingReady": true,
            "dmarcReady": true,
            "verifiedCount": 6,
            "pendingCount": 0
        }),
    );
    server.enqueue_json(200, json!({}));

    let client = server.client();
    let cloudflare = client.domains().cloudflare();

    let status = cloudflare.status(Some(DOMAIN_ID)).await.unwrap();
    assert!(status.data.connected);
    assert_eq!(
        status.data.connection_source.as_deref(),
        Some("customerOAuth")
    );
    assert_eq!(status.data.scopes, "zone:read dns:edit");

    let connect = cloudflare.connect(Some(DOMAIN_ID)).await.unwrap();
    assert!(connect
        .data
        .authorization_url
        .contains("dash.cloudflare.com"));

    let sync = cloudflare.sync(DOMAIN_ID).await.unwrap();
    assert_eq!(sync.data.zone, "brand.com");
    assert_eq!(sync.data.added, 5);
    assert_eq!(sync.data.verification_status, "verified");

    cloudflare.disconnect(Some(DOMAIN_ID)).await.unwrap();

    let reqs = server.requests();
    assert_eq!(
        reqs[0].path,
        format!("/api/v1/sending/domains/cloudflare/status?domainId={DOMAIN_ID}")
    );
    assert_eq!(reqs[0].method, "GET");
    assert_eq!(
        reqs[1].path,
        format!("/api/v1/sending/domains/cloudflare/connect?domainId={DOMAIN_ID}")
    );
    assert_eq!(reqs[1].method, "GET");
    assert_eq!(reqs[2].path, "/api/v1/sending/domains/cloudflare/sync");
    assert_eq!(reqs[2].method, "POST");
    let body: serde_json::Value = reqs[2].json().unwrap();
    assert_eq!(body, json!({ "domainId": DOMAIN_ID }));
    assert_eq!(
        reqs[3].path,
        format!("/api/v1/sending/domains/cloudflare/disconnect?domainId={DOMAIN_ID}")
    );
    assert_eq!(reqs[3].method, "DELETE");
}

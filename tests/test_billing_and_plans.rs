use serde_json::json;
use wazobiamail::models::Currency;

mod common;
use common::MockServer;

const INVOICE_ID: &str = "v1nM4xQ8tR2pL6wZ9aBc";
const PACK_ID: &str = "p4cK8xQ2tR6pL1wZ9aBc";

fn invoice_fixture() -> serde_json::Value {
    json!({
        "id": INVOICE_ID,
        "accountId": 42,
        "amount": 49.0,
        "currency": "USD",
        "status": "paid",
        "purpose": "subscription",
        "metadata": {
            "description": "Business monthly plan",
            "planId": "business",
            "billingInterval": "monthly"
        },
        "dueDate": "2026-11-02T00:00:00",
        "paidAt": "2026-11-01T10:30:00",
        "createdAt": "2026-11-01T10:00:00"
    })
}

#[tokio::test]
async fn test_list_invoices_returns_collection_page() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "items": [invoice_fixture()],
            "pagination": {
                "page": 1,
                "pageSize": 1,
                "totalItems": 1,
                "totalPages": 1,
                "hasNext": false,
                "hasPrev": false
            }
        }),
    );

    let client = server.client();
    let res = client.billing().list_invoices().await.unwrap();

    assert_eq!(res.data.items.len(), 1);
    assert_eq!(res.data.items[0].purpose, "subscription");
    assert_eq!(res.data.items[0].amount, 49.0);
    let pagination = res.data.pagination.unwrap();
    assert_eq!(pagination.page, 1);
    assert_eq!(pagination.page_size, 1);
    assert_eq!(pagination.total_items, 1);
    assert!(!pagination.has_next);

    let req = server.last_request();
    assert_eq!(req.method, "GET");
    assert_eq!(req.path, "/api/v1/billing/invoices");
}

#[tokio::test]
async fn test_get_invoice_returns_invoice_envelope() {
    let server = MockServer::start().await;
    server.enqueue_json(200, json!({ "invoice": invoice_fixture() }));

    let client = server.client();
    let res = client.billing().get_invoice(INVOICE_ID).await.unwrap();

    assert_eq!(res.data.invoice.id, INVOICE_ID);
    assert_eq!(res.data.invoice.currency, "USD");

    let req = server.last_request();
    assert_eq!(req.path, format!("/api/v1/billing/invoices/{INVOICE_ID}"));
}

#[tokio::test]
async fn test_download_invoice_pdf() {
    let server = MockServer::start().await;
    let fake_pdf_bytes = b"%PDF-1.4\n1 0 obj\n<<>>\nendobj\ntrailer\n<<>>\n%%EOF".to_vec();
    server.enqueue_bytes(200, "application/pdf", fake_pdf_bytes.clone());

    let client = server.client();
    let pdf = client.billing().download_invoice(INVOICE_ID).await.unwrap();

    assert_eq!(pdf, fake_pdf_bytes);

    let req = server.last_request();
    assert_eq!(
        req.path,
        format!("/api/v1/billing/invoices/{INVOICE_ID}/download")
    );
}

#[tokio::test]
async fn test_current_subscription_route_and_shape() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "subscription": {
                "id": "s7bS2xQ8tR4pL1wZ9aBc",
                "accountId": 42,
                "planId": "business",
                "status": "active",
                "startDate": "2026-11-01T00:00:00",
                "endDate": "2026-12-01T00:00:00",
                "autoRenew": true,
                "billingInterval": "monthly",
                "plan": {
                    "id": "business",
                    "name": "Business",
                    "usdPrice": 49.0,
                    "nairaPrice": 75000.0,
                    "monthlyQuota": 100000
                }
            }
        }),
    );
    server.enqueue_json(200, json!({ "subscription": null }));

    let client = server.client();
    let res = client.billing().get_subscription().await.unwrap();
    let subscription = res.data.subscription.unwrap();
    assert_eq!(subscription.plan_id, "business");
    assert_eq!(subscription.billing_interval, "monthly");
    assert_eq!(subscription.plan.as_ref().unwrap().monthly_quota, 100000);

    let empty = client.billing().get_subscription().await.unwrap();
    assert!(empty.data.subscription.is_none());

    let reqs = server.requests();
    assert_eq!(reqs[0].path, "/api/v1/subscriptions/current");
    assert_eq!(reqs[1].path, "/api/v1/subscriptions/current");
}

#[tokio::test]
async fn test_units_summary_packs_purchase_and_ledger() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "includedAllowance": 100000,
            "includedUsed": 2500,
            "includedRemaining": 97500,
            "purchasedBalance": 5000,
            "unlimited": false,
            "canBuyCredits": true
        }),
    );
    server.enqueue_json(
        200,
        json!({
            "items": [
                {
                    "id": PACK_ID,
                    "name": "10,000 email credits",
                    "description": "Add 10,000 email credits",
                    "units": 10000,
                    "priceNgnMinor": 200000,
                    "priceUsdMinor": 300,
                    "isActive": true,
                    "sortOrder": 1
                }
            ]
        }),
    );
    server.enqueue_json(
        200,
        json!({
            "invoiceId": INVOICE_ID,
            "units": 10000,
            "amountMinor": 200000,
            "creditAmountMinor": 200000,
            "serviceAmountMinor": 0,
            "currency": "NGN",
            "status": "pending_payment"
        }),
    );
    server.enqueue_json(
        200,
        json!({
            "items": [
                {
                    "id": "l9gR4xQ8tP2mL6wZ1aBc",
                    "accountId": 42,
                    "email": "owner@example.com",
                    "quantity": 10000,
                    "balanceAfter": 15000,
                    "source": "purchase",
                    "reference": INVOICE_ID,
                    "actorId": null,
                    "reason": "Verified email unit purchase",
                    "createdAt": "2026-11-10T12:00:00"
                }
            ],
            "pagination": {
                "page": 1,
                "pageSize": 25,
                "totalItems": 1,
                "totalPages": 1
            }
        }),
    );

    let client = server.client();

    let summary = client.units().get_summary().await.unwrap();
    assert_eq!(summary.data.included_allowance, 100000);
    assert_eq!(summary.data.included_remaining, 97500);
    assert_eq!(summary.data.purchased_balance, 5000);
    assert!(summary.data.can_buy_credits);

    let packs = client.units().list_packs().await.unwrap();
    assert_eq!(packs.data.items.len(), 1);
    assert_eq!(packs.data.items[0].units, 10000);
    assert_eq!(packs.data.items[0].price_ngn_minor, 200000);

    let purchase = client
        .units()
        .purchase(PACK_ID, Currency::Ngn)
        .await
        .unwrap();
    assert_eq!(purchase.data.invoice_id, INVOICE_ID);
    assert_eq!(purchase.data.status, "pending_payment");
    assert_eq!(purchase.data.currency, "NGN");

    let ledger = client.units().ledger().await.unwrap();
    assert_eq!(ledger.data.items.len(), 1);
    assert_eq!(ledger.data.items[0].quantity, 10000);
    assert_eq!(ledger.data.items[0].source, "purchase");
    assert_eq!(ledger.data.pagination.as_ref().unwrap().page_size, 25);

    let reqs = server.requests();
    assert_eq!(reqs[0].path, "/api/v1/units/summary");
    assert_eq!(reqs[1].path, "/api/v1/units/packs");
    assert_eq!(reqs[2].path, "/api/v1/units/purchases");
    assert_eq!(reqs[2].method, "POST");
    assert_eq!(reqs[3].path, "/api/v1/units/ledger");

    let body: serde_json::Value = reqs[2].json().unwrap();
    assert_eq!(body, json!({ "packId": PACK_ID, "currency": "NGN" }));
}

#[tokio::test]
async fn test_ledger_page_query_params() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "items": [],
            "pagination": {
                "page": 2,
                "pageSize": 10,
                "totalItems": 11,
                "totalPages": 2
            }
        }),
    );

    let client = server.client();
    let res = client.units().ledger_page(2, 10).await.unwrap();
    assert_eq!(res.data.items.len(), 0);

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/units/ledger?page=2&per_page=10");
}

#[tokio::test]
async fn test_plans_collection_and_effective_limits() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "items": [
                {
                    "id": "business",
                    "name": "Business",
                    "usdPrice": 49.0,
                    "nairaPrice": 75000.0,
                    "yearlyUsdPrice": 490.0,
                    "yearlyNairaPrice": 750000.0,
                    "intervalPrices": { "monthly": 49.0, "yearly": 490.0 },
                    "hourlyQuota": 1000,
                    "dailyQuota": 10000,
                    "monthlyQuota": 100000,
                    "sendRateLimitPerMinute": 120,
                    "dataRetentionDays": 90,
                    "features": { "customDomain": true },
                    "limits": { "domains": 10, "mailboxes": 25, "product": "wazobiamail" },
                    "overageUsdPer1000": 0.4,
                    "dedicatedIpAddonUsd": 20.0,
                    "status": "active"
                }
            ],
            "pagination": {
                "page": 1,
                "pageSize": 1,
                "totalItems": 1,
                "totalPages": 1,
                "hasNext": false,
                "hasPrev": false
            }
        }),
    );
    server.enqueue_json(
        200,
        json!({
            "limits": {
                "accountId": 42,
                "planId": "business",
                "sendRateLimitPerMinute": 120,
                "mailboxOpsPerMinute": 60,
                "mailboxLimit": 50,
                "baseMailboxLimit": 25,
                "additionalMailboxLimit": 25,
                "customDomainSendingEnabled": true,
                "domainLimit": 10,
                "receivingEnabled": true,
                "mailboxStorageGb": 50
            }
        }),
    );

    let client = server.client();
    let plans = client.plans().list().await.unwrap();
    assert_eq!(plans.data.items.len(), 1);
    assert_eq!(plans.data.items[0].monthly_quota, 100000);
    assert_eq!(plans.data.items[0].limits["domains"], 10);

    let effective = client.plans().get_effective_limits().await.unwrap();
    assert_eq!(effective.data.limits.plan_id, "business");
    assert_eq!(effective.data.limits.domain_limit, 10);
    assert_eq!(effective.data.limits.mailbox_storage_gb, 50);
    assert!(effective.data.limits.custom_domain_sending_enabled);

    let reqs = server.requests();
    assert_eq!(reqs[0].path, "/api/v1/plans");
    assert_eq!(reqs[1].path, "/api/v1/plans/effective");
}

#[tokio::test]
async fn test_status_check_returns_plan_and_units_blocks() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        json!({
            "plan": {
                "id": "business",
                "name": "Business",
                "status": "active",
                "billingInterval": "monthly",
                "startDate": "2026-11-01T00:00:00",
                "endDate": "2026-12-01T00:00:00",
                "autoRenew": true
            },
            "units": {
                "monthlyAllocation": 100000,
                "subscriptionUsed": 2500,
                "subscriptionRemaining": 97500,
                "purchasedBalance": 5000,
                "totalRemaining": 102500,
                "unlimited": false
            }
        }),
    );

    let client = server.client();
    let res = client.status().check().await.unwrap();

    assert_eq!(res.data.plan.name, "Business");
    assert_eq!(res.data.plan.billing_interval, "monthly");
    assert_eq!(res.data.units.monthly_allocation, 100000);
    assert_eq!(res.data.units.total_remaining, 102500);

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/status");
}

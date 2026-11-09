mod common;
use common::MockServer;

#[tokio::test]
async fn test_list_invoices() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!([
            {
                "id": "inv_001",
                "number": "INV-2026-001",
                "amount": 49.0,
                "currency": "USD",
                "status": "paid",
                "createdAt": "2026-11-01T00:00:00Z"
            }
        ]),
    );

    let client = server.client();
    let res = client.billing().list_invoices().await.unwrap();

    assert_eq!(res.data.len(), 1);
    assert_eq!(res.data[0].number, "INV-2026-001");
    assert_eq!(res.data[0].amount, 49.0);

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/billing/invoices");
}

#[tokio::test]
async fn test_download_invoice_pdf() {
    let server = MockServer::start().await;
    let fake_pdf_bytes = b"%PDF-1.4\n1 0 obj\n<<>>\nendobj\ntrailer\n<<>>\n%%EOF".to_vec();
    server.enqueue_bytes(200, "application/pdf", fake_pdf_bytes.clone());

    let client = server.client();
    let pdf = client.billing().download_invoice("inv_001").await.unwrap();

    assert_eq!(pdf, fake_pdf_bytes);

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/billing/invoices/inv_001/download");
}

#[tokio::test]
async fn test_units_summary_and_purchase() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!({
            "balance": 25000,
            "reserved": 0,
            "currency": "USD"
        }),
    );
    server.enqueue_json(
        200,
        serde_json::json!({
            "balance": 35000,
            "reserved": 0,
            "currency": "USD"
        }),
    );

    let client = server.client();
    let summary = client.units().get_summary().await.unwrap();
    assert_eq!(summary.data.balance, 25000);

    let purchased = client.units().purchase("pack_10k").await.unwrap();
    assert_eq!(purchased.data.balance, 35000);

    let reqs = server.requests();
    assert_eq!(reqs[0].path, "/api/v1/billing/units");
    assert_eq!(reqs[1].path, "/api/v1/billing/units/purchase");
    assert_eq!(reqs[1].method, "POST");
}

#[tokio::test]
async fn test_plans_effective_limits() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!({
            "planName": "Enterprise",
            "emailLimit": 1000000,
            "domainLimit": 50,
            "mailboxLimit": 200
        }),
    );

    let client = server.client();
    let res = client.plans().get_effective_limits().await.unwrap();

    assert_eq!(res.data.plan_name, "Enterprise");
    assert_eq!(res.data.email_limit, 1000000);
    assert_eq!(res.data.domain_limit, 50);
    assert_eq!(res.data.mailbox_limit, 200);

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/plans/effective");
}

#[tokio::test]
async fn test_status_check() {
    let server = MockServer::start().await;
    server.enqueue_json(
        200,
        serde_json::json!({
            "status": "operational",
            "accountId": "acc_789"
        }),
    );

    let client = server.client();
    let res = client.status().check().await.unwrap();

    assert_eq!(res.data.status, "operational");
    assert_eq!(res.data.account_id.as_deref(), Some("acc_789"));

    let req = server.last_request();
    assert_eq!(req.path, "/api/v1/status");
}

# WazobiaMail Rust SDK

[![crates.io](https://img.shields.io/crates/v/wazobiamail.svg)](https://crates.io/crates/wazobiamail)

**Start free with 3,000 emails/month.** WazobiaMail is a Resend alternative for transactional email through an API or SMTP, with sending domains, hosted mailboxes and deliverability tools.

Official, asynchronous, strongly typed Rust SDK for the [WazobiaMail](https://wazobiamail.com) SaaS platform.

Provides seamless integration for transactional email delivery, deliverability analysis, domain DNS verification, hosted mailboxes, external SMTP relays, and enterprise headless inboxes.

> 📚 **Official Platform Documentation**: Visit [docs.wazobiamail.com](https://docs.wazobiamail.com) for exhaustive API references, architecture guides, DNS setup walkthroughs, and deliverability guidance.

---

## Features

- **Asynchronous & Non-blocking**: Powered by `reqwest` and `tokio`.
- **100% Safe Rust**: `#![forbid(unsafe_code)]` with zero raw pointers.
- **Strongly Typed**: Comprehensive request builders and response models via `serde`.
- **Security-First**: Secret masking in `Debug` implementations to avoid leaking API keys in logs.
- **Enterprise Ready**: Full support for headless inbox messages, attachments, tracking, and idempotency keys.

---

## Installation

```bash
cargo add wazobiamail
cargo add tokio --features full
```

Or add it to `Cargo.toml` manually:

```toml
[dependencies]
wazobiamail = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

---

## Quick Start

```rust,no_run
use wazobiamail::{WazobiaMail, models::SendMessageRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reads WAZOBIA_API_KEY or WAZOBIAMAIL_API_KEY automatically from environment
    let client = WazobiaMail::from_env()?;

    let response = client
        .send(
            SendMessageRequest::builder()
                .from("Acme Store <orders@store.example.com>")
                .to("customer@example.com")
                .subject("Your Order #1042 has shipped!")
                .html("<h1>Order Update</h1><p>Your package is on its way!</p>")
                .text("Order Update: Your package is on its way!")
                .build()?,
        )
        .await?;

    println!("Queued tracking ID: {}", response.data.result.tracking_id);
    Ok(())
}
```

---

## Configuration & Builder

Initialize explicitly with an API key and custom parameters:

```rust,no_run
use std::time::Duration;
use wazobiamail::WazobiaMail;

# fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = WazobiaMail::builder()
    .api_key("wzbm_live_...")
    .base_url("https://app.wazobiamail.com/api/v1")
    .timeout(Duration::from_secs(45))
    .build()?;
# Ok(())
# }
```

---

## Core Capabilities

### 1. Direct Multipart Attachments

```rust,no_run
use wazobiamail::models::{AttachmentFile, SendMessageRequest};

# async fn example(client: &wazobiamail::WazobiaMail) -> Result<(), Box<dyn std::error::Error>> {
let request = SendMessageRequest::builder()
    .from("billing@store.com")
    .to("customer@example.com")
    .subject("Monthly Invoice")
    .html("<p>Attached is your invoice PDF.</p>")
    .build()?;

let files = vec![
    AttachmentFile::from_bytes("invoice.pdf", b"%PDF-1.4...", "application/pdf"),
];

let result = client.messages().send_with_attachments(request, files).await?;
# Ok(())
# }
```

### 2. Deliverability & Spam Analysis

```rust,no_run
use wazobiamail::models::SpamCheckRequest;

# async fn example(client: &wazobiamail::WazobiaMail) -> Result<(), Box<dyn std::error::Error>> {
let result = client.messages().spam_check(
    SpamCheckRequest::builder()
        .subject("Claim your reward")
        .html("<p>Congratulations, you won!</p>")
        .build()?
).await?;

println!("Deliverability Score: {}/100 ({})", result.data.deliverability_score, result.data.rating);
for rule in result.data.rules {
    println!(" - [{}] {}: {} (+{} points)", rule.code, rule.title, rule.description, rule.points);
}
# Ok(())
# }
```

---

### 3. Delivery Tracking, Stats, Read Receipts & Click Receipts

WazobiaMail provides built-in, granular observability for every transactional email dispatched.

#### 1. Read Receipts (Open Tracking)
Every HTML email sent through WazobiaMail can automatically include an invisible, privacy-compliant 1x1 tracking pixel. When the recipient opens the email in their client (Apple Mail, Gmail, Outlook, etc.):
- An `email.opened` event is recorded.
- Captures exact timestamp, recipient IP geolocation, device type, and client user-agent.
- Tracks unique opens vs. total open counts to measure genuine recipient engagement.
- Queryable via `client.messages().get_tracking(tracking_id)`.

#### 2. Click Receipts (Link Tracking)
Links in your email content can be rewritten using your verified tracking domain (`email.mail.brand.com` or `trk.wazobiamail.com`):
- When a recipient clicks any link, a `link.clicked` receipt is created with microsecond precision.
- Records the original destination URL, recipient email, click timestamp, and click count.
- Enables fraud detection, user engagement scoring, and automated lifecycle triggers.

#### 3. Delivery Lifecycle & Timeline
Every message transitions through well-defined lifecycle states:
`queued` ➔ `sent` ➔ `delivered` ➔ `opened` ➔ `clicked` ➔ `bounced` / `complained`

```rust,no_run
# async fn example(client: &wazobiamail::WazobiaMail) -> Result<(), Box<dyn std::error::Error>> {
// Inspect single message receipts and timeline
let detail = client.messages().get_tracking("trk_123456789").await?;

let msg = &detail.data.message.message;
println!("Status: {}", msg.status);
println!("Created At: {:?}", msg.created_at);
println!("Open Count (Read Receipts): {}", msg.open_count);
println!("Click Count (Click Receipts): {}", msg.click_count);

for event in &detail.data.message.events {
    println!("[{:?}] {} - Recipient: {:?}", event.occurred_at, event.event_type, event.recipient_email);
    if let Some(url) = &event.click_url {
        println!("  Clicked URL: {}", url);
    }
}

// Query aggregate stats and delivery list
let stats = client.messages().list_tracking(Some(20), None, Some("delivered")).await?;
println!("Total tracked: {}", stats.data.summary.messages);
for item in stats.data.messages {
    println!("{}: {} (opens: {}, clicks: {})", item.tracking_id, item.status, item.open_count, item.click_count);
}
# Ok(())
# }
```

---

### 4. Sending Domains & DNS Verification

```rust,no_run
# async fn example(client: &wazobiamail::WazobiaMail) -> Result<(), Box<dyn std::error::Error>> {
// List all configured domains
let domains = client.domains().list().await?;

// Add a new sending domain
let new_domain = client.domains().create("mail.brand.com").await?;

// Verify DNS records
let verification = client.domains().verify(&new_domain.data.domain.id).await?;
println!("Verification status: {}", verification.data.verification.verification_status);
# Ok(())
# }
```

---

### 🌐 Subdomain vs. Apex / Root Domain Architecture & Edge Cases

When configuring custom domains for transactional sending, choosing between a **subdomain** (e.g. `mail.brand.com` or `send.brand.com`) and an **apex/root domain** (`brand.com`) is a critical architectural decision.

#### 1. Why Using a Subdomain is Recommended (Root Domain NOT Used)

In production environments, **we strongly recommend using a dedicated sending subdomain** rather than your root/apex domain:

- **Preserve Existing Business Email (Google Workspace / Microsoft 365):**
  If your root domain (`brand.com`) already receives corporate email via Google Workspace or Microsoft 365, adding MX records pointing to WazobiaMail at the root apex `@` will **break your existing company email inbox routing**. Using `mail.brand.com` isolates transactional email routing completely.
- **Isolate Sender Reputation:**
  Transactional emails (password resets, invoices, OTP codes) have high engagement and deliverability requirements. Sending from a subdomain (`mail.brand.com`) ensures that any accidental marketing spam complaints won't degrade your corporate employee email deliverability.
- **Avoid SPF 10-Lookup Limit Exhaustion (RFC 7208):**
  SPF records have a strict limit of 10 DNS lookups. Root domains often accumulate multiple SaaS providers (Google, Zendesk, Salesforce, HubSpot). A dedicated subdomain gives WazobiaMail its own clean, independent SPF record without lookup count contention.

#### 2. DNS Hostnames When Root Domain is NOT Used

When verifying a subdomain like `mail.brand.com`, ensure you configure your DNS provider (Cloudflare, Route53, Namecheap, GoDaddy) with the correct hostnames:

| Record Type | Host / Name (if domain is `mail.brand.com`) | Target / Value | Notes |
| :--- | :--- | :--- | :--- |
| **TXT (SPF)** | `mail` (or `mail.brand.com`) | `v=spf1 include:_spf.wazobiamail.com ~all` | **Do NOT** place on `@` root |
| **TXT (DKIM)** | `wzbm._domainkey.mail` | `k=rsa; p=MIGfMA0GCSqGSIb3...` | Selector is scoped to the subdomain |
| **MX (Hosted)**| `mail` (priority 10 / 20) | `mx1.wazobiamail.com` / `mx2.wazobiamail.com` | Only required if receiving mail or hosted inboxes |
| **CNAME (Tracking)** | `email.mail` | `trk.wazobiamail.com` | Used for custom branded click/open tracking |
| **TXT (DMARC)** | `_dmarc.mail` (or fallback to `_dmarc`) | `v=DMARC1; p=reject; rua=mailto:dmarc@...` | Resolvers fall back to apex `_dmarc` if omitted |

#### 3. Critical Edge Cases & Troubleshooting

##### ⚠️ Edge Case 1: `UnverifiedSenderDomain` (HTTP 422 / 403)
- If you verified `mail.brand.com` in WazobiaMail, your `from` address **must** match that subdomain:
  - ✅ **Valid**: `from: "Acme Billing <billing@mail.brand.com>"`
  - ❌ **Invalid**: `from: "Acme Billing <billing@brand.com>"` (Fails because apex `brand.com` is unverified)
- Attempting to send from an unverified parent domain will return `WazobiaError::Validation` or `WazobiaError::Forbidden`.

##### 💡 Edge Case 2: Reply-To Decoupling (Best Practice)
- You can send from the verified subdomain while routing customer replies to your corporate root domain mailbox:
  ```rust,no_run
  # use wazobiamail::models::SendMessageRequest;
  # async fn example(client: &wazobiamail::WazobiaMail) -> Result<(), Box<dyn std::error::Error>> {
  let response = client.send(
      SendMessageRequest::builder()
          .from("Acme Orders <orders@mail.brand.com>") // Verified sending subdomain
          .reply_to("support@brand.com")              // Your corporate inbox on Google/Outlook
          .to("customer@example.com")
          .subject("Your receipt")
          .html("<p>Thank you for your purchase!</p>")
          .build()?
  ).await?;
  # Ok(())
  # }
  ```
- This gives you the deliverability benefits of a dedicated sending subdomain while preserving user-friendly reply handling.

##### ☁️ Edge Case 3: Cloudflare Proxy Status (Grey Cloud vs. Orange Cloud)
- **Problem**: In Cloudflare DNS, leaving the proxy enabled (**Orange Cloud**) for mail or tracking records breaks SMTP traffic, DNS verification, and SSL certificate issuance.
- **Solution**: Always toggle Cloudflare records to **"DNS only" (Grey Cloud / Proxied: Off)** for all WazobiaMail SPF, DKIM, MX, and tracking records. If you use `client.domains().cloudflare().sync(domain_id)`, this is handled automatically.

##### ⏳ Edge Case 4: Negative DNS Caching (SOA Minimum TTL Delay)
- If you run `client.domains().verify(domain_id)` immediately after publishing DNS records before they propagate, recursive resolvers may cache the negative response (`NXDOMAIN` or empty record).
- Subsequent verification checks will continue to fail until the DNS TTL expires (often 5 to 60 minutes).
- **Tip**: Test propagation first with `dig TXT mail.brand.com` or `dig TXT wzbm._domainkey.mail.brand.com +trace` before triggering verification.

##### 🛡️ Edge Case 5: DMARC Subdomain Policy (`sp=`) Inheritance
- If your apex domain (`brand.com`) has a DMARC record with `sp=reject` or `sp=quarantine`:
  - Any subdomain (e.g. `mail.brand.com`) automatically inherits that rejection policy unless it defines its own `_dmarc.mail.brand.com` record.
  - If SPF or DKIM alignment fails on the subdomain, recipient mail servers (Gmail, Yahoo, Outlook) will reject or spam-folder your emails immediately. Ensure both SPF and DKIM pass before sending production traffic.

##### 📬 Edge Case 6: Headless Inboxes & Hosted Mailboxes on Subdomains
- When provisioning hosted mailboxes (`client.mailboxes().create(domain_id, "support", ...)`), the mailbox address is generated under the specific verified domain (e.g. `support@mail.brand.com`).
- Inbound receiving requires the MX records for that exact subdomain to be pointing to WazobiaMail. Do not point the root domain MX to WazobiaMail if you have existing email services there.

##### 🔁 Edge Case 7: Network Timeouts & Idempotency
- If an HTTP request times out due to transient network issues, the email may already have been queued by the API Gateway.
- Always dispatch critical transactional messages with `client.send_with_idempotency(request, "your-key")`. Retrying with the same key returns the existing tracking receipt without duplicate sending.

### 5. Hosted Mailboxes & Headless Inboxes

```rust,no_run
# async fn example(client: &wazobiamail::WazobiaMail) -> Result<(), Box<dyn std::error::Error>> {
// Provision a new mailbox
let mailbox = client.mailboxes().create(
    "dom_123",
    "support",
    Some("Customer Support Team"),
).await?;

// Read incoming emails programmatically (Enterprise Headless Inbox)
let inbox = client.mailboxes().messages().list(&mailbox.data.mailbox.id, None).await?;
for msg in inbox.data.items {
    let from = msg.from.first().map(|address| address.email.as_str()).unwrap_or("unknown");
    println!("From: {} | Subject: {:?}", from, msg.subject);
}
# Ok(())
# }
```

---

## Error Handling

Errors map into the typed `WazobiaError` enum:

```rust,no_run
use wazobiamail::WazobiaError;

# async fn example(client: &wazobiamail::WazobiaMail) {
match client.domains().get("invalid_id").await {
    Ok(domain) => println!("Found: {:?}", domain),
    Err(WazobiaError::NotFound(msg)) => eprintln!("Domain not found: {}", msg),
    Err(WazobiaError::Validation { message, errors }) => {
        eprintln!("Invalid input: {}", message);
        for (field, err) in errors {
            eprintln!(" - {}: {}", field, err);
        }
    }
    Err(WazobiaError::RateLimit { message, retry_after }) => {
        eprintln!("Rate limited: {}. Retry after: {:?}", message, retry_after);
    }
    Err(e) => eprintln!("Unexpected error: {}", e),
}
# }
```

---

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

## Reading spam-check scores

The spam check returns a penalty `score`, a `threshold`, an `isSpam` boolean and an inverse `deliverabilityScore` (0–100, higher is better).

| `score` | `rating` | `isSpam` | `deliverabilityScore` | Decision |
| --- | --- | --- | --- | --- |
| `0.0 – 1.9` | `low_risk` | `false` | `81 – 100` | **Okay to send.** Clean or only minor findings. |
| `2.0 – 4.9` | `moderate_risk` | `false` | `51 – 80` | **Review first.** Not blocked, but fix the highest-point rules before sending. |
| `5.0 +` | `high_risk` | `true` | `0 – 50` | **Do not send.** Fix the findings and re-run the check. |

`isSpam` is the authoritative decision: it is true when `score >= threshold`. Read `threshold` from the response instead of hard-coding `5.0`; the server can change it. A green score is not a delivery guarantee — domain authentication, sender reputation and recipient engagement still matter.

## Official WazobiaMail SDKs

WazobiaMail is a **Resend alternative** for transactional email through an **API or SMTP**, with sending domains, hosted mailboxes and deliverability tools. Start free with **3,000 emails/month**.

The SDKs below all use the WazobiaMail API; SMTP/legacy clients that need SMTP submission can use WazobiaMail's SMTP access from the dashboard. Choose the package that matches your stack:

| Platform | Install | Registry | Source |
| --- | --- | --- | --- |
| TypeScript / Node.js | `npm install wazobiamail` | [npm](https://www.npmjs.com/package/wazobiamail) | [GitHub](https://github.com/WazobiaMail/wazobiamail-node) |
| Vue 3 | `npm install wazobiamail-vue wazobiamail` | [npm](https://www.npmjs.com/package/wazobiamail-vue) | [GitHub](https://github.com/WazobiaMail/wazobiamail-vue) |
| Nuxt 3 & 4 | `npm install wazobiamail-nuxt` | [npm](https://www.npmjs.com/package/wazobiamail-nuxt) | [GitHub](https://github.com/WazobiaMail/wazobiamail-nuxt) |
| Python | `pip install wazobiamail` | [PyPI](https://pypi.org/project/wazobiamail/) | [GitHub](https://github.com/WazobiaMail/wazobiamail-python) |
| PHP | `composer require wazobiamail/wazobiamail-php` | [Packagist](https://packagist.org/packages/wazobiamail/wazobiamail-php) | [GitHub](https://github.com/WazobiaMail/wazobiamail-php) |
| Laravel | `composer require wazobiamail/wazobiamail-laravel` | [Packagist](https://packagist.org/packages/wazobiamail/wazobiamail-laravel) | [GitHub](https://github.com/WazobiaMail/wazobiamail-laravel) |
| Rust | `cargo add wazobiamail` | [crates.io](https://crates.io/crates/wazobiamail) | [GitHub](https://github.com/WazobiaMail/wazobiamail-rust) |
| Go | `go get github.com/WazobiaMail/wazobiamail-go@v0.1.0` | [pkg.go.dev](https://pkg.go.dev/github.com/WazobiaMail/wazobiamail-go) | [GitHub](https://github.com/WazobiaMail/wazobiamail-go) |
| WordPress | Upload the plugin ZIP | [Packagist](https://packagist.org/packages/wazobiamail/wazobiamail-wordpress) · [GitHub releases](https://github.com/WazobiaMail/wazobiamail-wordpress/releases) | [GitHub](https://github.com/WazobiaMail/wazobiamail-wordpress) |

### What each package gives you

- **TypeScript / Node.js (`wazobiamail`)** — A zero-production-dependency typed client for Node.js 18+, Bun, Deno and edge runtimes. It ships ESM and CommonJS builds, typed errors, request builders, URL/multipart attachments, and resources for messages, domains, mailboxes, API keys, billing, units, plans, tracking, spam checks and support.
- **Vue 3 (`wazobiamail-vue`)** — A Vue plugin plus reactive composables: `useSendMail` for loading/success/error state, `useMailbox` for paginated headless inboxes with polling and optimistic delete, `useDomains` for domain/DNS workflows, and `useSpamCheck` for pre-flight deliverability checks.
- **Nuxt 3 & 4 (`wazobiamail-nuxt`)** — Keeps `runtimeConfig.wazobiamail.apiKey` on the Nitro server, adds the auto-imported `serverWazobiaClient(event)` helper, and auto-imports the Vue composables. Ideal for Nuxt apps that must not expose an API key to the browser bundle.
- **Python (`wazobiamail`)** — A synchronous, PEP 561-compliant client with **zero runtime dependencies** (standard library only), dataclass models and typed responses. Works in Django (sync views or `sync_to_async`), FastAPI (`def` endpoints or `run_in_threadpool`), Flask, scripts and workers. Includes webhook HMAC verification.
- **PHP (`wazobiamail/wazobiamail-php`)** — A Composer/Guzzle 7 client for PHP 8.1+. It accepts ergonomic flat send payloads or the canonical nested `content` shape, supports URL attachments and automatic multipart upload for inline files, and maps API failures to typed exceptions.
- **Laravel (`wazobiamail/wazobiamail-laravel`)** — Registers a native `wazobiamail` mail driver, service provider and `WazobiaMail` facade for Laravel 10–13. Existing `Mail`, Mailables and notifications keep working; the package also exposes domains, mailboxes and webhook verification.
- **Rust (`wazobiamail`)** — Async, strongly typed and `#![forbid(unsafe_code)]`, built on reqwest/tokio. Includes fluent request builders, typed response models, typed errors, secret masking in `Debug`, and resources for sending, domains, Cloudflare DNS, mailboxes, templates, tracking and billing.
- **Go (`github.com/WazobiaMail/wazobiamail-go`)** — Context-first, concurrency-safe and zero-dependency (standard library only). Typed request/response models, API-key or Bearer auth, multipart attachments, domains, mailboxes, billing, units, plans, support, templates and HMAC webhook verification.
- **WordPress (`wazobiamail-wordpress`)** — Routes every `wp_mail()` call through the WazobiaMail API without changing forms or plugins. It registers the site domain, displays SPF/DKIM/DMARC/MX records, verifies DNS, uploads attachments, and keeps recent delivery receipts in the admin screen.

### Documentation links

- [Official SDK guide and examples](https://docs.wazobiamail.com/docs/guides/sdks)
- [API reference](https://docs.wazobiamail.com/docs/api-reference)
- [Authentication and API keys](https://docs.wazobiamail.com/docs/guides/authentication)
- [Sending domains and DNS](https://docs.wazobiamail.com/docs/guides/domains)
- [Webhooks](https://docs.wazobiamail.com/docs/guides/webhooks)

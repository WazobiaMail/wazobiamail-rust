use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::config::{
    mask_secret, AuthScheme, ClientConfig, DEFAULT_BASE_URL, DEFAULT_TIMEOUT, DEFAULT_USER_AGENT,
};
use crate::errors::{Result, WazobiaError};
use crate::http::HttpClient;
use crate::models::common::ApiResponse;
use crate::models::messages::{SendEnvelope, SendMessageRequest};
use crate::resources::api_keys::ApiKeysResource;
use crate::resources::billing::{BillingResource, PlansResource, UnitsResource};
use crate::resources::domains::DomainsResource;
use crate::resources::mailboxes::MailboxesResource;
use crate::resources::messages::MessagesResource;
use crate::resources::smtp_relays::{SmtpAccessResource, SmtpRelaysResource};
use crate::resources::status::StatusResource;
use crate::resources::support::SupportResource;
use crate::resources::templates::TemplatesResource;

/// Primary client for interacting with the WazobiaMail SaaS API.
#[derive(Clone)]
pub struct WazobiaMail {
    http: Arc<HttpClient>,
    messages: MessagesResource,
    domains: DomainsResource,
    mailboxes: MailboxesResource,
    api_keys: ApiKeysResource,
    smtp_relays: SmtpRelaysResource,
    smtp_access: SmtpAccessResource,
    billing: BillingResource,
    units: UnitsResource,
    plans: PlansResource,
    status: StatusResource,
    support: SupportResource,
    templates: TemplatesResource,
}

impl WazobiaMail {
    /// Initialize a new client with an explicit API key.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }

    /// Initialize a new client by resolving the API key from environment variables
    /// (`WAZOBIA_API_KEY`, followed by `WAZOBIAMAIL_API_KEY`).
    pub fn from_env() -> Result<Self> {
        match ClientConfig::from_env() {
            Some(key) => Self::new(key),
            None => Err(WazobiaError::Authentication(
                "No API key found in WAZOBIA_API_KEY or WAZOBIAMAIL_API_KEY environment variables"
                    .to_string(),
            )),
        }
    }

    /// Create a fluent builder for configuring client options.
    pub fn builder() -> WazobiaMailBuilder {
        WazobiaMailBuilder::default()
    }

    /// Read resolved client configuration.
    pub fn config(&self) -> &ClientConfig {
        self.http.config()
    }

    /// Direct convenience shortcut for sending a transactional email.
    pub async fn send(&self, request: SendMessageRequest) -> Result<ApiResponse<SendEnvelope>> {
        self.messages.send(request).await
    }

    /// Direct convenience shortcut for sending a transactional email with an idempotency key.
    pub async fn send_with_idempotency(
        &self,
        request: SendMessageRequest,
        idempotency_key: &str,
    ) -> Result<ApiResponse<SendEnvelope>> {
        self.messages
            .send_with_idempotency(request, Some(idempotency_key))
            .await
    }

    // Resource Accessors
    pub fn messages(&self) -> &MessagesResource {
        &self.messages
    }

    /// Ergonomic alias for `messages()`.
    pub fn emails(&self) -> &MessagesResource {
        &self.messages
    }

    pub fn domains(&self) -> &DomainsResource {
        &self.domains
    }

    pub fn mailboxes(&self) -> &MailboxesResource {
        &self.mailboxes
    }

    pub fn api_keys(&self) -> &ApiKeysResource {
        &self.api_keys
    }

    pub fn smtp_relays(&self) -> &SmtpRelaysResource {
        &self.smtp_relays
    }

    pub fn smtp_access(&self) -> &SmtpAccessResource {
        &self.smtp_access
    }

    pub fn billing(&self) -> &BillingResource {
        &self.billing
    }

    pub fn units(&self) -> &UnitsResource {
        &self.units
    }

    pub fn plans(&self) -> &PlansResource {
        &self.plans
    }

    pub fn status(&self) -> &StatusResource {
        &self.status
    }

    pub fn support(&self) -> &SupportResource {
        &self.support
    }

    pub fn templates(&self) -> &TemplatesResource {
        &self.templates
    }
}

impl fmt::Debug for WazobiaMail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WazobiaMail")
            .field("base_url", &self.http.config().base_url)
            .field(
                "api_key",
                &mask_secret(self.http.config().api_key.as_deref()),
            )
            .field(
                "access_token",
                &mask_secret(self.http.config().access_token.as_deref()),
            )
            .field("timeout", &self.http.config().timeout)
            .finish()
    }
}

/// Fluent builder for the `WazobiaMail` client.
#[derive(Default)]
pub struct WazobiaMailBuilder {
    api_key: Option<String>,
    access_token: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    auth_scheme: Option<AuthScheme>,
    custom_headers: HeaderMap,
}

impl WazobiaMailBuilder {
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        let k = key.into().trim().to_string();
        if !k.is_empty() {
            self.api_key = Some(k);
        }
        self
    }

    pub fn access_token(mut self, token: impl Into<String>) -> Self {
        let t = token.into().trim().to_string();
        if !t.is_empty() {
            self.access_token = Some(t);
        }
        self
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        let u = url.into().trim().to_string();
        if !u.is_empty() {
            self.base_url = Some(u);
        }
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn auth_scheme(mut self, scheme: AuthScheme) -> Self {
        self.auth_scheme = Some(scheme);
        self
    }

    pub fn header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.custom_headers.insert(name, value);
        self
    }

    pub fn build(self) -> Result<WazobiaMail> {
        let api_key = self.api_key.or_else(ClientConfig::from_env);
        let access_token = self.access_token;

        if api_key.is_none() && access_token.is_none() {
            return Err(WazobiaError::Authentication(
                "An API key (or access token) is required to initialize WazobiaMail client"
                    .to_string(),
            ));
        }

        let base_url = self
            .base_url
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string())
            .trim_end_matches('/')
            .to_string();

        let timeout = self.timeout.unwrap_or(DEFAULT_TIMEOUT);
        let auth_scheme = self.auth_scheme.unwrap_or(AuthScheme::Header);

        let config = ClientConfig {
            api_key,
            access_token,
            base_url,
            timeout,
            auth_scheme,
            user_agent: DEFAULT_USER_AGENT.to_string(),
        };

        let http = Arc::new(HttpClient::new(config, self.custom_headers)?);

        Ok(WazobiaMail {
            messages: MessagesResource::new(Arc::clone(&http)),
            domains: DomainsResource::new(Arc::clone(&http)),
            mailboxes: MailboxesResource::new(Arc::clone(&http)),
            api_keys: ApiKeysResource::new(Arc::clone(&http)),
            smtp_relays: SmtpRelaysResource::new(Arc::clone(&http)),
            smtp_access: SmtpAccessResource::new(Arc::clone(&http)),
            billing: BillingResource::new(Arc::clone(&http)),
            units: UnitsResource::new(Arc::clone(&http)),
            plans: PlansResource::new(Arc::clone(&http)),
            status: StatusResource::new(Arc::clone(&http)),
            support: SupportResource::new(Arc::clone(&http)),
            templates: TemplatesResource::new(Arc::clone(&http)),
            http,
        })
    }
}

use std::env;
use std::fmt;
use std::time::Duration;

pub const DEFAULT_BASE_URL: &str = "https://app.wazobiamail.com/api/v1";
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
pub const SDK_VERSION: &str = "0.1.0";
pub const DEFAULT_USER_AGENT: &str = "wazobiamail-rust/0.1.0";

/// How credentials should be transmitted over HTTP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuthScheme {
    /// Transmit via `X-API-Key` header (standard for API keys).
    #[default]
    Header,
    /// Transmit via `Authorization: Bearer <token>` header.
    Bearer,
    /// Transmit via both `X-API-Key` and `Authorization: Bearer`.
    Both,
}

/// Helper function to safely mask secrets in logs and debug dumps.
pub fn mask_secret(value: Option<&str>) -> String {
    match value {
        None | Some("") => "***".to_string(),
        Some(s) if s.len() <= 8 => "***".to_string(),
        Some(s) => {
            let prefix = &s[..s.len().min(5)];
            let suffix = &s[s.len().saturating_sub(4)..];
            format!("{prefix}...{suffix}")
        }
    }
}

/// Resolved, immutable configuration for the WazobiaMail client.
#[derive(Clone)]
pub struct ClientConfig {
    pub api_key: Option<String>,
    pub access_token: Option<String>,
    pub base_url: String,
    pub timeout: Duration,
    pub auth_scheme: AuthScheme,
    pub user_agent: String,
}

impl fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientConfig")
            .field("api_key", &mask_secret(self.api_key.as_deref()))
            .field("access_token", &mask_secret(self.access_token.as_deref()))
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .field("auth_scheme", &self.auth_scheme)
            .field("user_agent", &self.user_agent)
            .finish()
    }
}

impl ClientConfig {
    /// Attempt to resolve configuration from standard environment variables.
    pub fn from_env() -> Option<String> {
        env::var("WAZOBIA_API_KEY")
            .ok()
            .and_then(|k| {
                if k.trim().is_empty() {
                    None
                } else {
                    Some(k.trim().to_string())
                }
            })
            .or_else(|| {
                env::var("WAZOBIAMAIL_API_KEY").ok().and_then(|k| {
                    if k.trim().is_empty() {
                        None
                    } else {
                        Some(k.trim().to_string())
                    }
                })
            })
    }
}

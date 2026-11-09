#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod client;
pub mod config;
pub mod errors;
pub mod http;
pub mod models;
pub mod resources;

pub use client::{WazobiaMail, WazobiaMailBuilder};
pub use config::{mask_secret, AuthScheme, ClientConfig};
pub use errors::{Result, WazobiaError};

/// Ergonomic prelude for importing primary client and common model types.
pub mod prelude {
    pub use crate::client::{WazobiaMail, WazobiaMailBuilder};
    pub use crate::errors::{Result, WazobiaError};
    pub use crate::models::common::ApiResponse;
    pub use crate::models::messages::{AttachmentFile, SendMessageRequest, UrlAttachment};
}

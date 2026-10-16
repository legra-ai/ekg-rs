#![cfg(not(target_family = "wasm"))]
//! Create a Hyper-specific TLS 1.3 connector.
//! This is very similar to ekg_aws_util::tls_connector, but it uses the
//! HttpConnector from the hyper_util crate rather than the aws_smithy_runtime
//! crate because we do not want the ekg-util crate to have a dependency on any
//! AWS SDK crates.
use {
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    hyper_util::client::legacy::connect::HttpConnector,
};

/// Create a TLS 1.3 connector to be used with Hyper, AWS SDK, etc.
pub async fn create() -> Result<HttpsConnector<HttpConnector>, ekg_error::Error> {
    tracing::debug!("Attempting to create a TLS 1.3 connector:");

    let tls_config = crate::tls_config::create().await?;

    // Finish setup of the rustls connector.
    let rustls_connector = HttpsConnectorBuilder::new()
        .with_tls_config(tls_config)
        .https_only()
        .enable_http2()
        .build();

    Ok(rustls_connector)
}

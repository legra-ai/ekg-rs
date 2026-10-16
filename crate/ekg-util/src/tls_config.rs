#![cfg(not(target_family = "wasm"))]

use rustls::{ALL_VERSIONS, ClientConfig, RootCertStore};

/// Create a TLS 1.3 config to be used with Hyper, AWS SDK, etc.
pub async fn create() -> Result<ClientConfig, ekg_error::Error> {
    tracing::debug!("Attempting to create a TLS 1.3 config:");

    // Let webpki load the Mozilla root certificates.
    let mut root_store = RootCertStore::empty();

    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    // The .with_protocol_versions call is where we set TLS1.3. You can add
    // rustls::version::TLS12 or replace them both with rustls::ALL_VERSIONS
    let config = ClientConfig::builder_with_protocol_versions(ALL_VERSIONS)
        .with_root_certificates(root_store)
        .with_no_client_auth();

    Ok(config)
}

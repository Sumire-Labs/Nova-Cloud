use yup_oauth2::ServiceAccountAuthenticator;
use google_drive3::{DriveHub, api::Permission};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

// A type alias for the DriveHub to make it easier to use.
pub type DriveHubType = DriveHub<HttpsConnector<HttpConnector>>;

/// Creates a new DriveHub instance for interacting with the Google Drive API
/// using a service account.
pub async fn create_drive_hub() -> DriveHubType {
    // Read the service account key from the file.
    let sa_key = yup_oauth2::read_service_account_key("config/google_client_secret.json")
        .await
        .expect("Failed to read service account key file");

    // Create an authenticator.
    let auth = ServiceAccountAuthenticator::builder(sa_key)
        .build()
        .await
        .expect("Failed to create service account authenticator");

    // Create the DriveHub.
    DriveHub::new(
        hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build()),
        auth,
    )
}


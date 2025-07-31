use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use google_drive3::{DriveHub, api::File};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use std::sync::Arc;

// A type alias for the DriveHub to make it easier to use.
pub type DriveHubType = DriveHub<HttpsConnector<HttpConnector>>;

/// Creates a new DriveHub instance for interacting with the Google Drive API.
pub async fn create_drive_hub() -> DriveHubType {
    // Read the client secret from the file.
    let secret = yup_oauth2::read_application_secret("config/google_client_secret.json")
        .await
        .expect("Failed to read client secret file");

    // Create an authenticator.
    let auth = InstalledFlowAuthenticator::builder(
        secret,
        InstalledFlowReturnMethod::Interactive,
    )
    .persist_tokens_to_disk("config/tokencache.json")
    .build()
    .await
    .expect("Failed to create authenticator");

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

/// Creates a new folder in Google Drive.
pub async fn create_folder(hub: &DriveHubType, folder_name: &str, parent_id: Option<&str>) -> Result<File, Box<dyn std::error::Error>> {
    let new_folder = File {
        name: Some(folder_name.to_string()),
        mime_type: Some("application/vnd.google-apps.folder".to_string()),
        parents: parent_id.map(|id| vec![id.to_string()]),
        ..
        Default::default()
    };

    let result = hub
        .files()
        .create(new_folder)
        .upload(std::io::empty(), "*/*".parse().unwrap())
        .await?;

    Ok(result.1)
}

use axum::{
    extract::State,
    http::{header, Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tower_http::cors::{Any, CorsLayer};

mod google_drive;
use google_drive::{create_drive_hub, DriveHubType};
use google_drive3::api::File;

// --- Data Structures ---

#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
}

// In-memory user database
type Db = Arc<Mutex<HashMap<String, String>>>;

// Application state
#[derive(Clone)]
struct AppState {
    db: Db,
    drive_hub: Arc<DriveHubType>,
}

// --- Main Application ---

#[tokio::main]
async fn main() {
    // Create the Google Drive hub using the service account.
    let drive_hub = Arc::new(create_drive_hub().await);

    // Create an in-memory database for users.
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    // Create the application state.
    let app_state = AppState {
        db,
        drive_hub,
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(app_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// --- Route Handlers ---

async fn root_handler() -> &'static str {
    "Hello, World!"
}

#[axum::debug_handler]
async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<User>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Step 1: Check if user exists (without holding the lock)
    {
        let db_lock = state.db.lock().unwrap();
        if db_lock.contains_key(&payload.username) {
            let response = serde_json::json!({ "message": "Username already exists" });
            return (StatusCode::CONFLICT, Json(response));
        }
    } // Lock is released here

    // Step 2: Perform the async operation (Google Drive API call)
    let new_folder = File {
        name: Some(payload.username.clone()),
        mime_type: Some("application/vnd.google-apps.folder".to_string()),
        // To place this folder inside a specific parent folder, add its ID here:
        // parents: Some(vec!["YOUR_PARENT_FOLDER_ID".to_string()]),
        ..
        Default::default()
    };

    let created_folder = match state.drive_hub.files().create(new_folder).upload(std::io::empty(), "*/*".parse().unwrap()).await {
        Ok((_response, folder)) => folder,
        Err(e) => {
            eprintln!("Failed to create folder for user '{}': {}", payload.username, e);
            let response = serde_json::json!({ "message": "Failed to create user folder in Google Drive. Check server logs." });
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    // Step 3: Lock the database again and insert the new user
    println!("Successfully created folder for user '{}': {}", payload.username, created_folder.name.unwrap_or_default());
    let mut db_lock = state.db.lock().unwrap();
    db_lock.insert(payload.username, payload.password);
    
    let response = serde_json::json!({ "message": "User registered successfully" });
    (StatusCode::CREATED, Json(response))
}

#[axum::debug_handler]
async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<User>,
) -> (StatusCode, Json<serde_json::Value>) {
    let db_lock = state.db.lock().unwrap();

    match db_lock.get(&payload.username) {
        Some(password) if *password == payload.password => {
            println!("User '{}' logged in successfully", payload.username);
            let response = serde_json::json!({ "message": "Login successful" });
            (StatusCode::OK, Json(response))
        }
        _ => {
            println!("Failed login attempt for user '{}'", payload.username);
            let response = serde_json::json!({ "message": "Invalid username or password" });
            (StatusCode::UNAUTHORIZED, Json(response))
        }
    }
}
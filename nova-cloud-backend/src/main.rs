use axum::{
    extract::{State, Path},
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

#[derive(Serialize, Debug, Clone)]
struct UserRecord {
    password: String,
    folder_id: String,
}

// In-memory user database
type Db = Arc<Mutex<HashMap<String, UserRecord>>>;

// Application state
#[derive(Clone)]
struct AppState {
    db: Db,
    drive_hub: Arc<DriveHubType>,
}

// --- Main Application ---

#[tokio::main]
async fn main() {
    let drive_hub = Arc::new(create_drive_hub().await);
    let db: Db = Arc::new(Mutex::new(HashMap::new()));
    let app_state = AppState { db, drive_hub };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        // Add the new route for listing files
        .route("/api/files/:username", get(list_files_handler))
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
    {
        let db_lock = state.db.lock().unwrap();
        if db_lock.contains_key(&payload.username) {
            return (StatusCode::CONFLICT, Json(serde_json::json!({ "message": "Username already exists" })));
        }
    }

    let new_folder = File {
        name: Some(payload.username.clone()),
        mime_type: Some("application/vnd.google-apps.folder".to_string()),
        parents: Some(vec!["1pFpk1JfsjwITQtfBJhGcCeAihzH1odD0".to_string()]),
        ..
        Default::default()
    };

    let created_folder = match state.drive_hub.files().create(new_folder).upload(std::io::empty(), "*/*".parse().unwrap()).await {
        Ok((_response, folder)) => folder,
        Err(e) => {
            eprintln!("Failed to create folder: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "message": "Failed to create user folder." })))
        }
    };

    let folder_id = created_folder.id.clone().unwrap_or_default();
    if folder_id.is_empty() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "message": "Failed to get folder ID from Google Drive." })))
    }

    println!("Created folder for '{}' with ID: {}", payload.username, folder_id);
    let mut db_lock = state.db.lock().unwrap();
    let user_record = UserRecord {
        password: payload.password,
        folder_id,
    };
    db_lock.insert(payload.username, user_record);
    
    (StatusCode::CREATED, Json(serde_json::json!({ "message": "User registered successfully" })))
}

#[axum::debug_handler]
async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<User>,
) -> (StatusCode, Json<serde_json::Value>) {
    let db_lock = state.db.lock().unwrap();
    match db_lock.get(&payload.username) {
        Some(record) if record.password == payload.password => {
            println!("User '{}' logged in successfully", payload.username);
            (StatusCode::OK, Json(serde_json::json!({ "message": "Login successful", "username": payload.username })))
        }
        _ => {
            (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "message": "Invalid username or password" })))
        }
    }
}

// --- New Handler for Listing Files ---
#[axum::debug_handler]
async fn list_files_handler(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Vec<File>>, StatusCode> {
    let folder_id = {
        let db_lock = state.db.lock().unwrap();
        match db_lock.get(&username) {
            Some(record) => record.folder_id.clone(),
            None => return Err(StatusCode::NOT_FOUND),
        }
    };

    let query = format!("'{}' in parents and trashed = false", folder_id);
    match state.drive_hub
        .files()
        .list()
        .q(&query)
        .param("fields", "files(id,name,mimeType,createdTime,modifiedTime,size,iconLink)")
        .doit()
        .await
    {
        Ok((_response, file_list)) => {
            Ok(Json(file_list.files.unwrap_or_default()))
        }
        Err(e) => {
            eprintln!("Failed to list files for user '{}': {}", username, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

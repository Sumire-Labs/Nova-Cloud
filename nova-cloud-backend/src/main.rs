use axum::{
    extract::{DefaultBodyLimit, Multipart, Path, State},
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
use google_drive3::api::{File, Permission};

const DB_PATH: &str = "database.json";
const UPLOAD_LIMIT_BYTES: usize = 100 * 1024 * 1024; // 100 MB

// --- Data Structures ---

#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserRecord {
    password: String,
    folder_id: String,
}

type Db = Arc<Mutex<HashMap<String, UserRecord>>>;

#[derive(Clone)]
struct AppState {
    db: Db,
    drive_hub: Arc<DriveHubType>,
}

// --- Database Persistence ---

async fn save_db_to_file(db: &Db) {
    let json_data = {
        let db_lock = db.lock().unwrap();
        match serde_json::to_string_pretty(&*db_lock) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("FATAL: Could not serialize database: {}", e);
                return;
            }
        }
    };

    if let Err(e) = tokio::fs::write(DB_PATH, json_data).await {
        eprintln!("FATAL: Could not write to database file '{}': {}", DB_PATH, e);
    }
}

async fn load_db_from_file() -> Db {
    match tokio::fs::read_to_string(DB_PATH).await {
        Ok(json_data) => match serde_json::from_str(&json_data) {
            Ok(map) => Arc::new(Mutex::new(map)),
            Err(e) => {
                eprintln!(
                    "WARN: Could not parse database file '{}', starting fresh. Error: {}",
                    DB_PATH, e
                );
                Arc::new(Mutex::new(HashMap::new()))
            }
        },
        Err(_) => {
            println!("INFO: No database file found, starting fresh.");
            Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

// --- Main Application ---

#[tokio::main]
async fn main() {
    let drive_hub = Arc::new(create_drive_hub().await);

    // Ensure the service account has writer permissions on the target folder.
    // This is crucial for allowing the service account to create files inside it.
    let permission = Permission {
        type_: Some("user".to_string()),
        role: Some("writer".to_string()),
        email_address: Some("nova-cloud-server@drive-467617.iam.gserviceaccount.com".to_string()),
        ..Default::default()
    };
    let _ = drive_hub.permissions().create("1pFpk1JfsjwITQtfBJhGcCeAihzH1odD0", permission)
        .supports_all_drives(true)
        .await;
        // We ignore the result, as this might fail if permission is already granted,
        // which is not a critical error for server startup.

    let db = load_db_from_file().await;
    let app_state = AppState { db, drive_hub };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/api/files/:username", get(list_files_handler))
        .route("/api/upload/:username", post(upload_file_handler))
        .layer(cors) // Apply CORS first
        .layer(DefaultBodyLimit::max(UPLOAD_LIMIT_BYTES))
        .with_state(app_state);

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
        if state.db.lock().unwrap().contains_key(&payload.username) {
            return (StatusCode::CONFLICT, Json(serde_json::json!({ "message": "Username already exists" })));
        }
    }

    let new_folder = File {
        name: Some(payload.username.clone()),
        mime_type: Some("application/vnd.google-apps.folder".to_string()),
        parents: Some(vec!["1pFpk1JfsjwITQtfBJhGcCeAihzH1odD0".to_string()]),
        ..Default::default()
    };

    let created_folder = match state.drive_hub.files().create(new_folder).upload(std::io::empty(), "*/*".parse().unwrap()).await {
        Ok((_, folder)) => folder,
        Err(e) => {
            eprintln!("Failed to create folder: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "message": "Failed to create user folder." })));
        }
    };

    let folder_id = created_folder.id.clone().unwrap_or_default();
    if folder_id.is_empty() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "message": "Failed to get folder ID." })));
    }

    let user_record = UserRecord { password: payload.password, folder_id };
    state.db.lock().unwrap().insert(payload.username, user_record);
    save_db_to_file(&state.db).await;

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
            (StatusCode::OK, Json(serde_json::json!({ "message": "Login successful", "username": payload.username })))
        }
        _ => (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "message": "Invalid username or password" }))),
    }
}

#[axum::debug_handler]
async fn list_files_handler(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Vec<File>>, StatusCode> {
    let folder_id = match state.db.lock().unwrap().get(&username) {
        Some(record) => record.folder_id.clone(),
        None => return Err(StatusCode::NOT_FOUND),
    };

    let query = format!("'{}' in parents and trashed = false", folder_id);
    match state.drive_hub.files().list().q(&query).param("fields", "files(id,name,mimeType,createdTime,modifiedTime,size,iconLink)").doit().await {
        Ok((_, file_list)) => Ok(Json(file_list.files.unwrap_or_default())),
        Err(e) => {
            eprintln!("Failed to list files for user '{}': {}", username, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
async fn upload_file_handler(
    State(state): State<AppState>,
    Path(username): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<File>, StatusCode> {
    let folder_id = match state.db.lock().unwrap().get(&username) {
        Some(record) => record.folder_id.clone(),
        None => return Err(StatusCode::NOT_FOUND),
    };

    while let Some(field) = multipart.next_field().await.map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)? {
        if field.name() == Some("file") {
            let file_name = field.file_name().unwrap_or("unknown_file").to_string();
            let data = field.bytes().await.map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Guess MIME type from file extension for robustness.
            let mime_type = mime_guess::from_path(&file_name)
                .first_or_octet_stream(); // Defaults to application/octet-stream

            let new_file_metadata = File {
                name: Some(file_name.clone()),
                parents: Some(vec![folder_id]),
                ..Default::default()
            };

            let cursor = std::io::Cursor::new(data);

            // Use `upload_resumable` with a guessed `Mime` type.
            // This is the definitive, correct approach.
            match state.drive_hub
                .files()
                .create(new_file_metadata)
                .supports_all_drives(true)
                .upload_resumable(cursor, mime_type)
                .await {
                Ok((_, uploaded_file)) => {
                    println!("Successfully uploaded file '{}' for user '{}'", uploaded_file.name.clone().unwrap_or_default(), username);
                    return Ok(Json(uploaded_file));
                },
                Err(e) => {
                    eprintln!("Failed to upload file for user '{}': {}", username, e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
    }

    Err(StatusCode::BAD_REQUEST)
}
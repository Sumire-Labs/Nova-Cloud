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

// --- Data Structures ---

#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
}

// In-memory user database
type Db = Arc<Mutex<HashMap<String, String>>>;

// --- Main Application ---

#[tokio::main]
async fn main() {
    // Create an in-memory database for users.
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(db) // Pass the database to the handlers
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// --- Route Handlers ---

async fn root_handler() -> &'static str {
    "Hello, World!"
}

async fn register_handler(
    State(db): State<Db>,
    Json(payload): Json<User>,
) -> (StatusCode, Json<serde_json::Value>) {
    let mut db_lock = db.lock().unwrap();

    if db_lock.contains_key(&payload.username) {
        let response = serde_json::json!({ "message": "Username already exists" });
        return (StatusCode::CONFLICT, Json(response));
    }

    println!("Registering new user: {}", payload.username);
    db_lock.insert(payload.username, payload.password);

    let response = serde_json::json!({ "message": "User registered successfully" });
    (StatusCode::CREATED, Json(response))
}

async fn login_handler(
    State(db): State<Db>,
    Json(payload): Json<User>,
) -> (StatusCode, Json<serde_json::Value>) {
    let db_lock = db.lock().unwrap();

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
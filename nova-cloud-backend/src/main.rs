
use axum::{
    http::Method,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Hello, World!"
}

async fn register_handler(Json(payload): Json<User>) {
    println!("Registering user: {:?}", payload);
    // We will add database logic here later.
}

async fn login_handler(Json(payload): Json<User>) {
    println!("Logging in user: {:?}", payload);
    // We will add authentication logic here later.
}

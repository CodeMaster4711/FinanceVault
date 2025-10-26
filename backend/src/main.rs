use axum::{extract::State, response::Json};
use sea_orm::DbConn;
use serde_json::{json, Value};
use std::net::SocketAddr;
use tokio;

mod auth;
mod auth_service;
mod db;
mod routes;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: DbConn,
}

impl Default for AppState {
    fn default() -> Self {
        // This is a placeholder implementation for middleware
        // In practice, you wouldn't use this default
        panic!("AppState should be created with actual database connection")
    }
}

#[tokio::main]
async fn main() {
    // Load .env file if it exists
    dotenvy::dotenv().ok();

    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Initialize database connection and run migrations
    let db_conn = match db::establish_connection().await {
        Ok(conn) => {
            tracing::info!("Database connection established successfully");
            conn
        }
        Err(e) => {
            tracing::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // Create application state
    let state = AppState { db_conn };

    // build our application with a route
    let app = routes::create_router().with_state(state);

    // run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
pub async fn root(State(state): State<AppState>) -> Json<Value> {
    // You can now use `state.db_conn` to interact with the database
    // For this example, we'll just confirm the connection is there.
    let db_status = if state.db_conn.ping().await.is_ok() {
        "Database connection is healthy"
    } else {
        "Database connection is not healthy"
    };

    Json(json!({ "message": "Hello from FinanceVault!", "database_status": db_status }))
}

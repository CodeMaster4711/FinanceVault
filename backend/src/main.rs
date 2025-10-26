use axum::{extract::State, response::Json};
use sea_orm::DbConn;
use serde_json::{json, Value};
use std::net::SocketAddr;
use tokio;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod auth;
mod auth_service;
mod db;
mod routes;
mod utils;

use routes::users::{
    AuthResponse, LoginRequest, PublicKeyResponse, RegisterRequest, UserProfileResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::users::register_user,
        routes::users::login_user,
        routes::users::logout_user,
        routes::users::get_public_key,
        routes::users::get_user_profile
    ),
    components(
        schemas(RegisterRequest, LoginRequest, AuthResponse, PublicKeyResponse, UserProfileResponse)
    ),
    tags(
        (name = "Authentication", description = "User authentication endpoints"),
        (name = "Encryption", description = "RSA encryption endpoints")
    ),
    info(
        title = "FinanceVault API",
        version = "0.1.0",
        description = "A secure financial vault API with RSA encryption and JWT authentication",
        contact(
            name = "FinanceVault Team",
            email = "support@financevault.com"
        )
    ),
    servers(
        (url = "http://localhost:8000", description = "Local development server")
    )
)]
pub struct ApiDoc;

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
    let app = routes::create_router()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state);

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

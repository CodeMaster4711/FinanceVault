use axum::{
    routing::get,
    Router,
    response::Json,
    extract::State,
};
use sea_orm::DbConn;
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tokio;

mod db;

#[derive(Clone)]
struct AppState {
    db_conn: DbConn,
}

#[tokio::main]
async fn main() {
    // Load .env file if it exists
    dotenvy::dotenv().ok();

    // Initialize database connection
    let db_conn = db::establish_connection().await.expect("Failed to connect to database");

    // Create application state
    let state = AppState { db_conn };

    // a permissive CORS layer is used for development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .with_state(state)
        .layer(cors);

    // run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root(State(state): State<AppState>) -> Json<Value> {
    // You can now use `state.db_conn` to interact with the database
    // For this example, we'll just confirm the connection is there.
    let db_status = if state.db_conn.ping().await.is_ok() {
        "Database connection is healthy"
    } else {
        "Database connection is not healthy"
    };

    Json(json!({ "message": "Hello from FinanceVault!", "database_status": db_status }))
}
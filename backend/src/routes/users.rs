use axum::{routing::get, Router, response::Json};
use serde_json::{json, Value};

// Define the routes for the users module
pub fn users_routes() -> Router<super::AppState> {
    Router::new().route("/users", get(get_users))
}

// Handler for GET /api/users
async fn get_users() -> Json<Value> {
    Json(json!({ "users": [{"id": 1, "name": "Test User"}] }))
}

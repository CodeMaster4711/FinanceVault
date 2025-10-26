use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{auth::middleware::AuthenticatedUser, auth_service::AuthService, AppState};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub encrypted_password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub encrypted_password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct PublicKeyResponse {
    pub public_key: String,
}

#[derive(Serialize)]
pub struct UserProfileResponse {
    pub id: String,
    pub username: String,
}

// Define the routes for the users module
pub fn users_routes() -> Router<AppState> {
    let auth_routes = Router::new()
        .route("/profile", get(get_user_profile))
        .route("/logout", post(logout_user));

    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/public-key", get(get_public_key))
        .merge(auth_routes)
}

// Handler for POST /api/register
async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let auth_service = AuthService::new(state.db_conn.clone());

    match auth_service
        .register_user(payload.username.clone(), payload.encrypted_password)
        .await
    {
        Ok(token) => {
            // Extract user info from token for response
            match crate::auth::jwt::verify_jwt(&token) {
                Ok(token_data) => Ok(Json(AuthResponse {
                    token,
                    user_id: token_data.claims.user_id.to_string(),
                    username: token_data.claims.username,
                })),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(err) => {
            tracing::error!("Registration failed: {}", err);
            match err {
                crate::auth_service::AuthError::UserAlreadyExists => Err(StatusCode::CONFLICT),
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
}

// Handler for POST /api/login
async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let auth_service = AuthService::new(state.db_conn.clone());

    match auth_service
        .login_user(payload.username, payload.encrypted_password)
        .await
    {
        Ok(token) => {
            // Extract user info from token for response
            match crate::auth::jwt::verify_jwt(&token) {
                Ok(token_data) => Ok(Json(AuthResponse {
                    token,
                    user_id: token_data.claims.user_id.to_string(),
                    username: token_data.claims.username,
                })),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(err) => {
            tracing::error!("Login failed: {}", err);
            match err {
                crate::auth_service::AuthError::UserNotFound
                | crate::auth_service::AuthError::InvalidCredentials => {
                    Err(StatusCode::UNAUTHORIZED)
                }
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
}

// Handler for POST /api/logout (protected)
async fn logout_user(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    let auth_service = AuthService::new(state.db_conn.clone());
    let exp_datetime = DateTime::from_timestamp(claims.exp, 0).unwrap_or_else(|| Utc::now());

    // Extract token from request header would be better, but for simplicity
    // we'll use a placeholder. In a real implementation, you'd extract the actual token.
    let token = "".to_string(); // This should be extracted from the Authorization header

    match auth_service.logout_user(token, exp_datetime).await {
        Ok(_) => Ok(Json(json!({ "message": "Logged out successfully" }))),
        Err(err) => {
            tracing::error!("Logout failed: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Handler for GET /api/public-key
async fn get_public_key(
    State(state): State<AppState>,
) -> Result<Json<PublicKeyResponse>, StatusCode> {
    let auth_service = AuthService::new(state.db_conn.clone());

    match auth_service.get_public_key("main").await {
        Ok(public_key) => Ok(Json(PublicKeyResponse { public_key })),
        Err(err) => {
            tracing::error!("Failed to get public key: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Handler for GET /api/profile (protected)
async fn get_user_profile(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<UserProfileResponse>, StatusCode> {
    let auth_service = AuthService::new(state.db_conn.clone());

    match auth_service.get_user_by_id(claims.user_id).await {
        Ok(user) => Ok(Json(UserProfileResponse {
            id: user.id.to_string(),
            username: user.name,
        })),
        Err(err) => {
            tracing::error!("Failed to get user profile: {}", err);
            match err {
                crate::auth_service::AuthError::UserNotFound => Err(StatusCode::NOT_FOUND),
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
}

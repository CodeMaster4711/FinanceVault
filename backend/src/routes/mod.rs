use axum::{routing::get, Router};
use super::{AppState, root};
use crate::utils::router_ext::RouterExt;
use tower_http::cors::{Any, CorsLayer};

mod users;

/// Creates the main application router and logs all registered routes.
pub fn create_router() -> Router<AppState> {
    // a permissive CORS layer is used for development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_router = Router::new().merge(users::users_routes());

    Router::new()
        .logged_route("/", get(root))
        .logged_nest("/api", api_router)
        .layer(cors)
}
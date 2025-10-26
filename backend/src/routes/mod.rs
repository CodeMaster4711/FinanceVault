use super::{root, AppState};
use crate::utils::router_ext::RouterExt;
use axum::body::Body;
use axum::http::{Request, Response};
use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};

pub mod users;

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
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                })
                .on_request(|_request: &Request<Body>, _span: &Span| {
                    tracing::info!("started processing request")
                })
                .on_response(
                    |_response: &Response<Body>, latency: std::time::Duration, _span: &Span| {
                        tracing::info!(
                            latency_ms = latency.as_millis(),
                            "finished processing request"
                        )
                    },
                ),
        )
        .layer(cors)
}

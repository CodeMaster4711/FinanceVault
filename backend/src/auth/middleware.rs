use crate::{auth::jwt::{verify_jwt, Claims}, AppState};
use axum::{async_trait, extract::{FromRequestParts, Request, State}, http::{request::Parts, StatusCode}, middleware::Next, response::Response};
use chrono::Utc;
use entity::InvalidJwt;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .or_else(|| {
            req.headers()
                .get("cookie")
                .and_then(|cookie_header| cookie_header.to_str().ok())
                .and_then(|cookies| {
                    cookies.split(';').find_map(|cookie| {
                        let mut parts = cookie.trim().splitn(2, '=');
                        if parts.next()? == "token" {
                            parts.next()
                        } else {
                            None
                        }
                    })
                })
        });

    if let Some(token) = token {
        // Check if token is in blacklist
        let is_blacklisted = InvalidJwt::find()
            .filter(entity::invalid_jwt::Column::Token.eq(token))
            .one(&state.db_conn)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .is_some();

        if is_blacklisted {
            return Err(StatusCode::UNAUTHORIZED);
        }

        // Verify JWT
        match verify_jwt(token) {
            Ok(token_data) => {
                // Check if token is expired
                if token_data.claims.exp < Utc::now().timestamp() {
                    return Err(StatusCode::UNAUTHORIZED);
                }

                // Add user info to request extensions
                req.extensions_mut().insert(token_data.claims);
                Ok(next.run(req).await)
            }
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

// Custom extractor for authenticated requests
pub struct AuthenticatedUser(pub Claims);

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
            .or_else(|| {
                parts.headers
                    .get("cookie")
                    .and_then(|cookie_header| cookie_header.to_str().ok())
                    .and_then(|cookies| {
                        cookies.split(';').find_map(|cookie| {
                            let mut parts = cookie.trim().splitn(2, '=');
                            if parts.next()? == "token" {
                                parts.next()
                            } else {
                                None
                            }
                        })
                    })
            });

        if let Some(token) = token {
            let is_blacklisted = InvalidJwt::find()
                .filter(entity::invalid_jwt::Column::Token.eq(token))
                .one(&state.db_conn)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .is_some();

            if is_blacklisted {
                return Err(StatusCode::UNAUTHORIZED);
            }

            match verify_jwt(token) {
                Ok(token_data) => {
                    // Check if token is expired
                    if token_data.claims.exp < Utc::now().timestamp() {
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                    Ok(AuthenticatedUser(token_data.claims))
                }
                Err(_) => Err(StatusCode::UNAUTHORIZED),
            }
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

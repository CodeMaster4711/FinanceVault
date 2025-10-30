use crate::{auth::middleware::AuthenticatedUser, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, FixedOffset, Utc};
use entity::{expenses, Expenses};
use sea_orm::prelude::Decimal;
use sea_orm::{ActiveModelTrait, ColumnTrait, DeleteResult, EntityTrait, QueryFilter, Set};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(serde::Deserialize, ToSchema)]
pub struct CreateExpenseRequest {
    pub description: String,
    pub amount: f64,
    pub date: String,
    pub category: String,
}

#[derive(serde::Deserialize, ToSchema)]
pub struct UpdateExpenseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

/// Get all expenses for the authenticated user
#[utoipa::path(
    get,
    path = "/api/expenses",
    responses(
        (status = 200, description = "List of user's expenses", body = [expenses::Model]),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Expenses",
    security(
        ("jwt_token" = [])
    )
)]
pub async fn get_expenses(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<expenses::Model>>, StatusCode> {
    // Nur Expenses des authentifizierten Users abrufen
    let expenses = Expenses::find()
        .filter(expenses::Column::UserId.eq(user.0.user_id))
        .all(&state.db_conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(expenses))
}

/// Get expense by ID for the authenticated user
#[utoipa::path(
    get,
    path = "/api/expenses/{id}",
    params(
        ("id" = Uuid, Path, description = "Expense ID")
    ),
    responses(
        (status = 200, description = "Expense found", body = expenses::Model),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Expense not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Expenses",
    security(
        ("jwt_token" = [])
    )
)]
pub async fn get_expense(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<expenses::Model>, StatusCode> {
    // TODO: Filterung nach user_id wenn die Datenbankstruktur angepasst wird
    // Authenticated user: user.0.user_id
    let _ = user.0.user_id; // Aktuell nicht verwendet, aber bereit für User-Bindung
    let expense = Expenses::find_by_id(id)
        .one(&state.db_conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(expense))
}

/// Create a new expense for the authenticated user
#[utoipa::path(
    post,
    path = "/api/expenses",
    request_body = CreateExpenseRequest,
    responses(
        (status = 200, description = "Expense created successfully", body = expenses::Model),
        (status = 400, description = "Bad request - invalid data"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Expenses",
    security(
        ("jwt_token" = [])
    )
)]
pub async fn create_expense(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateExpenseRequest>,
) -> Result<Json<expenses::Model>, StatusCode> {
    // TODO: user_id zur Expense hinzufügen wenn die Datenbankstruktur angepasst wird
    // Authenticated user: user.0.user_id
    let _ = user.0.user_id; // Aktuell nicht verwendet, aber bereit für User-Bindung

    let new_expense = expenses::ActiveModel {
        id: Set(Uuid::new_v4()),
        description: Set(payload.description),
        amount: Set(Decimal::try_from(payload.amount).map_err(|_| StatusCode::BAD_REQUEST)?),
        date: Set(
            chrono::NaiveDateTime::parse_from_str(&payload.date, "%Y-%m-%d %H:%M:%S")
                .map_err(|_| StatusCode::BAD_REQUEST)?,
        ),
        category: Set(payload.category),
    };

    let expense = Expenses::insert(new_expense)
        .exec(&state.db_conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let created_expense = Expenses::find_by_id(expense.last_insert_id)
        .one(&state.db_conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(created_expense))
}

pub async fn update_expense(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateExpenseRequest>,
) -> Result<Json<expenses::Model>, StatusCode> {
    // TODO: Prüfen ob Expense dem User gehört wenn die Datenbankstruktur angepasst wird
    // Authenticated user: user.0.user_id
    let _ = user.0.user_id; // Aktuell nicht verwendet, aber bereit für User-Bindung

    let mut expense: expenses::ActiveModel = Expenses::find_by_id(id)
        .one(&state.db_conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into();

    // Nur Felder aktualisieren, die gesetzt sind
    if let Some(description) = payload.description {
        expense.description = Set(description);
    }
    if let Some(amount) = payload.amount {
        if amount <= 0.0 {
            return Err(StatusCode::BAD_REQUEST);
        }
        expense.amount = Set(Decimal::try_from(amount).map_err(|_| StatusCode::BAD_REQUEST)?);
    }
    if let Some(date) = payload.date {
        expense.date = Set(
            chrono::NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S")
                .map_err(|_| StatusCode::BAD_REQUEST)?,
        );
    }
    if let Some(category) = payload.category {
        expense.category = Set(category);
    }

    let result = expense
        .update(&state.db_conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}

pub async fn delete_expense(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // TODO: Prüfen ob Expense dem User gehört bevor löschen wenn die Datenbankstruktur angepasst wird
    // Authenticated user: user.0.user_id
    let _ = user.0.user_id; // Aktuell nicht verwendet, aber bereit für User-Bindung

    let result: DeleteResult = Expenses::delete_by_id(id)
        .exec(&state.db_conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn expenses_routes() -> Router<AppState> {
    Router::new()
        .route("/expenses", get(get_expenses).post(create_expense))
        .route(
            "/expenses/:id",
            get(get_expense).put(update_expense).delete(delete_expense),
        )
}

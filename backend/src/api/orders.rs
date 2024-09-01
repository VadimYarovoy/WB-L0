use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing, Router,
};
use serde_json::Value;
use std::sync::Arc;
use tracing::{error, info};
use validator::Validate;

use super::dto;
use crate::AppState;

async fn get_order_by_id(Path(id): Path<i32>, State(state): State<Arc<AppState>>) -> Response {
    let client = state.pool.get().await.unwrap();

    let row = client
        .query_one("SELECT order_data FROM orders WHERE id = $1", &[&id])
        .await;

    match row {
        Ok(row) => {
            info!("Take value with id = {} from db", id);
            let order: serde_json::Value = row.get(0);

            (StatusCode::OK, Json(order)).into_response()
        }
        _ => (
            StatusCode::NOT_FOUND,
            format!("ERROR: There is no order with id = {id}"),
        )
            .into_response(),
    }
}

async fn create_order(
    State(state): State<Arc<AppState>>,
    axum::Json(order_data): axum::Json<dto::Order>,
) -> Response {
    if let Err(errors) = order_data.validate() {
        return (axum::http::StatusCode::BAD_REQUEST, Json(errors)).into_response();
    }

    let client = state.pool.get().await.unwrap();

    let json_value: Value = serde_json::to_value(&order_data).unwrap();

    let result = client
        .execute(
            "INSERT INTO orders (order_data) VALUES ($1)",
            &[&json_value],
        )
        .await;

    match result {
        Ok(_) => (
            StatusCode::CREATED,
            "Order created successfully".to_string(),
        )
            .into_response(),
        Err(err) => {
            error!("Error inserting order: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "ERROR: Could not insert order".to_string(),
            )
                .into_response()
        }
    }
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/get/:id", routing::get(get_order_by_id))
        .route("/create", routing::post(create_order))
}

use crate::AppState;
use axum::{extract::{Path, State, Json}, http::StatusCode, response::{IntoResponse, Response}, routing, Router};
use tracing::info;
use std::sync::Arc;

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

async fn create_order(State(_state): State<Arc<AppState>>) -> &'static str {
    "POST REQUEST"
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/get/:id", routing::get(get_order_by_id))
        .route("/create", routing::post(create_order))
}

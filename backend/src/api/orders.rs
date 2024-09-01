use crate::AppState;
use axum::{extract::State, routing, Router};
use std::sync::Arc;

async fn get_order_by_id(State(_state): State<Arc<AppState>>) -> &'static str {
    "GET REQUEST"
}

async fn create_order(State(_state): State<Arc<AppState>>) -> &'static str {
    "POST REQUEST"
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/get/:id", routing::get(get_order_by_id))
        .route("/create", routing::post(create_order))
}

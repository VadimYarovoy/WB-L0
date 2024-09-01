use axum::Router;

use crate::api;

pub async fn app() -> Router {
    Router::new().nest("/api/", api::routes())
}

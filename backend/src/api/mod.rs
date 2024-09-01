use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub mod dto;
pub mod orders;
pub mod test;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/test", test::routes())
        .nest("/orders", orders::routes())
}

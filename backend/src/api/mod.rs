use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub mod orders;
pub mod test;
pub mod dto;


pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/test", test::routes())
        .nest("/orders", orders::routes())
}

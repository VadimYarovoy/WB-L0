use axum::{routing, Router};

use tracing::info;

async fn root() -> &'static str {
    info!("ping request");
    "pong!\n"
}

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/ping", routing::get(root))
}

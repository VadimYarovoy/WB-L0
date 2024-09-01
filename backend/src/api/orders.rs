use axum::{routing, Router};

pub fn routes<S>() -> Router<S>
where
    S: Send + Sync + Clone + 'static,
{
    Router::new()
        .route("/get/:id", routing::get(|| async { "GET REQUEST" }))
        .route("/create", routing::post(|| async { "POST REQUEST" }))
}

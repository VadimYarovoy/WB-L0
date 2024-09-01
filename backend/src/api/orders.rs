use axum::{routing, Router};

async fn get_order_by_id() -> &'static str {
    "GET REQUEST"
}

async fn create_order() -> &'static str {
    "POST REQUEST"
}

pub fn routes<S>() -> Router<S>
where
    S: Send + Sync + Clone + 'static,
{
    Router::new()
        .route("/get/:id", routing::get(get_order_by_id))
        .route("/create", routing::post(create_order))
}

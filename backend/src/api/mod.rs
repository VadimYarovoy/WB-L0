use axum::Router;

pub mod orders;
pub mod test;

pub fn routes<S>() -> Router<S>
where
    S: Send + Sync + Clone + 'static,
{
    Router::new()
        .nest("/test", test::routes())
        .nest("/orders", orders::routes())
}

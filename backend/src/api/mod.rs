use axum::Router;

pub mod test;

pub fn routes<S>() -> Router<S>
where
    S: Send + Sync + Clone + 'static,
{
    Router::new().nest("/test", test::routes())
}

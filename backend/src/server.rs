use axum::Router;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use std::sync::Arc;
use tokio_postgres::NoTls;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::api;
use crate::config::AppConfig;

pub struct AppState {
    pub pool: Pool,
}

impl AppState {
    async fn create(config: AppConfig) -> Arc<AppState> {
        let mut cfg = Config::new();

        cfg.dbname = Some(config.db.database);
        cfg.host = Some(config.db.host);
        cfg.password = Some(config.db.password);
        cfg.user = Some(config.db.user);
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        let pool = cfg
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .expect("Fildel to create db pool");

        Arc::new(AppState { pool })
    }
}

pub async fn app(config: AppConfig) -> Router {
    let app_state = AppState::create(config).await;

    Router::new()
        .nest("/api/", api::routes())
        .with_state(Arc::clone(&app_state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

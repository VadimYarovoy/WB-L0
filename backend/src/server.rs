use axum::Router;
use deadpool_postgres::{Config, ManagerConfig, Pool as DbPool, RecyclingMethod, Runtime};
use deadpool_redis::Pool as ChPool;
use std::sync::Arc;
use tokio_postgres::NoTls;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::api;
use crate::config::{AppConfig, CacheConfig};

pub struct AppState {
    pub pool: DbPool,
    pub cache_pool: ChPool,
    pub cache_ttl: u64,
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

        let cache_cfg = CacheConfig {
            host: config.cache.host,
            port: config.cache.port,
            ttl: config.cache.ttl,
        };

        let redis_url = format!("redis://{}:{}", cache_cfg.host, cache_cfg.port);
        let cfg = deadpool_redis::Config::from_url(redis_url);
        let cache_pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

        let cache_ttl = cache_cfg.ttl;

        Arc::new(AppState {
            pool,
            cache_pool,
            cache_ttl,
        })
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

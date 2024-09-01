use backend::AppConfig;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let config: AppConfig = AppConfig::figment().extract()?;
    let addr = config.server.bind_addr();
    let app = backend::app(config).await;
    info!("SERVER: Starting");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    Ok(())
}

async fn shutdown_signal() {
    signal::unix::signal(signal::unix::SignalKind::terminate())
        .expect("failed to install signal handler")
        .recv()
        .await;
    opentelemetry::global::shutdown_tracer_provider();
}

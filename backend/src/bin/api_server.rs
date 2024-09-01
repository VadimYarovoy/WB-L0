use tracing::info;

use backend::AppConfig;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    let config: AppConfig = AppConfig::figment().extract()?;
    let addr = config.server.bind_addr();
    let app = backend::app().await;
    info!("SERVER: Starting");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

use lightning_nodes_service::{config::AppConfig, state::AppState};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lightning_nodes_service=info,tower_http=info".into()),
        )
        .init();
    if let Err(error) = self::run().await {
        error!(error = %error, "Application terminated with an error");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::from_env()?;
    let _app_state = AppState::create(app_config).await?;
    info!("HTTP Server runing !!");
    Ok(())
}

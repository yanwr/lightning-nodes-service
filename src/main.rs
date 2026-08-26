use std::sync::Arc;

use lightning_nodes_service::{config::AppConfig, nodes::jobs::job_replace_nodes::run_replace_nodes, state::AppState};
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
    let app_state = Arc::new(AppState::create(app_config.clone()).await?);

    let replace_interval = app_config.replace_interval;
    run_replace_nodes(Arc::clone(&app_state), replace_interval).await;
    info!("HTTP Server runing !!");
    Ok(())
}

use lightning_nodes_service::{
    config::AppConfig, nodes::jobs::job_replace_nodes::run_replace_nodes, routes::build_routes,
    state::AppState,
};
use std::sync::Arc;
use tokio::{net::TcpListener, signal};
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
    let job_replace_nodes =
        tokio::spawn(run_replace_nodes(Arc::clone(&app_state), replace_interval));

    let app_routes = build_routes(app_state);
    let address = format!("{}:{}", app_config.host, app_config.port);
    let listener = TcpListener::bind(&address).await?;
    info!(address = %address, "HTTP Server runing !!");
    axum::serve(listener, app_routes)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    job_replace_nodes.abort();
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to Ctrl+C handler");
        info!("shutdown signal Ctrl+C received");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to SIGTERM handler")
            .recv()
            .await;

        info!("shutdown signal SIGTERM received");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }
}

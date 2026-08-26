use std::{sync::Arc, time::Duration};

use tracing::{error, info};

use crate::{nodes::features::replace::replace_nodes, state::AppState};

pub async fn run_replace_nodes(
    app_state: Arc<AppState>,
    interval: Duration
) {
    let mut ticker = tokio::time::interval(interval);
    loop {
        ticker.tick().await;
        match replace_nodes(&app_state).await {
            Ok(()) => {
                info!(
                    "node import completed"
                );
            }
            Err(error) => {
                error!(
                    error = %error,
                    "node import failed"
                );
            }
        }
    }
}
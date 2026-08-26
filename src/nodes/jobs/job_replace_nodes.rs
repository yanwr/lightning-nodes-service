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
        info!("[Job Replace Nodes] starting node replacement ...");
        match replace_nodes(&app_state).await {
            Ok(()) => {
                info!("[Job Replace Nodes] nodes replace completed");
            }
            Err(error) => {
                // not return Err because the next replace attempt may succeed and the app dont stop
                error!(
                    error = %error,
                    "[Job Replace Nodes] nodes replace failed"
                );
            }
        }
    }
}
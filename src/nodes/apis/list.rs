use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{errors::AppError, nodes::features::list::{NodeResponse, list_nodes}, state::AppState};

pub async fn list(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<NodeResponse>>, AppError> {
    Ok(Json(list_nodes(&app_state).await?))
}
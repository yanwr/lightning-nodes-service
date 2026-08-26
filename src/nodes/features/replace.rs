use tracing::info;

use crate::{errors::AppError, nodes::model::Node, state::AppState};

pub async fn replace_nodes(
    app_state: &AppState,
) -> Result<(), AppError> {
    let mempool_nodes = app_state
        .gateways
        .mempool
        .fetch_rankings_connectivity()
        .await?;
    let nodes = mempool_nodes
        .into_iter()
        .map(Node::try_from)
        .collect::<Result<Vec<Node>, AppError>>()?;
    Node::replace(&app_state.postgres_pool, &nodes).await?;
    info!(nodes_replaceed = nodes.len(), "new replace nodes done!");
    Ok(())
}
use chrono::SecondsFormat;
use serde::Serialize;

use crate::{errors::AppError, nodes::model::Node, state::AppState};

const BTC_IN_STATS: i64= 100_000_000;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct NodeResponse {
    pub public_key: String,
    pub alias: String,
    pub capacity: String,
    pub first_seen: String,
}
impl From<Node> for NodeResponse {
    fn from(node: Node) -> Self {
        Self {
            public_key: node.public_key,
            alias: node.alias,
            capacity: sats_to_btc(node.capacity_sats),
            first_seen: node.first_seen.to_rfc3339_opts(SecondsFormat::Secs, true),
        }
    }
}

fn sats_to_btc(sats: i64) -> String {
    format!("{}.{:08}", sats / BTC_IN_STATS, sats % BTC_IN_STATS)
}

pub async fn list_nodes(
    app_state: &AppState,
) -> Result<Vec<NodeResponse>, AppError> {
    let nodes = Node::list(&app_state.postgres_pool).await?;
    Ok(nodes.into_iter().map(NodeResponse::from).collect())
}
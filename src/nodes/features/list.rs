use chrono::SecondsFormat;
use serde::Serialize;

use crate::{errors::AppError, nodes::model::Node, state::AppState};

const BTC_IN_STATS: i64 = 100_000_000;

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

pub async fn list_nodes(app_state: &AppState) -> Result<Vec<NodeResponse>, AppError> {
    let nodes = Node::list(&app_state.postgres_pool).await?;
    Ok(nodes.into_iter().map(NodeResponse::from).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;
    use test_context::test_context;

    use crate::{infras::test_context::TestContext, nodes::model::Node};

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_empty_nodes_when_database_has_no_nodes(ctx: &mut TestContext) {
        let result = list_nodes(&ctx.app_state).await;
        assert!(result.is_ok());
        let nodes = result.unwrap();
        assert!(nodes.is_empty());
    }

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_all_nodes_when_database_has_nodes(ctx: &mut TestContext) {
        let expected_nodes = vec![
            Node {
                public_key: "public-key-1".to_string(),
                alias: "Node One".to_string(),
                capacity_sats: 36_592_094_162,
                first_seen: DateTime::from_timestamp(1_522_942_122, 0).unwrap(),
            },
            Node {
                public_key: "public-key-2".to_string(),
                alias: "Node Two".to_string(),
                capacity_sats: 7_127_704_538,
                first_seen: DateTime::from_timestamp(1_528_159_480, 0).unwrap(),
            },
        ];

        Node::replace(&ctx.app_state.postgres_pool, &expected_nodes)
            .await
            .unwrap();

        let result = list_nodes(&ctx.app_state).await;
        assert!(result.is_ok());
        let nodes = result.unwrap();
        assert_eq!(nodes.len(), 2);
        let node = nodes
            .iter()
            .find(|node| node.public_key == "public-key-1")
            .expect("node should exist");
        assert_eq!(node.alias, "Node One");
        assert_eq!(node.capacity, "365.92094162");
        assert_eq!(node.first_seen, "2018-04-05T15:28:42Z");
        let node = nodes
            .iter()
            .find(|node| node.public_key == "public-key-2")
            .expect("node should exist");
        assert_eq!(node.alias, "Node Two");
        assert_eq!(node.capacity, "71.27704538");
        assert_eq!(node.first_seen, "2018-06-05T00:44:40Z");
    }
}

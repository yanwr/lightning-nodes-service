use tracing::info;

use crate::{errors::AppError, nodes::model::Node, state::AppState};

pub async fn replace_nodes(app_state: &AppState) -> Result<(), AppError> {
    let mempool_nodes = app_state
        .gateways
        .mempool
        .fetch_rankings_connectivity()
        .await?;
    let nodes = mempool_nodes
        .into_iter()
        .map(Node::try_from)
        .collect::<Result<Vec<Node>, AppError>>()?;
    if nodes.is_empty() {
        info!(
            nodes_replaceed = nodes.len(),
            "[Replace Nodes] There is no nodes to replace"
        );
        return Ok(());
    }
    Node::replace(&app_state.postgres_pool, &nodes).await?;
    info!(
        nodes_replaceed = nodes.len(),
        "[Replace Nodes] replace nodes done!"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use test_context::test_context;
    use wiremock::{
        Mock, ResponseTemplate,
        matchers::{method, path},
    };

    use crate::{
        errors::AppError, infras::test_context::TestContext,
        nodes::features::replace::replace_nodes,
    };

    async fn count_nodes(ctx: &TestContext) -> i64 {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM nodes")
            .fetch_one(&ctx.app_state.postgres_pool)
            .await
            .expect("failed to count nodes")
    }

    async fn node_exists(ctx: &TestContext, public_key: &str) -> bool {
        sqlx::query_scalar::<_, bool>(
            "
            SELECT EXISTS(
                SELECT 1
                FROM nodes
                WHERE public_key = $1
            )
            ",
        )
        .bind(public_key)
        .fetch_one(&ctx.app_state.postgres_pool)
        .await
        .expect("failed to check node existence")
    }

    async fn get_node(ctx: &TestContext, public_key: &str) -> (String, String, i64, DateTime<Utc>) {
        sqlx::query_as::<_, (String, String, i64, DateTime<Utc>)>(
            "
            SELECT public_key, alias, capacity_sats, first_seen
            FROM nodes
            WHERE public_key = $1
            ",
        )
        .bind(public_key)
        .fetch_one(&ctx.app_state.postgres_pool)
        .await
        .expect("failed to fetch node")
    }

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_ok_when_mempool_returns_valid_nodes(ctx: &mut TestContext) {
        let response = serde_json::json!([
            {
                "publicKey": "03abc123",
                "alias": "node-01",
                "capacity": 100000,
                "firstSeen": 1700000000
            },
            {
                "publicKey": "03def456",
                "alias": "node-02",
                "capacity": 200000,
                "firstSeen": 1700000100
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&ctx.mock_server)
            .await;

        let result = replace_nodes(&ctx.app_state).await;
        assert!(result.is_ok());
        assert_eq!(count_nodes(ctx).await, 2);
        let node = get_node(ctx, "03abc123").await;
        assert_eq!(node.0, "03abc123");
        assert_eq!(node.1, "node-01");
        assert_eq!(node.2, 100000);
        assert_eq!(node.3, DateTime::from_timestamp(1700000000, 0).unwrap());
    }

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_ok_when_mempool_returns_empty_nodes(ctx: &mut TestContext) {
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(Vec::<serde_json::Value>::new()))
            .mount(&ctx.mock_server)
            .await;

        let result = replace_nodes(&ctx.app_state).await;
        assert!(result.is_ok());
        assert_eq!(count_nodes(ctx).await, 0);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_mempool_gateway_error_when_mempool_returns_server_error(
        ctx: &mut TestContext,
    ) {
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&ctx.mock_server)
            .await;

        let result = replace_nodes(&ctx.app_state).await;
        assert!(matches!(
            result,
            Err(AppError::MempoolGatewayErrorResponse { .. })
        ));
    }

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_mempool_gateway_error_when_mempool_returns_invalid_json(
        ctx: &mut TestContext,
    ) {
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_string("invalid-json"))
            .mount(&ctx.mock_server)
            .await;

        let result = replace_nodes(&ctx.app_state).await;
        assert!(matches!(result, Err(AppError::MempoolGatewayError { .. })));
    }

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_invalid_data_error_when_node_has_negative_capacity(
        ctx: &mut TestContext,
    ) {
        let response = serde_json::json!([
            {
                "publicKey": "03abc123",
                "alias": "node-01",
                "capacity": -1,
                "firstSeen": 1700000000
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&ctx.mock_server)
            .await;

        let result = replace_nodes(&ctx.app_state).await;
        assert!(matches!(
            result,
            Err(AppError::MempoolGatewayInvalidData(_))
        ));
        assert_eq!(count_nodes(ctx).await, 0);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_invalid_data_error_when_node_has_invalid_first_seen(
        ctx: &mut TestContext,
    ) {
        let response = serde_json::json!([
            {
                "publicKey": "03abc123",
                "alias": "node-01",
                "capacity": 100000,
                "firstSeen": 9223372036854775807_i64
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&ctx.mock_server)
            .await;

        let result = replace_nodes(&ctx.app_state).await;
        assert!(matches!(
            result,
            Err(AppError::MempoolGatewayInvalidData(_))
        ));
        assert_eq!(count_nodes(ctx).await, 0);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_ok_and_replace_existing_nodes(ctx: &mut TestContext) {
        sqlx::query(
            "
            INSERT INTO nodes
                (public_key, alias, capacity_sats, first_seen)
            VALUES
                ($1, $2, $3, $4)
            ",
        )
        .bind("old-node")
        .bind("old-node")
        .bind(50000_i64)
        .bind(DateTime::from_timestamp(1600000000, 0).unwrap())
        .execute(&ctx.app_state.postgres_pool)
        .await
        .expect("failed to insert old node");

        assert_eq!(count_nodes(ctx).await, 1);

        let response = serde_json::json!([
            {
                "publicKey": "new-node-01",
                "alias": "new-node",
                "capacity": 300000,
                "firstSeen": 1700000000
            },
            {
                "publicKey": "new-node-02",
                "alias": "new-node-02",
                "capacity": 400000,
                "firstSeen": 1700000100
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&ctx.mock_server)
            .await;

        let result = replace_nodes(&ctx.app_state).await;
        assert!(result.is_ok());
        assert_eq!(count_nodes(ctx).await, 2);
        assert!(!node_exists(ctx, "old-node").await);
        assert!(node_exists(ctx, "new-node-01").await);
        assert!(node_exists(ctx, "new-node-02").await);
    }
}

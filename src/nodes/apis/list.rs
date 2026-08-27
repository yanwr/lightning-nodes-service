use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{errors::AppError, nodes::features::list::{NodeResponse, list_nodes}, state::AppState};

pub async fn list(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<NodeResponse>>, AppError> {
    Ok(Json(list_nodes(&app_state).await?))
}

#[cfg(test)] 
mod test {
    use axum::{body::Body, http::Request};
    use chrono::DateTime;
    use reqwest::{Method, StatusCode};
    use serde_json::Value;
    use tower::ServiceExt;
    use test_context::test_context;

    use crate::{
        infras::test_context::TestContext, nodes::model::Node, routes::build_routes,
    };

    fn build_request(uri: Option<&str>) -> Request<Body> {
         Request::builder()
            .method(Method::GET)
            .uri(uri.unwrap_or("/nodes"))
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .unwrap()
    }

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_200_when_list_nodes_with_formatted_values(
        ctx: &mut TestContext,
    ) {
        let nodes = vec![
            Node {
                public_key: "node-1".to_string(),
                alias: "Node One".to_string(),
                capacity_sats: 36_592_094_162,
                first_seen: DateTime::from_timestamp(1_522_941_222, 0).unwrap(),
            },
        ];

        Node::replace(
            &ctx.app_state.postgres_pool,
            &nodes,
        )
        .await
        .unwrap();

        let app = build_routes(ctx.app_state.clone().into());
        let response = app
            .oneshot(build_request(None))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(
            response.into_body(),
            usize::MAX,
        )
        .await
        .unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let node = &body[0];
        assert_eq!(node["public_key"], "node-1");
        assert_eq!(node["alias"], "Node One");
        assert_eq!(node["capacity"], "365.92094162");
        assert_eq!(node["first_seen"], "2018-04-05T15:13:42Z");
    }

        #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_200_when_list_nodes_with_empty_database(
        ctx: &mut TestContext,
    ) {
        let app = build_routes(ctx.app_state.clone().into());
        let response = app
            .oneshot(build_request(None))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert!(json.is_array());
        assert!(json.as_array().unwrap().is_empty());
    }

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_404_when_route_does_not_exist(
        ctx: &mut TestContext,
    ) {
        let app = build_routes(ctx.app_state.clone().into());

        let response = app
            .oneshot(build_request(Some("/node")))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_500_when_database_is_unavailable(
        ctx: &mut TestContext,
    ) {
        ctx.app_state.postgres_pool.close().await;
        let app = build_routes(ctx.app_state.clone().into());
        let response = app
            .oneshot(build_request(None))
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["error"]["code"], "DATABASE_ERROR");
        assert_eq!(
            json["error"]["message"],
            "The Database failed with internal errors"
        );
        assert!(json["error"]["request_id"].is_string());
    }

}
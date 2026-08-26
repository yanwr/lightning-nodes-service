use crate::{errors::AppError, gateways::mempool::gateway::MempoolGateway};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MempoolNodeResponse {
    pub public_key: String,
    pub alias: String,
    pub capacity: i64,
    pub first_seen: i64,
}

impl MempoolGateway {
    pub async fn fetch_rankings_connectivity(&self) -> Result<Vec<MempoolNodeResponse>, AppError> {
        let response = self
            .client
            .get(&self.url)
            .send()
            .await
            .map_err(|err| {
                error!(error = %err, "[MempoolGateway] Error to fetch_rankings_connectivity");
                return AppError::MempoolGatewayError { source: err };
            })?;
        if !response.status().is_success() {
            return Err(AppError::MempoolGatewayErrorResponse { status: response.status() })
        }
        let payload = response
            .json::<Vec<MempoolNodeResponse>>()
            .await
            .map_err(|err| {
                error!(error = %err, "[MempoolGateway] Error to parse fetch_rankings_connectivity response");
                return AppError::MempoolGatewayError { source: err };
            })?;
        Ok(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use reqwest::Client;
    use serde_json::json;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn create_gateway(server: &MockServer) -> MempoolGateway {
        MempoolGateway {
            client: Client::new(),
            url: format!("{}/api/v1/lightning/nodes/rankings/connectivity", server.uri()),
        }
    }

    #[tokio::test]
    async fn should_return_nodes_when_request_sucess() {
        let server = MockServer::start().await;
        let response_body = json!([
            {
                "publicKey": "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f",
                "alias": "ACINQ",
                "channels": 2908,
                "capacity": 36010516297_i64,
                "firstSeen": 1522941222_i64,
                "updatedAt": 1661274935_i64,
                "city": null,
                "country": {
                    "en": "United States"
                }
            },
            {
                "publicKey": "035e4ff418fc8b5554c5d9eea66396c227bd429a3251c8cbc711002ba215bfc226",
                "alias": "WalletOfSatoshi.com",
                "channels": 2772,
                "capacity": 15464503162_i64,
                "firstSeen": 1601429940_i64,
                "updatedAt": 1661812116_i64,
                "city": {
                    "en": "Vancouver"
                },
                "country": {
                    "en": "Canada"
                }
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/lightning/nodes/rankings/connectivity"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(response_body),
            )
            .mount(&server)
            .await;

        let gateway = create_gateway(&server);
        let result = gateway.fetch_rankings_connectivity().await;
        assert!(result.is_ok());
        let nodes = result.unwrap();
        assert_eq!(nodes.len(), 2);
        assert_eq!(
            nodes[0].public_key,
            "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f"
        );
        assert_eq!(nodes[0].alias, "ACINQ");
        assert_eq!(nodes[0].capacity, 36_010_516_297);
        assert_eq!(nodes[0].first_seen, 1_522_941_222);
        assert_eq!(
            nodes[1].public_key,
            "035e4ff418fc8b5554c5d9eea66396c227bd429a3251c8cbc711002ba215bfc226"
        );
        assert_eq!(nodes[1].alias, "WalletOfSatoshi.com");
        assert_eq!(nodes[1].capacity, 15_464_503_162);
        assert_eq!(nodes[1].first_seen, 1_601_429_940);
    }

    #[tokio::test]
    async fn should_return_empty_vec_when_response_is_empty() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/lightning/nodes/rankings/connectivity"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([])),
            )
            .mount(&server)
            .await;

        let gateway = create_gateway(&server);
        let result = gateway.fetch_rankings_connectivity().await;
        assert!(result.is_ok());
        let nodes = result.unwrap();
        assert!(nodes.is_empty());
    }

    #[tokio::test]
    async fn should_return_error_when_mempool_returns_500() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/lightning/nodes/rankings/connectivity"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let gateway = create_gateway(&server);
        let result = gateway.fetch_rankings_connectivity().await;
        assert!(matches!(
            result,
            Err(AppError::MempoolGatewayErrorResponse { status })
                if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR
        ));
    }

    #[tokio::test]
    async fn should_return_error_when_mempool_returns_404() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/lightning/nodes/rankings/connectivity"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let gateway = create_gateway(&server);
        let result = gateway.fetch_rankings_connectivity().await;
        assert!(matches!(
            result,
            Err(AppError::MempoolGatewayErrorResponse { status })
                if status == reqwest::StatusCode::NOT_FOUND
        ));
    }

    #[tokio::test]
    async fn should_return_error_when_response_body_is_invalid() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/lightning/nodes/rankings/connectivity"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("this is not valid json"),
            )
            .mount(&server)
            .await;

        let gateway = create_gateway(&server);
        let result = gateway.fetch_rankings_connectivity().await;
        assert!(matches!(
            result,
            Err(AppError::MempoolGatewayError { .. })
        ));
    }

    #[tokio::test]
    async fn should_return_error_when_response_json_has_invalid_schema(
    ) {
        let server = MockServer::start().await;
        let response_body = json!([
            {
                "publicKey": "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f",
                "alias": "ACINQ",
                "capacity": "invalid-capacity",
                "firstSeen": 1522941222_i64
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/lightning/nodes/rankings/connectivity"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(response_body),
            )
            .mount(&server)
            .await;

        let gateway = create_gateway(&server);
        let result = gateway.fetch_rankings_connectivity().await;
        assert!(matches!(
            result,
            Err(AppError::MempoolGatewayError { .. })
        ));
    }

    #[tokio::test]
    async fn should_return_error_when_server_is_unreachable() {
        let gateway = MempoolGateway {
            client: Client::new(),
            url: "http://127.0.0.1:1/api/v1/lightning/nodes/rankings/connectivity"
                .to_owned(),
        };

        let result = gateway.fetch_rankings_connectivity().await;
        assert!(matches!(
            result,
            Err(AppError::MempoolGatewayError { .. })
        ));
    }
}
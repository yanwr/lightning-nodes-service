use std::time::Duration;

use test_context::AsyncTestContext;
use testcontainers::{ContainerAsync, runners::AsyncRunner};
use testcontainers_modules::postgres::Postgres;
use wiremock::MockServer;

use crate::{
    config::{AppConfig, GatewayConfig},
    infras::database::DatabaseConfig,
    state::AppState,
};

pub struct TestContext {
    pub mock_server: MockServer,
    pub app_state: AppState,
    container: ContainerAsync<Postgres>,
}
impl AsyncTestContext for TestContext {
    async fn setup() -> Self {
        let (container, database_url) = self::start_postgres().await;
        let mock_server = MockServer::start().await;
        let app_config = self::start_configs(database_url, &mock_server);
        let app_state = AppState::create(app_config)
            .await
            .expect("[TestContext] failed to create app state");
        Self {
            container,
            mock_server,
            app_state,
        }
    }

    async fn teardown(self) {
        self.app_state.postgres_pool.close().await;
        drop(self.container);
    }
}

async fn start_postgres() -> (ContainerAsync<Postgres>, String) {
    let container = Postgres::default()
        .start()
        .await
        .expect("[TestContext] failed to start postgres container");
    let host = container
        .get_host()
        .await
        .expect("[TestContext] failed to get postgres host");
    let port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("failed to get postgres port");
    let database_url = format!("postgres://postgres:postgres@{}:{}/postgres", host, port);
    (container, database_url)
}

fn start_configs(database_url: String, mock_server: &MockServer) -> AppConfig {
    AppConfig {
        host: String::from("127.0.0.1"),
        port: 0,
        database: DatabaseConfig {
            url: database_url,
            max_connections: 5,
            min_connections: 1,
        },
        gateways: GatewayConfig {
            mempool_url: mock_server.uri(),
            mempool_timeout: Duration::from_secs(5),
        },
        replace_interval: Duration::from_secs(900),
    }
}

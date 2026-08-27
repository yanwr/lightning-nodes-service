use crate::{config::AppConfig, gateways::mempool::gateway::MempoolGateway};

pub mod fetch_rankings_connectivity;
pub mod gateway;

#[derive(Debug, Clone)]
pub struct Gateways {
    pub mempool: MempoolGateway,
}

impl Gateways {
    pub fn new(app_config: &AppConfig) -> Result<Self, reqwest::Error> {
        Ok(Self {
            mempool: MempoolGateway::new(app_config)?,
        })
    }
}

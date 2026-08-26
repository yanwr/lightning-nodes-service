use crate::{config::AppConfig, gateways::mempool::gateway::MempoolGateway};

pub mod gateway;
pub mod fetch_rankings_connectivity;


#[derive(Debug, Clone)]
pub struct Gateways {
    pub mempool: MempoolGateway
}

impl Gateways {
    pub fn new(app_config: &AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { 
            mempool: MempoolGateway::new(app_config)?
        })
    }
}
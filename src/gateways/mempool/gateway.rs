use reqwest::Client;

use crate::config::AppConfig;


#[derive(Debug, Clone)]
pub struct MempoolGateway {
    pub(crate) client: Client,
    pub(crate) url: String
}

impl MempoolGateway {
    pub fn new(app_config: &AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { 
            client: Client::builder()
                .timeout(app_config.gateways.mempool_timeout)
                .build()?,
            url: app_config.gateways.mempool_url.to_string()
        })
    }
}
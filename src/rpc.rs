use anyhow::Result;
use reqwest::Client;

use crate::config::Config;

pub struct RpcClient {
    client: Client,
    config: Config,
}

impl RpcClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn call(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let body = serde_json::json!({
            "jsonrpc": "1.0",
            "id": "bitcoin-rpc-cli",
            "method": method,
            "params": params
        });

        let response = self
            .client
            .post(&self.config.rpc_url)
            .basic_auth(
                &self.config.rpc_user,
                Some(&self.config.rpc_password),
            )
            .json(&body)
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;

        Ok(json)
    }
}
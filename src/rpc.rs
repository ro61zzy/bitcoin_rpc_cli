use anyhow::{bail, Context, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{
    config::Config,
    models::RpcResponse,
};

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

    pub async fn call<T>(
        &self,
        method: &str,
        params: impl serde::Serialize,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
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
            .await
            .context("Failed to connect to Bitcoin Core RPC endpoint")?;

        let rpc_response = response
            .json::<RpcResponse<T>>()
            .await
            .context("Failed to parse JSON-RPC response from Bitcoin Core")?;

        if let Some(error) = rpc_response.error {
            bail!("RPC Error ({}): {}", error.code, error.message);
        }

        let result = rpc_response
            .result
            .context("Bitcoin Core returned an empty result")?;

        Ok(result)
    }
}
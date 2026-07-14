use anyhow::{Context, Result};
use std::env;

pub struct Config {
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            rpc_url: env::var("RPC_URL").context("Missing environment variable: RPC_URL")?,

            rpc_user: env::var("RPC_USER").context("Missing environment variable: RPC_USER")?,

            rpc_password: env::var("RPC_PASSWORD")
                .context("Missing environment variable: RPC_PASSWORD")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_struct_can_be_created() {
        let config = Config {
            rpc_url: "http://localhost:18443".to_string(),
            rpc_user: "bitcoin".to_string(),
            rpc_password: "password".to_string(),
        };

        assert_eq!(config.rpc_url, "http://localhost:18443");
        assert_eq!(config.rpc_user, "bitcoin");
        assert_eq!(config.rpc_password, "password");
    }
}

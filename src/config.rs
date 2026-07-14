use std::env;

pub struct Config {
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            rpc_url: env::var("RPC_URL").expect("RPC_URL is missing"),
            rpc_user: env::var("RPC_USER").expect("RPC_USER is missing"),
            rpc_password: env::var("RPC_PASSWORD").expect("RPC_PASSWORD is missing"),
        }
    }
}
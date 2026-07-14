use anyhow::Result;
use serde_json::{Value, json};

use crate::{config::Config, rpc::RpcClient};

pub async fn run(method: String, params: Vec<String>) -> Result<()> {
    let config = Config::from_env()?;
    let rpc = RpcClient::new(config);

    let response: Value = rpc.call(&method, json!(params)).await?;

    println!();
    println!("╭──────────────────────────────────────────────╮");
    println!("│                RPC Response                  │");
    println!("╰──────────────────────────────────────────────╯");
    println!("{:#}", response);

    Ok(())
}

use anyhow::Result;
use serde_json::json;

use crate::{
    config::Config,
    models::{BlockchainInfo, RpcResponse},
    rpc::RpcClient,
};

pub async fn run() -> Result<()> {
    let config = Config::from_env();
    let rpc = RpcClient::new(config);

    let response: RpcResponse<BlockchainInfo> = rpc
        .call("getblockchaininfo", json!([]))
        .await?;

    println!();
    println!("Blockchain Information");
    println!("======================");
    println!("Chain: {}", response.result.chain);
    println!("Blocks: {}", response.result.blocks);
    println!("Headers: {}", response.result.headers);
    println!("Difficulty: {}", response.result.difficulty);
    println!(
        "Verification Progress: {:.2}%",
        response.result.verificationprogress * 100.0
    );

    Ok(())
}
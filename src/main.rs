mod cli;
mod config;
mod error;
mod models;
mod rpc;

mod commands {
    pub mod address;
    pub mod blockchain;
    pub mod wallet;
}

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use rpc::RpcClient;
use serde_json::json;
use models::{BlockchainInfo, RpcResponse};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::BlockchainInfo => {
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
        }

        Commands::WalletInfo => {
            println!("Wallet info command selected");
        }

       Commands::Balance => {
    let config = Config::from_env();
    let rpc = RpcClient::new(config);

    let response: RpcResponse<f64> = rpc
        .call("getbalance", json!([]))
        .await?;

    println!("Wallet Balance");
    println!("==============");
    println!("{:.8} BTC", response.result);
}

        Commands::NewAddress => {
            println!("New address command selected");
        }

        Commands::Rpc { method, params } => {
    let config = Config::from_env();
    let rpc = RpcClient::new(config);

    let response: serde_json::Value = rpc
        .call(&method, json!(params))
        .await?;

    println!("{:#}", response);
}
    }

    Ok(())
}
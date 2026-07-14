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
use models::{
    Balances,
    BlockchainInfo,
    RpcResponse,
    WalletInfo,
};

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
    let config = Config::from_env();
    let rpc = RpcClient::new(config);

    let wallet: RpcResponse<WalletInfo> = rpc
        .call("getwalletinfo", json!([]))
        .await?;

    let balances: RpcResponse<Balances> = rpc
        .call("getbalances", json!([]))
        .await?;

    println!();
    println!("Wallet Information");
    println!("==================");
    println!("Wallet Name: {}", wallet.result.walletname);
    println!("Transactions: {}", wallet.result.txcount);
    println!("Trusted Balance: {:.8} BTC", balances.result.mine.trusted);
    println!(
        "Untrusted Pending: {:.8} BTC",
        balances.result.mine.untrusted_pending
    );
    println!("Immature Balance: {:.8} BTC", balances.result.mine.immature);
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
    let config = Config::from_env();
    let rpc = RpcClient::new(config);

    let response: RpcResponse<String> = rpc
        .call("getnewaddress", json!([]))
        .await?;

    println!("New Receiving Address");
    println!("=====================");
    println!("{}", response.result);
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
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::BlockchainInfo => {
            let config = Config::from_env();
            let rpc = RpcClient::new(config);

            let response = rpc
                .call("getblockchaininfo", json!([]))
                .await?;

            println!("{:#?}", response);
        }

        Commands::WalletInfo => {
            println!("Wallet info command selected");
        }

        Commands::Balance => {
            println!("Balance command selected");
        }

        Commands::NewAddress => {
            println!("New address command selected");
        }

        Commands::Rpc { method, params } => {
            println!("RPC method: {}", method);
            println!("Params: {:?}", params);
        }
    }

    Ok(())
}
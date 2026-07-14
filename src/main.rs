mod cli;
mod config;
mod error;
mod models;
mod rpc;

mod commands {
    pub mod address;
    pub mod blockchain;
    pub mod rpc;
    pub mod wallet;
}

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

   match cli.command {
    Commands::BlockchainInfo => {
        commands::blockchain::run().await?;
    }

    Commands::WalletInfo => {
        commands::wallet::info().await?;
    }

    Commands::Balance => {
        commands::wallet::balance().await?;
    }

    Commands::NewAddress => {
        commands::address::run().await?;
    }

    Commands::Rpc { method, params } => {
        commands::rpc::run(method, params).await?;
    }
}

    Ok(())
}
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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::BlockchainInfo => {
            println!("Blockchain info command selected");
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
}
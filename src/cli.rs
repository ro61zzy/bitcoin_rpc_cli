use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bitcoin_rpc_cli")]
#[command(about = "A Bitcoin Core JSON-RPC CLI client")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    BlockchainInfo,
    WalletInfo,
    Balance,
    NewAddress,

    Rpc {
        method: String,
        params: Vec<String>,
    },
}
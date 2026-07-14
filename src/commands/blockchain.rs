use anyhow::Result;
use serde_json::json;

use crate::{config::Config, models::BlockchainInfo, rpc::RpcClient};

pub async fn run() -> Result<()> {
    let config = Config::from_env()?;
    let rpc = RpcClient::new(config);

    let info: BlockchainInfo = rpc.call("getblockchaininfo", json!([])).await?;

    println!();
    println!("╭──────────────────────────────────────────────╮");
    println!("│            Blockchain Information            │");
    println!("╰──────────────────────────────────────────────╯");
    println!("Chain:                    {}", info.chain);
    println!("Blocks:                   {}", info.blocks);
    println!("Headers:                  {}", info.headers);
    println!("Difficulty:               {}", info.difficulty);
    println!(
        "Verification Progress:    {:.2}%",
        info.verificationprogress * 100.0
    );

    Ok(())
}

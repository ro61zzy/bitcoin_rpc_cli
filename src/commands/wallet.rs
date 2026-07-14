use anyhow::Result;
use serde_json::json;

use crate::{
    config::Config,
    models::{Balances, WalletInfo},
    rpc::RpcClient,
};

pub async fn info() -> Result<()> {
    let config = Config::from_env()?;
    let rpc = RpcClient::new(config);

    let wallet: WalletInfo = rpc
        .call("getwalletinfo", json!([]))
        .await?;

    let balances: Balances = rpc
        .call("getbalances", json!([]))
        .await?;

    println!();
    println!("Wallet Information");
    println!("==================");
    println!("Wallet Name: {}", wallet.walletname);
    println!("Transactions: {}", wallet.txcount);
    println!("Trusted Balance: {:.8} BTC", balances.mine.trusted);
    println!(
        "Untrusted Pending: {:.8} BTC",
        balances.mine.untrusted_pending
    );
    println!("Immature Balance: {:.8} BTC", balances.mine.immature);

    Ok(())
}

pub async fn balance() -> Result<()> {
    let config = Config::from_env()?;
    let rpc = RpcClient::new(config);

    let balance: f64 = rpc
        .call("getbalance", json!([]))
        .await?;

    println!();
    println!("Wallet Balance");
    println!("==============");
    println!("{:.8} BTC", balance);

    Ok(())
}
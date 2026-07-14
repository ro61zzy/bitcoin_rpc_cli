use anyhow::Result;
use serde_json::json;

use crate::{
    config::Config,
    models::{Balances, RpcResponse, WalletInfo},
    rpc::RpcClient,
};

pub async fn info() -> Result<()> {
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

    Ok(())
}

pub async fn balance() -> Result<()> {
    let config = Config::from_env();
    let rpc = RpcClient::new(config);

    let response: RpcResponse<f64> = rpc
        .call("getbalance", json!([]))
        .await?;

    println!();
    println!("Wallet Balance");
    println!("==============");
    println!("{:.8} BTC", response.result);

    Ok(())
}
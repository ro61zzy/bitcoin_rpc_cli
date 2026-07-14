use anyhow::Result;
use serde_json::json;

use crate::{
    config::Config,
    rpc::RpcClient,
};

pub async fn run() -> Result<()> {
    let config = Config::from_env()?;
    let rpc = RpcClient::new(config);

    let address: String = rpc
        .call("getnewaddress", json!([]))
        .await?;

    println!();
    println!("New Receiving Address");
    println!("=====================");
    println!("{}", address);

    Ok(())
}
use anyhow::Result;
use serde_json::json;

use crate::{
    config::Config,
    models::RpcResponse,
    rpc::RpcClient,
};

pub async fn run() -> Result<()> {
    let config = Config::from_env();
    let rpc = RpcClient::new(config);

    let response: RpcResponse<String> = rpc
        .call("getnewaddress", json!([]))
        .await?;

    println!();
    println!("New Receiving Address");
    println!("=====================");
    println!("{}", response.result);

    Ok(())
}
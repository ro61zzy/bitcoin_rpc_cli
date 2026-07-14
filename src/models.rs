use serde::Deserialize;

use crate::error::RpcError;

#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    pub result: Option<T>,
    pub error: Option<RpcError>,
}

#[derive(Debug, Deserialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: f64,
    pub verificationprogress: f64,
}

#[derive(Debug, Deserialize)]
pub struct WalletInfo {
    pub walletname: String,
    pub txcount: u64,
}

#[derive(Debug, Deserialize)]
pub struct Balances {
    pub mine: MineBalances,
}

#[derive(Debug, Deserialize)]
pub struct MineBalances {
    pub trusted: f64,
    pub untrusted_pending: f64,
    pub immature: f64,
}

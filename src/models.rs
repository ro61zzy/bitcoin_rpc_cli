use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    pub result: T,
    pub error: Option<serde_json::Value>,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: f64,
    pub verificationprogress: f64,
}
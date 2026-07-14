use anyhow::Result;
use serde_json::Value;

use crate::{config::Config, rpc::RpcClient};

pub async fn run(method: String, params: Vec<String>) -> Result<()> {
    let config = Config::from_env()?;
    let rpc = RpcClient::new(config);

    let parsed_params: Vec<Value> = params
        .into_iter()
        .map(|param| serde_json::from_str::<Value>(&param).unwrap_or(Value::String(param)))
        .collect();

    let response: Value = rpc.call(&method, parsed_params).await?;

    println!();
    println!("╭──────────────────────────────────────────────╮");
    println!("│                RPC Response                  │");
    println!("╰──────────────────────────────────────────────╯");
    println!("{:#}", response);

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    #[test]
    fn parses_numeric_parameter() {
        let value =
            serde_json::from_str::<Value>("200").unwrap_or(Value::String("200".to_string()));

        assert_eq!(value, Value::from(200));
    }

    #[test]
    fn parses_boolean_parameter() {
        let value =
            serde_json::from_str::<Value>("true").unwrap_or(Value::String("true".to_string()));

        assert_eq!(value, Value::Bool(true));
    }

    #[test]
    fn falls_back_to_string() {
        let value =
            serde_json::from_str::<Value>("hello").unwrap_or(Value::String("hello".to_string()));

        assert_eq!(value, Value::String("hello".to_string()));
    }
}

use crate::method::convention;
use serde::Serialize;
use serde_json::Value;

use crate::method::convention::{ErrorData, Request};

pub trait Rpc {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized;
    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData>;
}

pub fn create_request<T>(namespace: &str, method: &str, input: T) -> Result<Request, ErrorData>
where
    T: Serialize,
{
    let json_value = serde_json::to_value(input).map_err(|_| ErrorData::std(-32601))?;
    let request = Request {
        jsonrpc: String::from(convention::JSONRPC_VERSION),
        method: format!("{}::{}", namespace, method), //for value Add, it would be Patient::Add
        params: vec![json_value],
        id: Value::from(1),
    };
    Ok(request)
}

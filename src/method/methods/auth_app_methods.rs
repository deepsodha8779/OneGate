use super::authentication::AuthMethods;
use log::info;
use serde_json::Value;

use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::Rpc;

#[allow(clippy::large_enum_variant)]
pub enum AuthAppMethods {
    Auth(AuthMethods),
}

impl Rpc for AuthAppMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        info!(target: "AuthAppMethods::from_name", "str: {:?}, data: {:?}", str, data);
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, elements)) = names.split_first() {
            match *first {
                "Auth" => Ok(AuthAppMethods::Auth(AuthMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };
        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        println!("{}", namespace);
        match self {
            AuthAppMethods::Auth(auth) => auth.to_rpc("Auth"),
        }
    }
}

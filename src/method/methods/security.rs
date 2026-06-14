use crate::dto::security::{Id, Security, SecurityInput, UpdateSecurity};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum SecurityMethods {
    Add(SecurityInput, Option<String>),
    Update(UpdateSecurity, Option<String>),
    Delete(Id, Option<String>),
    GetById(Id, Option<Security>),
    GetAll,
}

impl Rpc for SecurityMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<SecurityInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SecurityMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateSecurity>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SecurityMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SecurityMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SecurityMethods::GetById(input, None))
                }
                "GetAll" => Ok(SecurityMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            SecurityMethods::Add(input, _) => create_request(namespace, "Add", input),

            SecurityMethods::Update(input, _) => create_request(namespace, "Update", input),

            SecurityMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            SecurityMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            SecurityMethods::GetById(input, _) => create_request(namespace, "GetById", input),
        }
    }
}

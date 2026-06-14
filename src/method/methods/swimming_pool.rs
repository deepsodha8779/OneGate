use crate::dto::swimming_pool::{Id, SwimmingPool, SwimmingPoolInput, UpdateSwimmingPool};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum SwimmingPoolMethods {
    Add(SwimmingPoolInput, Option<String>),
    Update(UpdateSwimmingPool, Option<String>),
    Delete(Id, Option<String>),
    GetById(Id, Option<SwimmingPool>),
    GetAll,
}

impl Rpc for SwimmingPoolMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<SwimmingPoolInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SwimmingPoolMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateSwimmingPool>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SwimmingPoolMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SwimmingPoolMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SwimmingPoolMethods::GetById(input, None))
                }
                "GetAll" => Ok(SwimmingPoolMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            SwimmingPoolMethods::Add(input, _) => create_request(namespace, "Add", input),

            SwimmingPoolMethods::Update(input, _) => create_request(namespace, "Update", input),

            SwimmingPoolMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            SwimmingPoolMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            SwimmingPoolMethods::GetById(input, _) => create_request(namespace, "GetById", input),
        }
    }
}

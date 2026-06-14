use crate::dto::home_types::{Home, HomeInput, Id, UpdateHome};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum HomeMethods {
    Add(HomeInput, Option<String>),
    Update(UpdateHome, Option<String>),
    Delete(Id, Option<String>),
    GetById(Id, Option<Home>),
    GetAll,
}

impl Rpc for HomeMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<HomeInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(HomeMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateHome>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(HomeMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(HomeMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(HomeMethods::GetById(input, None))
                }
                "GetAll" => Ok(HomeMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            HomeMethods::Add(input, _) => create_request(namespace, "Add", input),

            HomeMethods::Update(input, _) => create_request(namespace, "Update", input),

            HomeMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            HomeMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            HomeMethods::GetById(input, _) => create_request(namespace, "GetById", input),
        }
    }
}

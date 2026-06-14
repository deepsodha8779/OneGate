use crate::dto::owner::{Id, Owner, OwnerInput, UpdateOwner};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum OwnerMethods {
    Add(OwnerInput, Option<String>),
    Update(UpdateOwner, Option<String>),
    Delete(Id, Option<String>),
    GetById(Id, Option<Owner>),
    GetAll,
}

impl Rpc for OwnerMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<OwnerInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OwnerMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateOwner>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OwnerMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OwnerMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OwnerMethods::GetById(input, None))
                }
                "GetAll" => Ok(OwnerMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            OwnerMethods::Add(input, _) => create_request(namespace, "Add", input),

            OwnerMethods::Update(input, _) => create_request(namespace, "Update", input),

            OwnerMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            OwnerMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            OwnerMethods::GetById(input, _) => create_request(namespace, "GetById", input),
        }
    }
}

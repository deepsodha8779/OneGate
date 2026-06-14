use crate::dto::guest::{Guest, GuestInput, Id, UpdateGuest};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum GuestMethods {
    Add(GuestInput, Option<String>),
    Update(UpdateGuest, Option<String>),
    Delete(Id, Option<String>),
    GetById(Id, Option<Guest>),
    GetAll,
}

impl Rpc for GuestMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<GuestInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(GuestMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateGuest>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(GuestMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(GuestMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(GuestMethods::GetById(input, None))
                }
                "GetAll" => Ok(GuestMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            GuestMethods::Add(input, _) => create_request(namespace, "Add", input),

            GuestMethods::Update(input, _) => create_request(namespace, "Update", input),

            GuestMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            GuestMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            GuestMethods::GetById(input, _) => create_request(namespace, "GetById", input),
        }
    }
}

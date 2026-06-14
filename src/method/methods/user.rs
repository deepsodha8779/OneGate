use crate::dto::common::user::{Id, UpdateUser, User, UserInput};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum UserMethods {
    Add(UserInput, Option<String>),
    Update(UpdateUser, Option<String>),
    Delete(Id, Option<String>),
    GetById(Id, Option<User>),
    GetAll,
}

impl Rpc for UserMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<UserInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(UserMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateUser>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(UserMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(UserMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(UserMethods::GetById(input, None))
                }
                "GetAll" => Ok(UserMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            UserMethods::Add(input, _) => create_request(namespace, "Add", input),

            UserMethods::Update(input, _) => create_request(namespace, "Update", input),

            UserMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            UserMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            UserMethods::GetById(input, _) => create_request(namespace, "GetById", input),
        }
    }
}

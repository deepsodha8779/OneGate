use crate::dto::member::{Id, Member, MemberInput, UpdateMember};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum MemberMethods {
    Add(MemberInput, Option<String>),
    Update(UpdateMember, Option<String>),
    Delete(Id, Option<String>),
    GetById(Id, Option<Member>),
    GetAll,
}

impl Rpc for MemberMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<MemberInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(MemberMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateMember>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(MemberMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(MemberMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(MemberMethods::GetById(input, None))
                }
                "GetAll" => Ok(MemberMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };
        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            MemberMethods::Add(input, _) => create_request(namespace, "Add", input),

            MemberMethods::Update(input, _) => create_request(namespace, "Update", input),

            MemberMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            MemberMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            MemberMethods::GetById(input, _) => create_request(namespace, "GetById", input),
        }
    }
}

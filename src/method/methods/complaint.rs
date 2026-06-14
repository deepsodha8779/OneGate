use crate::dto::complaint::{Complaint, ComplaintInput, Id, UpdateComplaint};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum ComplaintMethods {
    Add(ComplaintInput, Option<String>),
    Update(UpdateComplaint, Option<String>),
    Delete(Id, Option<String>),
    GetById(Id, Option<Complaint>),
    GetAll,
}

impl Rpc for ComplaintMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<ComplaintInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ComplaintMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateComplaint>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ComplaintMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ComplaintMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ComplaintMethods::GetById(input, None))
                }
                "GetAll" => Ok(ComplaintMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            ComplaintMethods::Add(input, _) => create_request(namespace, "Add", input),

            ComplaintMethods::Update(input, _) => create_request(namespace, "Update", input),

            ComplaintMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            ComplaintMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            ComplaintMethods::GetById(input, _) => create_request(namespace, "GetById", input),
        }
    }
}

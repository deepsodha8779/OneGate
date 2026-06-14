use crate::dto::staff::{Id, Staff, StaffInput, UpdateStaff};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum StaffMethods {
    Add(StaffInput, Option<String>),
    Update(UpdateStaff, Option<String>),
    Delete(Id, Option<String>),
    GetById(Id, Option<Staff>),
    GetAll,
}

impl Rpc for StaffMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<StaffInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(StaffMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateStaff>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(StaffMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(StaffMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(StaffMethods::GetById(input, None))
                }
                "GetAll" => Ok(StaffMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            StaffMethods::Add(input, _) => create_request(namespace, "Add", input),

            StaffMethods::Update(input, _) => create_request(namespace, "Update", input),

            StaffMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            StaffMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            StaffMethods::GetById(input, _) => create_request(namespace, "GetById", input),
        }
    }
}

use crate::dto::entrylog::{EntryLog, EntryLogInput, Id, UpdateEntryLog};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum EntryLogMethods {
    Add(EntryLogInput, Option<String>),
    Update(UpdateEntryLog, Option<String>),
    Delete(Id, Option<String>),
    GetById(Id, Option<EntryLog>),
    GetAll,
}

impl Rpc for EntryLogMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<EntryLogInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(EntryLogMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateEntryLog>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(EntryLogMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(EntryLogMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(EntryLogMethods::GetById(input, None))
                }
                "GetAll" => Ok(EntryLogMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            EntryLogMethods::Add(input, _) => create_request(namespace, "Add", input),

            EntryLogMethods::Update(input, _) => create_request(namespace, "Update", input),

            EntryLogMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            EntryLogMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            EntryLogMethods::GetById(input, _) => create_request(namespace, "GetById", input),
        }
    }
}

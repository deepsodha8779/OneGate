use crate::dto::notification::{Id, Notification, NotificationInput, UpdateNotification};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum NotificationMethods {
    Add(NotificationInput, Option<String>),
    Update(UpdateNotification, Option<String>),
    Delete(Id, Option<String>),
    GetById(Id, Option<Notification>),
    GetAll,
}

impl Rpc for NotificationMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<NotificationInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(NotificationMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateNotification>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(NotificationMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(NotificationMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(NotificationMethods::GetById(input, None))
                }
                "GetAll" => Ok(NotificationMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            NotificationMethods::Add(input, _) => create_request(namespace, "Add", input),

            NotificationMethods::Update(input, _) => create_request(namespace, "Update", input),

            NotificationMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            NotificationMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            NotificationMethods::GetById(input, _) => create_request(namespace, "GetById", input),
        }
    }
}

use crate::dto::notice::{Id, Notice, NoticeInput, UpdateNotice};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum NoticeMethods {
    Add(NoticeInput, Option<String>),
    Update(UpdateNotice, Option<String>),
    Delete(Id, Option<String>),
    GetById(Id, Option<Notice>),
    GetAll,
}

impl Rpc for NoticeMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<NoticeInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(NoticeMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateNotice>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(NoticeMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(NoticeMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(NoticeMethods::GetById(input, None))
                }
                "GetAll" => Ok(NoticeMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            NoticeMethods::Add(input, _) => create_request(namespace, "Add", input),

            NoticeMethods::Update(input, _) => create_request(namespace, "Update", input),

            NoticeMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            NoticeMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            NoticeMethods::GetById(input, _) => create_request(namespace, "GetById", input),
        }
    }
}

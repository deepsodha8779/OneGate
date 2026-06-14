use crate::dto::society::{IdInput, Society, SocietyInput, UpdateSociety};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum SocietyMethods {
    Add(SocietyInput, Option<String>),
    Update(UpdateSociety, Option<String>),
    Delete(IdInput, Option<String>),
    GetById(IdInput, Option<Society>),
    GetAll,
}

impl Rpc for SocietyMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<SocietyInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SocietyMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateSociety>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SocietyMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<IdInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SocietyMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<IdInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SocietyMethods::GetById(input, None))
                }
                "GetAll" => Ok(SocietyMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            SocietyMethods::Add(input, _) => create_request(namespace, "Add", input),

            SocietyMethods::Update(input, _) => create_request(namespace, "Update", input),

            SocietyMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            SocietyMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            SocietyMethods::GetById(input, _) => create_request(namespace, "GetById", input),
        }
    }
}

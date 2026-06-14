use crate::dto::amenity::{AmenityInput, Id, UpdateAmenity};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum AmenityMethods {
    Add(AmenityInput, Option<String>),
    Update(UpdateAmenity, Option<String>),
    Delete(Id, Option<String>),
    GetAll,
}

impl Rpc for AmenityMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<AmenityInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AmenityMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateAmenity>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AmenityMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AmenityMethods::Delete(input, None))
                }
                "GetAll" => Ok(AmenityMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            AmenityMethods::Add(input, _) => create_request(namespace, "Add", input),

            AmenityMethods::Update(input, _) => create_request(namespace, "Update", input),

            AmenityMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            AmenityMethods::GetAll => create_request(namespace, "GetAll", Value::Null),
        }
    }
}

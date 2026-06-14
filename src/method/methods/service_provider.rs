use crate::dto::service_provider::{
    Id, ServiceProvider, ServiceProviderInput, UpdateServiceProvider,
};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum ServiceProviderMethods {
    Add(ServiceProviderInput, Option<String>),
    Update(UpdateServiceProvider, Option<String>),
    Delete(Id, Option<String>),
    GetById(Id, Option<ServiceProvider>),
    GetAll,
}

impl Rpc for ServiceProviderMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<ServiceProviderInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ServiceProviderMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdateServiceProvider>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ServiceProviderMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ServiceProviderMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ServiceProviderMethods::GetById(input, None))
                }
                "GetAll" => Ok(ServiceProviderMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            ServiceProviderMethods::Add(input, _) => create_request(namespace, "Add", input),

            ServiceProviderMethods::Update(input, _) => create_request(namespace, "Update", input),

            ServiceProviderMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            ServiceProviderMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            ServiceProviderMethods::GetById(input, _) => {
                create_request(namespace, "GetById", input)
            }
        }
    }
}

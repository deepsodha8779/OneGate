use crate::dto::payment::{Id, Payment, PaymentInput, UpdatePayment};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum PaymentMethods {
    Add(PaymentInput, Option<String>),
    Update(UpdatePayment, Option<String>),
    Delete(Id, Option<String>),
    GetById(Id, Option<Payment>),
    GetAll,
}

impl Rpc for PaymentMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<PaymentInput>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(PaymentMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UpdatePayment>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(PaymentMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(PaymentMethods::Delete(input, None))
                }
                "GetById" => {
                    let input = serde_json::from_value::<Id>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(PaymentMethods::GetById(input, None))
                }
                "GetAll" => Ok(PaymentMethods::GetAll),
                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };
        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            PaymentMethods::Add(input, _) => create_request(namespace, "Add", input),

            PaymentMethods::Update(input, _) => create_request(namespace, "Update", input),

            PaymentMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            PaymentMethods::GetAll => create_request(namespace, "GetAll", Value::Null),

            PaymentMethods::GetById(input, _) => create_request(namespace, "GetById", input),
        }
    }
}

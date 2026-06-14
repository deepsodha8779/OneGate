use crate::dto::auth_types::{LoginMobile, SignUpDTO};
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::create_request;
use crate::rpc::rpc::Rpc;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum AuthMethods {
    LoginMobile(LoginMobile, Option<String>),
    SignUp(SignUpDTO, Option<String>),
    Image,
}

impl Rpc for AuthMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res: Result<AuthMethods, ErrorData> =
            if let Some((first, _elements)) = names.split_first() {
                match *first {
                    "LoginMobile" => {
                        let login_mobile = serde_json::from_value::<LoginMobile>(data[0].clone())
                            .map_err(|_| ErrorData::std(-32601))?;
                        Ok(AuthMethods::LoginMobile(login_mobile, None))
                    }
                    "SignUp" => {
                        let validate_otp = serde_json::from_value::<SignUpDTO>(data[0].clone())
                            .map_err(|_| ErrorData::std(-32601))?;
                        Ok(AuthMethods::SignUp(validate_otp, None))
                    }
                    "Image" => Ok(AuthMethods::Image),
                    _ => Err(ErrorData::std(-32601)),
                }
            } else {
                Err(ErrorData::std(-32601))
            };
        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            AuthMethods::SignUp(signup, _) => create_request(namespace, "SignUp", signup),
            AuthMethods::LoginMobile(login_mobile, _) => {
                create_request(namespace, "LoginMobile", login_mobile)
            }
            AuthMethods::Image => create_request(namespace, "Image", Value::Null),
        }
    }
}

use crate::method::{convention, methods::authentication::AuthMethods};
use crate::services::auth::{login_submit, signup_submit, url_submit};
use serde_json::Value;

use crate::app_state::AppState;

pub async fn process(
    methods: AuthMethods,
    state: &AppState,
) -> Result<Value, convention::ErrorData> {
    match methods {
        AuthMethods::LoginMobile(login_mobile, _) => {
            login_submit(login_mobile, state.clone()).await
        }
        AuthMethods::SignUp(signup, _) => signup_submit(signup, state.clone()).await,
        AuthMethods::Image => url_submit(state.clone()).await,
    }
}

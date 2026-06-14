use crate::apis::auth;
use crate::app_state::AppState;
use crate::method::convention;
use crate::method::methods::auth_app_methods::AuthAppMethods;
use serde_json::Value;

pub async fn rpc_select(
    app_state: &AppState,
    methods: AuthAppMethods,
) -> Result<Value, convention::ErrorData> {
    match methods {
        AuthAppMethods::Auth(a) => auth::process(a, app_state).await,
    }
}

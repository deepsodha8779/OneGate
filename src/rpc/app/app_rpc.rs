use crate::apis::{
    amenity, complaint, entrylog, guest, home, member, notice, notification, owner, payment,
    security, service_provider, society, staff, swimming_pool, user,
};
use crate::app_state::AppState;
use crate::method::convention;
use crate::method::methods::app_methods::AppMethods;
use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;

pub async fn rpc_select(
    app_state: &AppState,
    methods: AppMethods,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, convention::ErrorData> {
    match methods {
        AppMethods::Home(a) => home::process(a, app_state, biscuit).await,
        AppMethods::Society(a) => society::process_society(a, app_state, biscuit).await,
        AppMethods::Amenity(a) => amenity::process(a, app_state, biscuit).await,
        AppMethods::EntryLog(a) => entrylog::process(a, app_state).await,
        AppMethods::Security(a) => security::process(a, app_state, biscuit).await,
        AppMethods::ServiceProvider(a) => service_provider::process(a, app_state, biscuit).await,
        AppMethods::User(a) => user::process(a, app_state, biscuit).await,
        AppMethods::Payment(a) => payment::process(a, app_state).await,
        AppMethods::Notice(a) => notice::process(a, app_state, biscuit).await,
        AppMethods::Complaint(a) => complaint::process(a, app_state, biscuit).await,
        AppMethods::Guest(a) => guest::process(a, app_state).await,
        AppMethods::Member(a) => member::process(a, app_state).await,
        AppMethods::Notification(a) => notification::process(a, app_state).await,
        AppMethods::Owner(a) => owner::process(a, app_state).await,
        AppMethods::Staff(a) => staff::process(a, app_state).await,
        AppMethods::SwimmingPool(a) => swimming_pool::process(a, app_state, biscuit).await,
    }
}

use crate::method::convention;
use crate::method::methods::payment::PaymentMethods;
use crate::services::payment::command::{add_payment, delete_payment, update_payment};
use crate::services::payment::query::get_payment_data;
use serde_json::Value;

use crate::app_state::AppState;

pub async fn process(
    methods: PaymentMethods,
    state: &AppState,
) -> Result<Value, convention::ErrorData> {
    match methods {
        PaymentMethods::Add(input, _) => add_payment(input, state.clone()).await,
        PaymentMethods::Update(input, _) => {
            update_payment(input.input, input.id, state.clone()).await
        }
        PaymentMethods::Delete(input, _) => delete_payment(input.id, state.clone()).await,
        PaymentMethods::GetAll => get_payment_data(state.clone()).await,
        PaymentMethods::GetById(_, _) => todo!(),
    }
}

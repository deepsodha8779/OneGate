use super::common::user::{User, UserInput};
use serde::{Deserialize, Serialize};

use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/ServiceProvider.ts")]

pub struct ServiceProvider {
    pub id: String,
    pub user: User,
    pub aadhar_card_no: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(
    export,
    export_to = "bindings/service_provider/ServiceProviderInput.ts"
)]
pub struct ServiceProviderInput {
    pub user: UserInput,
    pub provider_type: ServiceProviderTypes,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(
    export,
    export_to = "bindings/service_provider/UpdateServiceProvider.ts"
)]
pub struct UpdateServiceProvider {
    pub id: String,
    pub input: ServiceProviderInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/service_provider/Id.ts")]
pub struct Id {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, TS, sqlx::Type)]
#[sqlx(type_name = "serviceprovidertypes", rename_all = "lowercase")]
#[ts(
    export,
    export_to = "bindings/service_provider/ServiceProviderTypes.ts"
)]
pub enum ServiceProviderTypes {
    CableTvRepairer,
    #[default]
    Other,
    Vendor,
    MilkMan,
    Maid,
    Laundry,
    Driver,
    Cook,
    CarCleaner,
}

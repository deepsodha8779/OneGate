use biscuit_auth::PrivateKey;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub private_key: PrivateKey,
}

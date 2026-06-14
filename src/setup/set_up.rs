use crate::setup::prelude::*;
use crate::utils::env_helper::AppEnv;
use anyhow::Result;
use sqlx::PgPool;

pub async fn setup(app_environment: AppEnv, pool: PgPool) -> Result<()> {
    match app_environment {
        AppEnv::Development => dev_setup(pool.clone()).await?,
        AppEnv::Production => prod_setup(pool.clone()).await?,
    }
    Ok(())
}

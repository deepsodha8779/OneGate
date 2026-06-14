use anyhow::{Ok, Result};
use log::info;

use sqlx::PgPool;

use crate::setup::db_setup::{
    first_society_setup::first_society_setup, first_user_setup::first_user_setup, first_user_auth_setup::first_user_auth_setup,
};

pub async fn prod_setup(pool: PgPool) -> Result<()> {
    info!(target: "Setup", "Setting up Prod Env");
    first_society_setup(pool.clone()).await?;
    first_user_setup(pool.clone()).await?;
    first_user_auth_setup(pool.clone()).await?;
    Ok(())
}

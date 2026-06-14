mod apis;
mod app_state;
mod database;
mod dto;
mod server;
mod services;
mod setup;
mod utils;
extern crate validator;
mod method;
mod rpc;
use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok();
    server::server().await
}

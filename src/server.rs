use crate::app_state::AppState;
use crate::rpc::app::app_rpc_handler::rpc_handler;
use crate::setup::set_up::setup;
use crate::utils::env_helper::AppEnv;
use crate::{apis::health_check::health_check, rpc::auth::rpc_handler::auth_rpc_handler};
use actix_cors::Cors;
use actix_web::{http::header, middleware, middleware::Logger, web, App, HttpServer};
use anyhow::Result;
use biscuit_actix_middleware::BiscuitMiddleware;
use biscuit_auth::{Algorithm, PrivateKey, PublicKey};
use dotenv::var;
use log::debug;
use sqlx::migrate::Migrator;
use sqlx::PgPool;
use std::path::Path;


pub async fn server() -> Result<()> {
    let app_environment = AppEnv::current_env()?;
    let database_url = var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool = PgPool::connect(&database_url).await?;
    let migrator = Migrator::new(Path::new("././migrations")).await?;
    migrator
        .run(&pool)
        .await
        .expect("Failed to migrate the database");
    let private_key = PrivateKey::from_bytes_hex(
        &var("PRIVATE_KEY").expect("Private key is not set in .env file"),
        Algorithm::Ed25519,
    )
    .expect("Failed to parse private key");

    let public_key = PublicKey::from_bytes_hex(
        &var("PUBLIC_KEY").expect("Public key is not set in .env file"),
        Algorithm::Ed25519,
    )
    .expect("Failed to parse public key");
    debug!("Public Key for testing purpose: {}", public_key);

    setup(app_environment.clone(), pool.clone()).await?;
    let server = HttpServer::new(move || {
        let cors_base = Cors::default()
            .allowed_methods(vec!["POST", "GET"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        let cors = match app_environment {
            AppEnv::Development => cors_base
                .allowed_origin("http://localhost:5000")
                .allowed_origin("http://127.0.0.1:5173")
                .allowed_origin("http://localhost:5173"),
            AppEnv::Production => cors_base.allowed_origin("http://localhost:5000"),
        };
        let app_state = AppState {
            pool: pool.clone(),
            private_key: private_key.clone(),
        };
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(Logger::new("%a %{User-Agent}i - %D millisecond"))
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .service(web::resource("/auth").route(web::post().to(auth_rpc_handler)))
            .service(
                web::resource("/api")
                    .route(web::post().to(rpc_handler))
                    .wrap(BiscuitMiddleware::new(public_key)),
            )
            .route("/health_check", web::get().to(health_check))
    });
    let _ = server.bind("0.0.0.0:5000")?.run().await;
    Ok(())
}

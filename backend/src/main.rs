use actix_web::{App, HttpServer, web};
use config::{Config as ConfigLoader, Environment};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use tokio::runtime::Builder as TokioRuntimeBuilder;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub worker_threads: usize,
    pub max_blocking_threads: usize,
    pub database_url: String,
    pub database_connection_limit: u32,
}

fn main() {
    let config: Config = ConfigLoader::builder()
        .add_source(Environment::with_prefix("CAJABAMBOO"))
        .build()
        .expect("Failed to build configuration")
        .try_deserialize()
        .expect("Failed to deserialize configuration");

    let runtime = TokioRuntimeBuilder::new_multi_thread()
        .worker_threads(config.worker_threads)
        .max_blocking_threads(config.max_blocking_threads)
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    runtime.block_on(async {
        let pool = PgPoolOptions::new()
            .max_connections(config.database_connection_limit)
            .connect(&config.database_url)
            .await
            .expect("Failed to create connection pool");

        HttpServer::new(move || App::new().app_data(web::Data::new(pool.clone())))
            .bind((config.server_host.as_str(), config.server_port))
            .expect("Failed to bind server")
            .run()
            .await
            .expect("Failed to run server");
    })
}

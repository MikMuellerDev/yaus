use std::{env, process};

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use api::ValidCredentials;
use config::User;
use sqlx::PgPool;

#[macro_use]
extern crate log;

mod api;
mod config;
mod db;

use config::Error as ConfigError;

pub struct State {
    pub db_pool: PgPool,
    pub user: User,
}

#[actix_web::main]
async fn main() {
    //std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let config_path = &env::var("YAUS_CONFIG_PATH").unwrap_or("config.toml".to_string());
    let mut conf = match config::read_config(config_path) {
        Ok(config) => config,
        Err(err) => {
            error!(
                "Failed to read configuration file at {config_path}: {}",
                match err {
                    ConfigError::Io(err) => format!("IO error: {err}"),
                    ConfigError::Parse(err) => format!("Invalid TOML format: {err}"),
                }
            );
            process::exit(1);
        }
    };

    // Scan for environent variables
    conf.scan_env();

    // Initialize the database
    let db_pool = match db::connect(&conf.database).await {
        Err(err) => {
            error!(
                "Could not initialize database connection: {}\n{:?}",
                err.to_string(),
                &conf.database,
            );
            process::exit(1);
        }
        Ok(pool) => pool,
    };

    // Run sqlx migrations on startup
    if let Err(err) = db::run_migrations(&db_pool).await {
        error!(
            "Could not run startup database migration: {}",
            err.to_string()
        );
        process::exit(1);
    };

    // Create the server
    let server = match HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(Data::new(State {
                db_pool: db_pool.clone(),
                user: conf.user.clone(),
            }))
            .service(api::handle_redirect)
            .service(
                // Is required in order to enable the authentication middleware just for the `/api` scope
                web::scope("/api")
                    .wrap(ValidCredentials)
                    .route("/auth", web::get().to(|| HttpResponse::Ok()))
                    .route("/url/{short_id}", web::get().to(api::get_target))
                    .route("/url", web::post().to(api::create_url))
                    .route("/url/{short_id}", web::delete().to(api::delete_url))
                    .route("/urls/{limit}", web::get().to(api::list_urls)),
            )
    })
    .bind(("::0", conf.server.port))
    {
        Ok(server) => server,
        Err(err) => {
            error!("Could not start HTTP server: {err}");
            process::exit(1);
        }
    };
    // Start the server
    info!("YAUS is running on http://localhost:{}", conf.server.port);
    match server.run().await {
        Ok(_) => warn!("YAUS is shutting down..."),
        Err(err) => {
            error!("Could not start HTTP server: {err}");
            process::exit(1);
        }
    };
}

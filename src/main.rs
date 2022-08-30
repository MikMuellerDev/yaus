use std::process;

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use api::ValidCredentials;
use serde::Deserialize;
use sqlx::PgPool;

#[macro_use]
extern crate log;

mod api;
mod db;

pub struct State {
    pub db_pool: PgPool,
    pub user: User,
}

#[derive(Clone, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // TODO: replace with environment variables for configuration
    let db_config = db::DBConfig {
        hostname: "localhost".to_string(),
        port: 5432,
        username: "test".to_string(),
        password: "test".to_string(),
        database: "test".to_string(),
    };

    let user = User {
        username: "test".to_string(),
        password: "secret".to_string(),
    };

    // Initialize the database
    let db_pool = match db::connect(db_config).await {
        Err(err) => {
            error!(
                "Could not initialize database connection: {}",
                err.to_string()
            );
            process::exit(1);
        }
        Ok(pool) => pool,
    };

    // Run sqlx migrations on startup
    if let Err(err) = sqlx::migrate!().run(&db_pool).await {
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
                user: user.clone(),
            }))
            .service(api::handle_redirect)
            .service(
                // Is required in order to enable the authentication middleware just for the `/api` scope
                web::scope("/api")
                    .wrap(ValidCredentials)
                    .route("/url", web::post().to(api::create_url))
                    .route("/url/{query}", web::delete().to(api::delete_url))
                    .route("/urls", web::get().to(api::list_urls)),
            )
    })
    .bind(("localhost", 8080))
    {
        Ok(server) => server,
        Err(err) => {
            error!("Could not start HTTP server: {err}");
            process::exit(1);
        }
    };
    // Start the server
    match server.run().await {
        Ok(_) => (),
        Err(err) => {
            error!("Could not start HTTP server: {err}");
            process::exit(1);
        }
    };
}

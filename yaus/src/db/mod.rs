use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::DatabaseConfig;

pub mod url;

pub async fn connect(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    debug!("Initializing database pool...");

    // Create a database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .min_connections(5)
        .connect(
            format!(
                "postgres://{}:{}@{}:{}/{}",
                config.username, config.password, config.hostname, config.port, config.database,
            )
            .as_str(),
        )
        .await?;

    // Attempt to perform a query for testing the connection
    let _: (i8,) = sqlx::query_as("SELECT $1")
        .bind(0_i8)
        .fetch_one(&pool)
        .await?;

    info!("Successfully connected to the database");
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    debug!("Running SQL migrations...");
    sqlx::migrate!().run(pool).await?;
    info!("Successfully executed SQL migrations");
    Ok(())
}

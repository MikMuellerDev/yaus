use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    pub short: String,
    pub target_url: String,
}

#[derive(Debug)]
pub enum Error {
    ShortExists,
    ShortDoesNotExist,
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub async fn create_url(url: &Url, pool: &PgPool) -> Result<()> {
    match sqlx::query!(
        r#"
        INSERT INTO
        url(
            short,
            target_url
        )
        VALUES($1, $2)
        ON CONFLICT (short) DO NOTHING
        "#,
        url.short,
        url.target_url,
    )
    .execute(pool)
    .await?
    .rows_affected()
    {
        0 => Err(Error::ShortExists),
        _ => Ok(()),
    }
}

pub async fn delete_url(short: &str, pool: &PgPool) -> Result<()> {
    match sqlx::query!(
        r#"
        DELETE FROM
        url
        WHERE short=$1
        "#,
        short,
    )
    .execute(pool)
    .await?
    .rows_affected()
    {
        0 => Err(Error::ShortDoesNotExist),
        _ => Ok(()),
    }
}

pub async fn get_url(short: &str, pool: &PgPool) -> Result<Url> {
    let url = sqlx::query_as!(
        Url,
        r#"
        SELECT
            short,
            target_url
        FROM url
        WHERE short=$1
        "#,
        short,
    )
    .fetch_optional(pool)
    .await?;

    match url {
        None => Err(Error::ShortDoesNotExist),
        Some(url) => Ok(url),
    }
}

pub async fn list_urls(pool: &PgPool, max_entries: i64) -> Result<Vec<Url>> {
    Ok(sqlx::query_as!(
        Url,
        r#"
        SELECT
            short,
            target_url
        FROM url
        LIMIT $1
        "#,
        max_entries,
    )
    .fetch_all(pool)
    .await?)
}

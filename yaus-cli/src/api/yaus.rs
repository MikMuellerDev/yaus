use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};

use super::client::Client;
use super::errors::{Error, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Redirect {
    pub short: String,
    pub target_url: String,
}

impl Client<'_> {
    pub async fn list_urls(&self) -> Result<Vec<Redirect>> {
        let result = self
            .client
            .execute(self.build_request::<()>(Method::GET, "/api/urls", None)?)
            .await?;
        match result.status() {
            StatusCode::OK => Ok(result.json().await?),
            status => Err(Error::Yaus(status)),
        }
    }

    pub async fn get_target(&self, short_id: &str) -> Result<Redirect> {
        let result = self
            .client
            .execute(
                self.build_request::<()>(
                    Method::GET,
                    {
                        let mut url = self.url.clone();
                        url.set_path(&format!("/api/url/{short_id}"));
                        url
                    }
                    .as_str(),
                    None,
                )?,
            )
            .await?;
        match result.status() {
            StatusCode::OK => Ok(result.json().await?),
            status => Err(Error::Yaus(status)),
        }
    }

    pub async fn create_url(&self, redirect: &Redirect) -> Result<()> {
        let result = self
            .client
            .execute(self.build_request::<&Redirect>(Method::POST, "/api/url", Some(redirect))?)
            .await?;
        match result.status() {
            StatusCode::OK => Ok(()),
            status => Err(Error::Yaus(status)),
        }
    }

    pub async fn delete_url(&self, short_id: &str) -> Result<()> {
        let result = self
            .client
            .execute(
                self.build_request::<&Redirect>(
                    Method::DELETE,
                    {
                        let mut url = self.url.clone();
                        url.set_path(&format!("/api/url/{short_id}"));
                        url
                    }
                    .as_str(),
                    None,
                )?,
            )
            .await?;
        match result.status() {
            StatusCode::OK => Ok(()),
            status => Err(Error::Yaus(status)),
        }
    }
}

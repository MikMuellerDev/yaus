use super::{
    errors::{Error, Result},
    HTTP_USER_AGENT,
};
use reqwest::{Method, StatusCode, Url};
use serde::Serialize;

pub struct Client<'client> {
    pub client: reqwest::Client,
    pub user: User<'client>,
    pub url: Url,
}

#[derive(Serialize)]
pub struct User<'user> {
    pub username: &'user str,
    pub password: &'user str,
}

impl<'client> Client<'client> {
    pub async fn new(raw_url: &str, user: User<'client>) -> Result<Client<'client>> {
        // Parse the source url into an URL struct
        let url = Url::parse(raw_url)?;

        // Default client with user agent is created
        let client = reqwest::Client::builder()
            .user_agent(HTTP_USER_AGENT)
            .build()?;

        // Attempt to authenticate using the provided credentials
        let response = client
            .request(Method::GET, url.join("/api/auth")?)
            .query(&[("username", user.username), ("password", user.password)])
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(Self { client, user, url }),
            status => Err(Error::Yaus(status)),
        }
    }
}

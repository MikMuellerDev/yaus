use reqwest::{Method, Request};
use serde::Serialize;

use super::{client::Client, errors::Result};

impl Client<'_> {
    pub fn build_request<Body: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: Option<Body>,
    ) -> Result<Request> {
        // Create a request
        let request = self.client.request(method, self.url.join(path)?).query(&[
            ("username", self.user.username),
            ("password", self.user.password),
        ]);
        // Append a body if needed
        match body {
            Some(body) => Ok(request.json(&body).build()?),
            None => Ok(request.build()?),
        }
    }
}

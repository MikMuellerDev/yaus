mod middleware;
mod url;

pub use middleware::ValidCredentials;
pub use url::*;

#[derive(serde::Serialize)]
pub struct GenericResponse<'response> {
    success: bool,
    message: &'response str,
    error: Option<&'response str>,
}

impl<'response> GenericResponse<'response> {
    pub fn success(message: &'response str) -> Self {
        Self {
            success: true,
            message,
            error: None,
        }
    }
    pub fn err(message: &'response str, error: &'response str) -> Self {
        Self {
            success: false,
            message,
            error: Some(error),
        }
    }
}

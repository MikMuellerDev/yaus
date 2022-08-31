mod client;
mod errors;
mod request;
mod yaus;

pub use client::{Client, User};
pub use errors::{Error, Result};
pub use yaus::Redirect;

const HTTP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

use reqwest::StatusCode;
use url::ParseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UrlParse(ParseError),
    Reqwest(reqwest::Error),
    Yaus(StatusCode),
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Self::UrlParse(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum Error {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Json parsing error: {0}")]
    JsonParsingError(String),
}

impl From<reqwest::Error> for Error {
    fn from(reqwest_err: reqwest::Error) -> Error {
        Error::NetworkError(reqwest_err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(parsing_err: serde_json::Error) -> Error {
        Error::JsonParsingError(parsing_err.to_string())
    }
}

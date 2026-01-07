// We use this instead of thiserror::transparent because we need the types to be
// serializable/deserializable
#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum Error {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Tauri Store plugin error: {0}")]
    StoreError(String),
    #[error("Json parsing error: {0}")]
    JsonParsingError(String),
    #[error("Parsing error: {0}")]
    ParsingError(String),
    #[error("The Json response is not as expected: {0}")]
    JsonResponseContentNotAsExpected(String),
    #[error("serde_wasm_bindgen error: {0}")]
    WasmConversionError(String),
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

// impl From<serde_wasm_bindgen::Error> for Error {
//     fn from(err: serde_wasm_bindgen::Error) -> Error {
//         Error::WasmConversionError(err.to_string())
//     }
// }

use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    /// A malformed JSON payload has been returned.
    #[error("Invalid JSON returned")]
    InvalidJson {
        /// The error data from GitLab.
        data: Vec<u8>,
    },

    /// The server returned a non success status code.
    #[error("Status code is not success")]
    NotSuccess { obj: serde_json::Value },
}

#[derive(Debug, Error)]
pub enum ApiError<E>
where
    E: Error,
{
    /// The client encountered an error.
    #[error("Client error: {}", source)]
    Client { source: E },

    /// The URL failed to parse.
    #[error("Failed to parse url: {}", source)]
    UrlParse {
        #[from]
        /// The parse error.
        source: url::ParseError,
    },

    /// A server error occured.
    #[error("A server error occured: {}", status)]
    ServerError {
        /// The status code for the return.
        status: http::StatusCode,

        /// The server error.
        source: ServerError,
    },

    /// Failed to parse an expected data type from JSON.
    #[error("Could not parse {} data from JSON: {}", typename, source)]
    DataType {
        /// The name of the type that could not be deserialized.
        typename: &'static str,

        /// The JSON payload to parse.
        obj: serde_json::Value,

        /// The JSON parse error.
        source: serde_json::Error,
    },
}

impl<E> ApiError<E>
where
    E: Error,
{
    pub fn client(source: E) -> Self {
        ApiError::Client { source }
    }
}

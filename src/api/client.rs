use std::error::Error;

use async_trait::async_trait;
use bytes::Bytes;
use http::{request::Builder as RequestBuilder, Response};
use url::Url;

use crate::api::error::ApiError;

/// A trait representing a client which can communicate with the Bitfinex REST API.
pub trait RestClient {
    /// The errors which may occur for this client.
    type Error: Error;

    /// Get the URL for the endpoint for the client.
    ///
    /// This method adds the hostname for the client's target instance.
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;
}

/// A trait representing a client which can communicate with the Bitfinex REST API.
pub trait Client: RestClient {
    /// Send a REST query.
    fn rest(
        &self,
        request_builder: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

/// A trait representing an asynchronous client which can communicate with the Bitfinex REST API.
#[async_trait]
pub trait AsyncClient: RestClient {
    /// Send a REST query asynchronously.
    async fn rest_async(
        &self,
        request_builder: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

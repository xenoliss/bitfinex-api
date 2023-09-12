use async_trait::async_trait;
use bytes::Bytes;
use http::{request::Builder as RequestBuilder, Response};
use reqwest::blocking::Client as ReqClient;
use reqwest::Client as ReqAsyncClient;
use thiserror::Error;
use url::Url;

use crate::{
    api::client::{AsyncClient, Client, RestClient},
    api::error::ApiError,
};

#[derive(Debug, Error)]
pub enum RestError {
    #[error("`HTTP error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },

    #[error("Communication with Bitfinex: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
}

const PUB_API_URL: &str = "https://api-pub.bitfinex.com/v2/";
const AUTH_API_URL: &str = "https://api.bitfinex.com/v2/";

#[derive(Debug)]
pub struct Bitfinex {
    /// The client to use for API calls.
    client: ReqClient,

    /// The base URL to use for API calls.
    rest_url: Url,
}

#[derive(Debug)]
pub struct AsyncBitfinex {
    /// The client to use for API calls.
    client: ReqAsyncClient,

    /// The base URL to use for API calls.
    rest_url: Url,
}

impl Bitfinex {
    pub fn new() -> Self {
        Self {
            client: ReqClient::new(),
            rest_url: Url::parse(PUB_API_URL).unwrap(),
        }
    }
}

impl AsyncBitfinex {
    pub fn new() -> Self {
        Self {
            client: ReqAsyncClient::new(),
            rest_url: Url::parse(PUB_API_URL).unwrap(),
        }
    }
}

impl Default for Bitfinex {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AsyncBitfinex {
    fn default() -> Self {
        Self::new()
    }
}

impl RestClient for Bitfinex {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        Ok(self.rest_url.join(endpoint)?)
    }
}

impl RestClient for AsyncBitfinex {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        Ok(self.rest_url.join(endpoint)?)
    }
}

impl Client for Bitfinex {
    fn rest(
        &self,
        request_builder: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let call = || {
            let http_request = request_builder.body(body)?;

            let request = http_request.try_into()?;
            let rsp = self.client.execute(request)?;

            let mut http_rsp = Response::builder()
                .status(rsp.status())
                .version(rsp.version());

            if let Some(headers) = http_rsp.headers_mut() {
                for (key, value) in rsp.headers() {
                    headers.insert(key, value.clone());
                }
            }

            Ok(http_rsp.body(rsp.bytes()?)?)
        };

        call().map_err(ApiError::client)
    }
}

#[async_trait]
impl AsyncClient for AsyncBitfinex {
    async fn rest_async(
        &self,
        request_builder: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<<Self as RestClient>::Error>> {
        let call = || async {
            let http_request = request_builder.body(body)?;

            let request = http_request.try_into()?;
            let rsp = self.client.execute(request).await?;

            let mut http_rsp = Response::builder()
                .status(rsp.status())
                .version(rsp.version());

            if let Some(headers) = http_rsp.headers_mut() {
                for (key, value) in rsp.headers() {
                    headers.insert(key, value.clone());
                }
            }

            Ok(http_rsp.body(rsp.bytes().await?)?)
        };

        call().await.map_err(ApiError::client)
    }
}

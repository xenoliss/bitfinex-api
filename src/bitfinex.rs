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
    auth::Auth,
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

const PUB_API_URL: &str = "https://api-pub.bitfinex.com";
const AUTH_API_URL: &str = "https://api.bitfinex.com";

#[derive(Debug)]
pub struct Bitfinex {
    /// The client to use for API calls.
    client: ReqClient,

    /// The base URL to use for public API calls.
    pub_rest_url: Url,

    /// The base URL to use for authenticated API calls.
    authenticated_rest_url: Url,

    /// The authentication information to use.
    auth: Option<Auth>,
}

#[derive(Debug)]
pub struct AsyncBitfinex {
    /// The client to use for API calls.
    client: ReqAsyncClient,

    /// The base URL to use for public API calls.
    pub_rest_url: Url,

    /// The base URL to use for authenticated API calls.
    authenticated_rest_url: Url,

    /// The authentication information to use.
    auth: Option<Auth>,
}

impl Bitfinex {
    pub fn new(auth: Option<Auth>) -> Self {
        Self {
            client: ReqClient::new(),
            pub_rest_url: Url::parse(PUB_API_URL).unwrap(),
            authenticated_rest_url: Url::parse(AUTH_API_URL).unwrap(),
            auth,
        }
    }
}

impl AsyncBitfinex {
    pub fn new(auth: Option<Auth>) -> Self {
        Self {
            client: ReqAsyncClient::new(),
            pub_rest_url: Url::parse(PUB_API_URL).unwrap(),
            authenticated_rest_url: Url::parse(AUTH_API_URL).unwrap(),
            auth,
        }
    }
}

impl RestClient for Bitfinex {
    type Error = RestError;

    fn rest_endpoint(
        &self,
        endpoint: &str,
        is_authenticated: bool,
    ) -> Result<Url, ApiError<Self::Error>> {
        if is_authenticated {
            Ok(self.authenticated_rest_url.join(endpoint)?)
        } else {
            Ok(self.pub_rest_url.join(endpoint)?)
        }
    }
}

impl RestClient for AsyncBitfinex {
    type Error = RestError;

    fn rest_endpoint(
        &self,
        endpoint: &str,
        is_authenticated: bool,
    ) -> Result<Url, ApiError<Self::Error>> {
        if is_authenticated {
            Ok(self.authenticated_rest_url.join(endpoint)?)
        } else {
            Ok(self.pub_rest_url.join(endpoint)?)
        }
    }
}

impl Client for Bitfinex {
    fn rest(
        &self,
        mut request_builder: RequestBuilder,
        body: Vec<u8>,
        path_to_sign: Option<String>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let call = || {
            // If a path to signe has been provided, compute and adds the necessary authorization headers to the request.
            if let (Some(path_to_sign), Some(auth)) = (path_to_sign, &self.auth) {
                auth.set_headers(request_builder.headers_mut().unwrap(), &path_to_sign, &body);
            }

            // Build the request.
            let http_request = request_builder.body(body)?;

            // Convert it to a reqwest::Request type and send it.
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request)?;

            // Build the HTTP response.
            let mut http_rsp = Response::builder()
                .status(rsp.status())
                .version(rsp.version());

            // Insert any headers in the reponses.
            if let Some(headers) = http_rsp.headers_mut() {
                for (key, value) in rsp.headers() {
                    headers.insert(key, value.clone());
                }
            }

            // Return the reponse as raw bytes.
            Ok(http_rsp.body(rsp.bytes()?)?)
        };

        call().map_err(ApiError::client)
    }
}

#[async_trait]
impl AsyncClient for AsyncBitfinex {
    async fn rest_async(
        &self,
        mut request_builder: RequestBuilder,
        body: Vec<u8>,
        path_to_sign: Option<String>,
    ) -> Result<Response<Bytes>, ApiError<<Self as RestClient>::Error>> {
        let call = || async {
            // If a path to signe has been provided, compute and adds the necessary authorization headers to the request.
            if let (Some(path_to_sign), Some(auth)) = (path_to_sign, &self.auth) {
                auth.set_headers(request_builder.headers_mut().unwrap(), &path_to_sign, &body);
            }

            // Build the request.
            let http_request = request_builder.body(body)?;

            // Convert it to a reqwest::Request type and send it.
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request).await?;

            // Build the HTTP response.
            let mut http_rsp = Response::builder()
                .status(rsp.status())
                .version(rsp.version());

            // Insert any headers in the reponses.
            if let Some(headers) = http_rsp.headers_mut() {
                for (key, value) in rsp.headers() {
                    headers.insert(key, value.clone());
                }
            }

            // Return the reponse as raw bytes.
            Ok(http_rsp.body(rsp.bytes().await?)?)
        };

        call().await.map_err(ApiError::client)
    }
}

use std::any;

use async_trait::async_trait;
use http::{header, Method, Request};
use serde::de::DeserializeOwned;

use crate::api::{
    client::{AsyncClient, Client},
    error::{ApiError, ServerError},
    params::QueryParams,
    query::{url_to_http_uri, AsyncQuery, Query},
};

/// A trait for providing the necessary information for a single REST API endpoint.
pub trait Endpoint {
    /// The HTTP method to use for the endpoint.
    fn method(&self) -> Method;

    /// The path to the endpoint.
    fn endpoint(&self) -> &'static str;

    /// Query parameters for the endpoint.
    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    /// The body for the endpoint.
    ///
    /// Returns the `Content-Encoding` header for the data as well as the data itself.
    fn body(&self) -> Option<(&'static str, Vec<u8>)> {
        None
    }
}

impl<E, T, C> Query<T, C> for E
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(self.endpoint())?;
        self.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.method())
            .uri(url_to_http_uri(url));

        let (req, data) = if let Some((mime, data)) = self.body() {
            (req.header(header::CONTENT_TYPE, mime), data)
        } else {
            (req, Vec::new())
        };

        let rsp = client.rest(req, data)?;
        let status = rsp.status();

        let v = serde_json::from_slice(rsp.body()).map_err(|_e| ApiError::ServerError {
            status,
            source: ServerError::InvalidJson {
                data: rsp.body().into_iter().copied().collect(),
            },
        })?;

        if !status.is_success() {
            return Err(ApiError::ServerError {
                status,
                source: ServerError::NotSuccess { obj: v },
            });
        }

        serde_json::from_value::<T>(v.clone()).map_err(|e| ApiError::DataType {
            typename: any::type_name::<T>(),
            obj: v,
            source: e,
        })
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<T, C> for E
where
    E: Endpoint + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(self.endpoint())?;
        self.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.method())
            .uri(url_to_http_uri(url));

        let (req, data) = if let Some((mime, data)) = self.body() {
            (req.header(header::CONTENT_TYPE, mime), data)
        } else {
            (req, Vec::new())
        };

        let rsp = client.rest_async(req, data).await?;
        let status = rsp.status();

        let v = serde_json::from_slice(rsp.body()).map_err(|_e| ApiError::ServerError {
            status,
            source: ServerError::InvalidJson {
                data: rsp.body().into_iter().copied().collect(),
            },
        })?;

        if !status.is_success() {
            return Err(ApiError::ServerError {
                status,
                source: ServerError::NotSuccess { obj: v },
            });
        }

        serde_json::from_value::<T>(v.clone()).map_err(|e| ApiError::DataType {
            typename: any::type_name::<T>(),
            obj: v,
            source: e,
        })
    }
}

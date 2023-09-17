use async_trait::async_trait;
use http::{header, Request};

use super::{
    client::{AsyncClient, Client},
    endpoint::Endpoint,
    error::{ApiError, ServerError},
    query::{url_to_http_uri, AsyncQuery, Query},
};

/// A query modifier that ignores the data returned from an endpoint.
#[derive(Debug, Clone, Copy)]
pub struct Ignore<E> {
    endpoint: E,
}

/// Ignore the resulting data from an endpoint.
pub fn ignore<E>(endpoint: E) -> Ignore<E> {
    Ignore { endpoint }
}
impl<E, C> Query<(), C> for Ignore<E>
where
    E: Endpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        let is_authenicated = self.endpoint.is_authenticated();
        let endpoint = self.endpoint.endpoint();

        // Build the URL.
        let mut url = client.rest_endpoint(&endpoint, is_authenicated)?;

        // Add query parameters to the URL.
        self.endpoint.parameters().add_to_url(&mut url);

        let request_builder = Request::builder()
            .method(self.endpoint.method())
            .uri(url_to_http_uri(url));

        // Add the body to the request if any.
        let (request_builder, data) = if let Some((mime, data)) = self.endpoint.body() {
            (request_builder.header(header::CONTENT_TYPE, mime), data)
        } else {
            (request_builder, Vec::new())
        };

        // Send off the request
        let rsp = client.rest(request_builder, data, is_authenicated.then_some(endpoint))?;

        // Check the response status and extract errors if needed.
        let status = rsp.status();

        if !status.is_success() {
            // For debug purposes try to deseralize the error.
            let v = serde_json::from_slice(rsp.body()).map_err(|_e| ApiError::ServerError {
                status,
                source: ServerError::InvalidJson {
                    data: rsp.body().into_iter().copied().collect(),
                },
            })?;

            return Err(ApiError::ServerError {
                status,
                source: ServerError::NotSuccess { obj: v },
            });
        }

        // Skip the deserialization process.
        Ok(())
    }
}

#[async_trait]
impl<E, C> AsyncQuery<(), C> for Ignore<E>
where
    E: Endpoint + Sync,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        let is_authenicated = self.endpoint.is_authenticated();
        let endpoint = self.endpoint.endpoint();

        // Build the URL.
        let mut url = client.rest_endpoint(&endpoint, is_authenicated)?;

        // Add query parameters to the URL.
        self.endpoint.parameters().add_to_url(&mut url);

        let request_builder = Request::builder()
            .method(self.endpoint.method())
            .uri(url_to_http_uri(url));

        // Add the body to the request if any.
        let (request_builder, data) = if let Some((mime, data)) = self.endpoint.body() {
            (request_builder.header(header::CONTENT_TYPE, mime), data)
        } else {
            (request_builder, Vec::new())
        };

        // // Send off the request
        // let rsp = client
        //     .rest_async(request_builder, data, is_authenicated.then_some(endpoint))
        //     .await?;

        // // Check the response status and extract errors if needed.
        // let status = rsp.status();

        // if !status.is_success() {
        //     // For debug purposes try to deseralize the error.
        //     let v = serde_json::from_slice(rsp.body()).map_err(|_e| ApiError::ServerError {
        //         status,
        //         source: ServerError::InvalidJson {
        //             data: rsp.body().into_iter().copied().collect(),
        //         },
        //     })?;

        //     return Err(ApiError::ServerError {
        //         status,
        //         source: ServerError::NotSuccess { obj: v },
        //     });
        // }

        // Skip the deserialization process.
        Ok(())
    }
}

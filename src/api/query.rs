use async_trait::async_trait;
use http::Uri;
use url::Url;

use crate::api::{
    client::{AsyncClient, Client},
    error::ApiError,
};

pub fn url_to_http_uri(url: Url) -> Uri {
    url.as_str().parse::<Uri>().unwrap()
}

/// A trait which represents a query which may be made to the Bitfinex REST API.
pub trait Query<T, C>
where
    C: Client,
{
    /// Perform the query against the client.
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

/// A trait which represents an asynchronous query which may be made to the Bitfinex REST API.
#[async_trait]
pub trait AsyncQuery<T, C>
where
    C: AsyncClient,
{
    /// Perform the query asynchronously against the client.
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

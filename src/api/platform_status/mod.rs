use super::endpoint::Endpoint;
use derive_builder::Builder;

use http::Method;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Builder)]
pub struct PlatformStatus {}

impl PlatformStatus {
    pub fn builder() -> PlatformStatusBuilder {
        PlatformStatusBuilder::default()
    }
}

impl Endpoint for PlatformStatus {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> &'static str {
        "platform/status"
    }
}

#[derive(Debug, Deserialize)]
pub struct PlatformStatusResp(Vec<u8>);

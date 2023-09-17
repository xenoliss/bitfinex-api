use derive_builder::Builder;
use http::Method;
use serde::Serialize;

use crate::api::endpoint::Endpoint;

use super::types::Order;

#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct RetrieveOrdersBySymbol<'a> {
    symbol: &'a str,
    #[serde(rename(serialize = "id"))]
    #[builder(default)]
    ids: Option<Vec<u32>>,
}

impl<'a> RetrieveOrdersBySymbol<'a> {
    pub fn builder() -> RetrieveOrdersBySymbolBuilder<'a> {
        RetrieveOrdersBySymbolBuilder::default()
    }
}

impl<'a> Endpoint for RetrieveOrdersBySymbol<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("v2/auth/r/orders/{}", self.symbol)
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn body(&self) -> Option<(&'static str, Vec<u8>)> {
        let body = serde_json::to_string(self).unwrap();
        Some(("application/json", body.into_bytes()))
    }
}

pub type RetrieveOrdersBySymbolResp = Vec<Order>;

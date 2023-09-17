use derive_builder::Builder;
use http::Method;
use serde::Serialize;

use crate::api::endpoint::Endpoint;

use super::types::Order;

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct RetrieveOrdersBySymbol<'a> {
    symbol: &'a str,
    #[builder(default)]
    ids: Option<Vec<u64>>,
}

impl<'a> RetrieveOrdersBySymbol<'a> {
    pub fn builder() -> RetrieveOrdersBySymbolBuilder<'a> {
        RetrieveOrdersBySymbolBuilder::default()
    }

    fn json_body(&self) -> String {
        #[derive(Debug, Serialize)]
        pub struct JsonParams<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: &'a Option<Vec<u64>>,
        }

        let p = JsonParams { id: &self.ids };

        serde_json::to_string(&p).unwrap()
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
        Some(("application/json", self.json_body().into_bytes()))
    }
}

pub type RetrieveOrdersBySymbolResp = Vec<Order>;

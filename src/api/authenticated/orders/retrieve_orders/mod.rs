use derive_builder::Builder;
use http::Method;
use serde::Serialize;

use crate::api::endpoint::Endpoint;

use super::types::Order;

#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct RetrieveOrders {
    #[serde(rename(serialize = "id"))]
    #[builder(default)]
    ids: Option<Vec<u32>>,
}

impl RetrieveOrders {
    pub fn builder() -> RetrieveOrdersBuilder {
        RetrieveOrdersBuilder::default()
    }
}

impl Endpoint for RetrieveOrders {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("v2/auth/r/orders")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn body(&self) -> Option<(&'static str, Vec<u8>)> {
        let body = serde_json::to_string(self).unwrap();
        Some(("application/json", body.into_bytes()))
    }
}

pub type RetrieveOrdersResp = Vec<Order>;

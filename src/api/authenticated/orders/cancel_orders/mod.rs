use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::api::{authenticated::orders::types::OrderRaw, endpoint::Endpoint};

use super::types::Order;

#[derive(Debug, Builder, Serialize)]
pub struct CancelOrders {
    #[serde(rename(serialize = "id"))]
    ids: Vec<u64>,
}

impl CancelOrders {
    pub fn builder() -> CancelOrdersBuilder {
        CancelOrdersBuilder::default()
    }
}

impl Endpoint for CancelOrders {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("v2/auth/w/order/cancel")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn body(&self) -> Option<(&'static str, Vec<u8>)> {
        let body = serde_json::to_string(self).unwrap();
        Some(("application/json", body.into_bytes()))
    }
}

#[derive(Debug)]
pub struct CancelOrdersResp {
    pub mts: u64,
    pub ty: String,
    pub message_id: u64,
    pub orders: Vec<Order>,
    pub code: u64,
    pub status: String,
    pub text: String,
}

impl<'de> Deserialize<'de> for CancelOrdersResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct CancelOrdersRawResp(
            u64,
            String,
            u64,
            Option<()>,
            Vec<OrderRaw>,
            u64,
            String,
            String,
        );

        impl From<CancelOrdersRawResp> for CancelOrdersResp {
            fn from(value: CancelOrdersRawResp) -> Self {
                let CancelOrdersRawResp(mts, ty, message_id, _, orders, code, status, text) = value;

                Self {
                    mts,
                    ty,
                    message_id,
                    orders: orders.into_iter().map(|order| order.into()).collect(),
                    code,
                    status,
                    text,
                }
            }
        }

        let raw = CancelOrdersRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::api::{authenticated::orders::types::OrderRaw, common::PlaceHolder, endpoint::Endpoint};

use super::types::Order;

#[derive(Debug, Clone)]
pub enum CancelOrdersType {
    OnlyIds(Vec<u64>),
    All,
}

#[derive(Debug, Builder)]
pub struct CancelOrders {
    cancel_orders_type: CancelOrdersType,
}

impl CancelOrders {
    pub fn builder() -> CancelOrdersBuilder {
        CancelOrdersBuilder::default()
    }

    fn json_body(&self) -> String {
        match &self.cancel_orders_type {
            CancelOrdersType::OnlyIds(ids) => {
                #[derive(Debug, Serialize)]
                pub struct JsonParams<'a> {
                    id: &'a Vec<u64>,
                }

                let p = JsonParams { id: ids };

                serde_json::to_string(&p).unwrap()
            }
            CancelOrdersType::All => String::from("{\"all\": \"1\"}"),
        }
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
        Some(("application/json", self.json_body().into_bytes()))
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
            PlaceHolder,
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

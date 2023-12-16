use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::api::endpoint::Endpoint;

use super::types::Order;
use super::types::OrderRaw;

/// https://docs.bitfinex.com/reference/rest-auth-orders-history
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct OrdersHistory {
    #[builder(default)]
    start: Option<u64>,
    #[builder(default)]
    end: Option<u64>,
    #[builder(default)]
    limit: Option<u64>,
    #[builder(default)]
    id: Option<u64>,
}

impl OrdersHistory {
    pub fn builder() -> OrdersHistoryBuilder {
        OrdersHistoryBuilder::default()
    }

    fn json_body(&self) -> String {
        #[serde_as]
        #[derive(Debug, Serialize)]
        pub struct JsonParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
            start: Option<u64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
            end: Option<u64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
            limit: Option<u64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<u64>,
        }

        let p = JsonParams {
            start: self.start,
            end: self.end,
            limit: self.limit,
            id: self.id,
        };

        serde_json::to_string(&p).unwrap()
    }
}

impl Endpoint for OrdersHistory {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("v2/auth/r/orders/hist")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn body(&self) -> Option<(&'static str, Vec<u8>)> {
        Some(("application/json", self.json_body().into_bytes()))
    }
}

#[derive(Debug)]
pub struct OrdersHistoryResp {
    pub orders: Vec<Order>,
}

impl<'de> Deserialize<'de> for OrdersHistoryResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct OrdersHistoryRawResp(Vec<OrderRaw>);

        impl From<OrdersHistoryRawResp> for OrdersHistoryResp {
            fn from(value: OrdersHistoryRawResp) -> Self {
                let OrdersHistoryRawResp(orders) = value;

                Self {
                    orders: orders.into_iter().map(|order| order.into()).collect(),
                }
            }
        }

        let raw = OrdersHistoryRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

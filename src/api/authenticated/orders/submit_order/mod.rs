use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::api::{authenticated::orders::types::OrderRaw, endpoint::Endpoint};

use super::types::{Order, OrderType};

#[serde_as]
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct SubmitOrder<'a> {
    #[serde(rename(serialize = "type"))]
    ty: OrderType,
    symbol: &'a str,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    amount: f64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    price: f64,
    lev: Option<u8>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    price_trailing: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    price_aux_limit: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    price_oco_stop: Option<f64>,
    gid: Option<u64>,
    cid: Option<u64>,
    flags: Option<u64>,
    tif: Option<String>,
}

impl<'a> SubmitOrder<'a> {
    pub fn builder() -> SubmitOrderBuilder<'a> {
        SubmitOrderBuilder::default()
    }
}

impl<'a> Endpoint for SubmitOrder<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("v2/auth/w/order/submit")
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
pub struct SubmitOrderResp {
    pub mts: u64,
    pub ty: String,
    pub message_id: u64,
    pub order: Order,
    pub code: u64,
    pub status: String,
    pub text: String,
}

impl<'de> Deserialize<'de> for SubmitOrderResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct SubmitOrderRawResp(u64, String, u64, Option<()>, OrderRaw, u64, String, String);

        impl From<SubmitOrderRawResp> for SubmitOrderResp {
            fn from(value: SubmitOrderRawResp) -> Self {
                let SubmitOrderRawResp(mts, ty, message_id, _, order, code, status, text) = value;

                Self {
                    mts,
                    ty,
                    message_id,
                    order: order.into(),
                    code,
                    status,
                    text,
                }
            }
        }

        let raw = SubmitOrderRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

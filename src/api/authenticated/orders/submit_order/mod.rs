use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::api::{authenticated::orders::types::OrderRaw, endpoint::Endpoint};

use super::types::{Order, OrderType};

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct SubmitOrder<'a> {
    ty: OrderType,
    symbol: &'a str,
    amount: f64,
    price: f64,
    #[builder(default)]
    lev: Option<u8>,
    #[builder(default)]
    price_trailing: Option<f64>,
    #[builder(default)]
    price_aux_limit: Option<f64>,
    #[builder(default)]
    price_oco_stop: Option<f64>,
    #[builder(default)]
    gid: Option<u64>,
    #[builder(default)]
    cid: Option<u64>,
    #[builder(default)]
    flags: Option<u64>,
    #[builder(default)]
    tif: Option<&'a str>,
}

impl<'a> SubmitOrder<'a> {
    pub fn builder() -> SubmitOrderBuilder<'a> {
        SubmitOrderBuilder::default()
    }

    fn json_body(&self) -> String {
        #[serde_as]
        #[derive(Debug, Serialize)]
        pub struct JsonParams<'a> {
            #[serde(rename(serialize = "type"))]
            ty: OrderType,
            symbol: &'a str,
            #[serde_as(as = "serde_with::DisplayFromStr")]
            amount: f64,
            #[serde_as(as = "serde_with::DisplayFromStr")]
            price: f64,
            #[serde(skip_serializing_if = "Option::is_none")]
            lev: Option<u8>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
            price_trailing: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
            price_aux_limit: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
            price_oco_stop: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            gid: Option<u64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            cid: Option<u64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            flags: Option<u64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tif: &'a Option<&'a str>,
        }

        let p = JsonParams {
            ty: self.ty,
            symbol: self.symbol,
            amount: self.amount,
            price: self.price,
            lev: self.lev,
            price_trailing: self.price_trailing,
            price_aux_limit: self.price_aux_limit,
            price_oco_stop: self.price_oco_stop,
            gid: self.gid,
            cid: self.cid,
            flags: self.flags,
            tif: &self.tif,
        };

        serde_json::to_string(&p).unwrap()
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
        Some(("application/json", self.json_body().into_bytes()))
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

use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use super::common::Len;
use crate::api::{endpoint::Endpoint, params::QueryParams};

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct RawBook<'a> {
    symbol: &'a str,
    #[builder(default)]
    len: Option<Len>,
}

impl<'a> RawBook<'a> {
    pub fn builder() -> RawBookBuilder<'a> {
        RawBookBuilder::default()
    }
}

impl<'a> Endpoint for RawBook<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("book/{}/R0", self.symbol)
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("len", self.len.map(|len| len as u8));
        params
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RawBookResp {
    RawBookTradingResp(Vec<RawBookTradingResp>),
    RawBookFundingResp(Vec<RawBookFundingResp>),
}

#[derive(Debug)]
pub struct RawBookFundingResp {
    pub offer_id: u64,
    pub period: u64,
    pub rate: f64,
    pub amount: f64,
}

#[derive(Debug)]
pub struct RawBookTradingResp {
    pub order_id: u64,
    pub price: f64,
    pub amount: f64,
}

impl<'de> Deserialize<'de> for RawBookTradingResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct RawBookTradingRawResp(u64, f64, f64);

        impl From<RawBookTradingRawResp> for RawBookTradingResp {
            fn from(value: RawBookTradingRawResp) -> Self {
                Self {
                    order_id: value.0,
                    price: value.1,
                    amount: value.2,
                }
            }
        }

        let raw = RawBookTradingRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

impl<'de> Deserialize<'de> for RawBookFundingResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct RawBookFundingRawResp(u64, u64, f64, f64);

        impl From<RawBookFundingRawResp> for RawBookFundingResp {
            fn from(value: RawBookFundingRawResp) -> Self {
                Self {
                    offer_id: value.0,
                    period: value.1,
                    rate: value.2,
                    amount: value.2,
                }
            }
        }

        let raw = RawBookFundingRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

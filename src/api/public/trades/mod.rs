use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{common::Sort, endpoint::Endpoint, params::QueryParams};

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Trades<'a> {
    symbol: &'a str,
    #[builder(default)]
    limit: Option<u16>,
    #[builder(default)]
    sort: Option<Sort>,
    #[builder(default)]
    start: Option<u64>,
    #[builder(default)]
    end: Option<u64>,
}

impl<'a> Trades<'a> {
    pub fn builder() -> TradesBuilder<'a> {
        TradesBuilder::default()
    }
}

impl<'a> Endpoint for Trades<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("v2/trades/{}/hist", self.symbol)
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params
            .push_opt("limit", self.limit)
            .push_opt("sort", self.sort.map(|e| e as i8))
            .push_opt("start", self.start)
            .push_opt("end", self.end);
        params
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TradesResp {
    TradesTradingResp(Vec<TradeTradingResp>),
    TradesFundingResp(Vec<TradeFundingResp>),
}

#[derive(Debug)]
pub struct TradeTradingResp {
    pub id: u64,
    pub mts: u64,
    pub amount: f64,
    pub price: f64,
}

#[derive(Debug)]
pub struct TradeFundingResp {
    pub id: u64,
    pub mts: u64,
    pub amount: f64,
    pub rate: f64,
    pub period: u64,
}

impl<'de> Deserialize<'de> for TradeTradingResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct TradeTradingRawResp(u64, u64, f64, f64);

        impl From<TradeTradingRawResp> for TradeTradingResp {
            fn from(value: TradeTradingRawResp) -> Self {
                let TradeTradingRawResp(id, mts, amount, price) = value;

                Self {
                    id,
                    mts,
                    amount,
                    price,
                }
            }
        }

        let raw = TradeTradingRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

impl<'de> Deserialize<'de> for TradeFundingResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct TradeFundingRawResp(u64, u64, f64, f64, u64);

        impl From<TradeFundingRawResp> for TradeFundingResp {
            fn from(value: TradeFundingRawResp) -> Self {
                let TradeFundingRawResp(id, mts, amount, rate, period) = value;

                Self {
                    id,
                    mts,
                    amount,
                    rate,
                    period,
                }
            }
        }

        let raw = TradeFundingRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{common::Symbols, endpoint::Endpoint, params::QueryParams};

#[derive(Debug, Clone, Builder)]
pub struct Tickers<'a> {
    symbols: Symbols<'a>,
}

impl<'a> Tickers<'a> {
    pub fn builder() -> TickersBuilder<'a> {
        TickersBuilder::default()
    }
}

impl<'a> Endpoint for Tickers<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        String::from("v2/tickers")
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push("symbols", self.symbols.as_query_string());
        params
    }
}

pub type TickersResp = Vec<TickerResp>;

#[derive(Debug)]
pub enum TickerResp {
    TickersTradingResp {
        symbol: String,
        bid: f64,
        bid_size: f64,
        ask: f64,
        ask_size: f64,
        daily_change: f64,
        daily_change_relative: f64,
        last_price: f64,
        volume: f64,
        high: f64,
        low: f64,
    },
    TickersFundingResp {
        symbol: String,
        frr: f64,
        bid: f64,
        bid_period: u8,
        bid_size: f64,
        ask: f64,
        ask_period: u8,
        ask_size: f64,
        daily_change: f64,
        daily_change_relative: f64,
        last_price: f64,
        volume: f64,
        high: f64,
        low: f64,
        frr_amount_available: f64,
    },
}

impl<'de> Deserialize<'de> for TickerResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        #[serde(untagged)]
        enum TickersRawResp {
            TickersTradingRawResp(String, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64),
            TickersFundingRawResp(
                String,
                f64,
                f64,
                u8,
                f64,
                f64,
                u8,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                Option<()>,
                Option<()>,
                f64,
            ),
        }

        impl From<TickersRawResp> for TickerResp {
            fn from(value: TickersRawResp) -> Self {
                match value {
                    TickersRawResp::TickersTradingRawResp(
                        symbol,
                        bid,
                        bid_size,
                        ask,
                        ask_size,
                        daily_change,
                        daily_change_relative,
                        last_price,
                        volume,
                        high,
                        low,
                    ) => Self::TickersTradingResp {
                        symbol,
                        bid,
                        bid_size,
                        ask,
                        ask_size,
                        daily_change,
                        daily_change_relative,
                        last_price,
                        volume,
                        high,
                        low,
                    },
                    TickersRawResp::TickersFundingRawResp(
                        symbol,
                        frr,
                        bid,
                        bid_period,
                        bid_size,
                        ask,
                        ask_period,
                        ask_size,
                        daily_change,
                        daily_change_relative,
                        last_price,
                        volume,
                        high,
                        low,
                        _,
                        _,
                        frr_amount_available,
                    ) => Self::TickersFundingResp {
                        symbol,
                        frr,
                        bid,
                        bid_period,
                        bid_size,
                        ask,
                        ask_period,
                        ask_size,
                        daily_change,
                        daily_change_relative,
                        last_price,
                        volume,
                        high,
                        low,
                        frr_amount_available,
                    },
                }
            }
        }

        let raw = TickersRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

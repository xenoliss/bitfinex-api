use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use super::endpoint::Endpoint;

#[derive(Debug, Clone, Copy, Builder)]
pub struct Ticker<'a> {
    symbol: &'a str,
}

impl<'a> Ticker<'a> {
    pub fn builder() -> TickerBuilder<'a> {
        TickerBuilder::default()
    }
}

impl<'a> Endpoint for Ticker<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("ticker/{}", self.symbol)
    }
}

#[derive(Debug)]
pub enum TickerResp {
    TickerTradingResp {
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
    TickerFundingResp {
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
        enum TickerRawResp {
            TickerTradingRawResp(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64),
            TickerFundingRawResp(
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

        impl From<TickerRawResp> for TickerResp {
            fn from(value: TickerRawResp) -> Self {
                match value {
                    TickerRawResp::TickerTradingRawResp(
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
                    ) => Self::TickerTradingResp {
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
                    TickerRawResp::TickerFundingRawResp(
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
                    ) => Self::TickerFundingResp {
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

        let raw = TickerRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

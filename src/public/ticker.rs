use std::fmt::Display;

use serde::Deserialize;

use crate::public::PUB_ENDPOINT;

use super::BitfinexRequest;

#[derive(Debug)]
pub enum Ticker {
    TradingPairTicker {
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
    FundingTicker {
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

impl<'de> Deserialize<'de> for Ticker {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        #[serde(untagged)]
        pub enum TickerRaw {
            TradingPairTickerRaw(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64),
            FundingTickerRaw(
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

        impl From<TickerRaw> for Ticker {
            fn from(value: TickerRaw) -> Self {
                match value {
                    TickerRaw::TradingPairTickerRaw(
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
                    ) => Self::TradingPairTicker {
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
                    TickerRaw::FundingTickerRaw(
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
                    ) => Self::FundingTicker {
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

        let raw = TickerRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

#[derive(Debug)]
pub struct GetTickerReq<'a> {
    pub symbol: &'a str,
}

impl<'a> BitfinexRequest for GetTickerReq<'a> {
    fn path(&self) -> String {
        format!("ticker/{}", self.symbol)
    }
}

impl<'a> Display for GetTickerReq<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}

pub async fn get_ticker(request: GetTickerReq<'_>) -> Ticker {
    reqwest::get(format!("{PUB_ENDPOINT}/{request}"))
        .await
        .unwrap()
        .json::<Ticker>()
        .await
        .unwrap()
}

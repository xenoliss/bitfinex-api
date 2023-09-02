use std::fmt::Display;

use serde::Deserialize;

use crate::public::PUB_ENDPOINT;

use super::BitfinexRequest;

#[derive(Debug)]
pub enum TickerWithSymbol {
    TradingPairTickerWithSymbol {
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
    FundingTickerWithSymbol {
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

impl<'de> Deserialize<'de> for TickerWithSymbol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        #[serde(untagged)]
        pub enum TickerWithSymbolRaw {
            TradingPairTickerWithSymbolRaw(
                String,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
            ),
            FundingTickerWithSymbolRaw(
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

        impl From<TickerWithSymbolRaw> for TickerWithSymbol {
            fn from(value: TickerWithSymbolRaw) -> Self {
                match value {
                    TickerWithSymbolRaw::TradingPairTickerWithSymbolRaw(
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
                    ) => Self::TradingPairTickerWithSymbol {
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
                    TickerWithSymbolRaw::FundingTickerWithSymbolRaw(
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
                    ) => Self::FundingTickerWithSymbol {
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

        let raw = TickerWithSymbolRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

#[derive(Debug)]
pub enum GetTickersReq<'a> {
    All,
    Only(Vec<&'a str>),
}

impl<'a> BitfinexRequest for GetTickersReq<'a> {
    fn path(&self) -> String {
        "/tickers".into()
    }

    fn url_params(&self) -> Option<String> {
        match self {
            GetTickersReq::All => Some("symbols=ALL".into()),
            GetTickersReq::Only(symbols) => {
                let symbols = symbols
                    .iter()
                    .map(|symbol| symbol.to_string())
                    .collect::<Vec<_>>()
                    .join(",");

                Some(format!("symbols={symbols}"))
            }
        }
    }
}

impl<'a> Display for GetTickersReq<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}?{}", self.path(), self.url_params().unwrap())
    }
}

pub async fn get_tickers(request: GetTickersReq<'_>) -> Vec<TickerWithSymbol> {
    reqwest::get(format!("{PUB_ENDPOINT}/{request}"))
        .await
        .unwrap()
        .json::<Vec<TickerWithSymbol>>()
        .await
        .unwrap()
}

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{BitfinexRequest, PUB_ENDPOINT};

#[derive(Debug)]
pub struct TickerHistory {
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub mts: u64,
}

impl<'de> Deserialize<'de> for TickerHistory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        pub struct TickerHistoryRaw(
            String,
            f64,
            Option<()>,
            f64,
            Option<()>,
            Option<()>,
            Option<()>,
            Option<()>,
            Option<()>,
            Option<()>,
            Option<()>,
            Option<()>,
            u64,
        );

        impl From<TickerHistoryRaw> for TickerHistory {
            fn from(value: TickerHistoryRaw) -> Self {
                Self {
                    symbol: value.0,
                    bid: value.1,
                    ask: value.3,
                    mts: value.12,
                }
            }
        }

        let raw: TickerHistoryRaw = TickerHistoryRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

#[derive(Debug, Serialize)]
pub struct GetTickersHistoryReqFilter {
    pub limit: Option<u8>,
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[derive(Debug)]
pub enum GetTickersHistoryReq<'a> {
    All {
        filter: Option<GetTickersHistoryReqFilter>,
    },
    Only {
        symbols: Vec<&'a str>,
        filter: Option<GetTickersHistoryReqFilter>,
    },
}

impl<'a> BitfinexRequest for GetTickersHistoryReq<'a> {
    fn path(&self) -> String {
        "tickers/hist".into()
    }

    fn url_params(&self) -> Option<String> {
        match self {
            GetTickersHistoryReq::All { filter } => {
                let filter = serde_qs::to_string(filter).unwrap();
                Some(format!("symbols=ALL&{filter}"))
            }
            GetTickersHistoryReq::Only { symbols, filter } => {
                let symbols = symbols
                    .iter()
                    .map(|symbol| symbol.to_string())
                    .collect::<Vec<_>>()
                    .join(",");

                let filter: String = serde_qs::to_string(filter).unwrap();
                Some(format!("symbols={symbols}&{filter}"))
            }
        }
    }
}

impl<'a> Display for GetTickersHistoryReq<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}?{}", self.path(), self.url_params().unwrap())
    }
}

pub async fn get_tickers_history(request: GetTickersHistoryReq<'_>) -> Vec<TickerHistory> {
    reqwest::get(format!("{PUB_ENDPOINT}/{request}"))
        .await
        .unwrap()
        .json::<Vec<TickerHistory>>()
        .await
        .unwrap()
}

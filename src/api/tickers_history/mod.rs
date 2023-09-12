use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use super::{common::Symbols, endpoint::Endpoint, params::QueryParams};

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct TickersHistory<'a> {
    symbols: Symbols<'a>,
    #[builder(default)]
    limit: Option<u8>,
    #[builder(default)]
    start: Option<u64>,
    #[builder(default)]
    end: Option<u64>,
}

impl<'a> TickersHistory<'a> {
    pub fn builder() -> TickersHistoryBuilder<'a> {
        TickersHistoryBuilder::default()
    }
}

impl<'a> Endpoint for TickersHistory<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        "tickers/hist".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params
            .push("symbols", self.symbols.as_query_string())
            .push_opt("limit", self.limit)
            .push_opt("start", self.start)
            .push_opt("end", self.end);
        params
    }
}

pub type TickersHistoryResp = Vec<TickerHistoryResp>;

#[derive(Debug)]
pub struct TickerHistoryResp {
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub mts: u64,
}

impl<'de> Deserialize<'de> for TickerHistoryResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct TickerHistoryRawResp(
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

        impl From<TickerHistoryRawResp> for TickerHistoryResp {
            fn from(value: TickerHistoryRawResp) -> Self {
                Self {
                    symbol: value.0,
                    bid: value.1,
                    ask: value.3,
                    mts: value.12,
                }
            }
        }

        let raw: TickerHistoryRawResp = TickerHistoryRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

use std::fmt::Display;

use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{
    common::{Section, Sort, TimeFrame},
    endpoint::Endpoint,
    params::QueryParams,
};

#[derive(Debug, Clone, Copy)]
pub enum AvailableCandles<'a> {
    TradingCandles {
        time_frame: TimeFrame,
        trading_pair: &'a str,
    },
    FundingCandles {
        time_frame: TimeFrame,
        currency: &'a str,
        period: u8,
    },
    AggregateFundingCandles {
        time_frame: TimeFrame,
        currency: &'a str,
        aggregation: u8,
        period_start: u8,
        period_end: u8,
    },
}

impl<'a> Display for AvailableCandles<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AvailableCandles::TradingCandles {
                time_frame,
                trading_pair,
            } => write!(f, "trade:{time_frame}:{trading_pair}"),
            AvailableCandles::FundingCandles {
                time_frame,
                currency,
                period,
            } => write!(f, "trade:{time_frame}:{currency}:p{period}"),
            AvailableCandles::AggregateFundingCandles {
                time_frame,
                currency,
                aggregation,
                period_start,
                period_end,
            } => write!(
                f,
                "trade:{time_frame}:{currency}:a{aggregation}:p{period_start}:p{period_end}"
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Candles<'a> {
    candles: AvailableCandles<'a>,
    section: Section,
    #[builder(default)]
    sort: Option<Sort>,
    #[builder(default)]
    start: Option<u64>,
    #[builder(default)]
    end: Option<u64>,
    #[builder(default)]
    limit: Option<u64>,
}

impl<'a> Candles<'a> {
    pub fn builder() -> CandlesBuilder<'a> {
        CandlesBuilder::default()
    }
}

impl<'a> Endpoint for Candles<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("v2/candles/{}/{}", self.candles, self.section)
    }

    fn is_authenticated(&self) -> bool {
        false
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params
            .push_opt("sort", self.sort.map(|sort| sort as i8))
            .push_opt("start", self.start)
            .push_opt("end", self.end)
            .push_opt("limit", self.limit);
        params
    }
}

pub type LastCandlesResp = CandleResp;
pub type HistCandlesResp = Vec<CandleResp>;

#[derive(Debug)]
pub struct CandleResp {
    pub mts: u64,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

impl<'de> Deserialize<'de> for CandleResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct CandlesRawResp(u64, f64, f64, f64, f64, f64);

        impl From<CandlesRawResp> for CandleResp {
            fn from(value: CandlesRawResp) -> Self {
                let CandlesRawResp(mts, open, close, high, low, volume) = value;

                Self {
                    mts,
                    open,
                    close,
                    high,
                    low,
                    volume,
                }
            }
        }

        let raw = CandlesRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

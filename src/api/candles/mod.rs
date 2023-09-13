use std::fmt::Display;

use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use super::{
    common::{Section, Sort},
    endpoint::Endpoint,
    params::QueryParams,
};

#[derive(Debug, Clone, Copy)]
pub enum TimeFrame {
    OneMin,
    FiveMins,
    FifteenMins,
    ThirtyMins,
    OneHour,
    ThreeHours,
    SixHours,
    TwelveHours,
    OneDay,
    OneWeek,
    FourteenDays,
    OneMonth,
}

impl Display for TimeFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeFrame::OneMin => write!(f, "1m"),
            TimeFrame::FiveMins => write!(f, "5m"),
            TimeFrame::FifteenMins => write!(f, "15m"),
            TimeFrame::ThirtyMins => write!(f, "30m"),
            TimeFrame::OneHour => write!(f, "1h"),
            TimeFrame::ThreeHours => write!(f, "3h"),
            TimeFrame::SixHours => write!(f, "6h"),
            TimeFrame::TwelveHours => write!(f, "12h"),
            TimeFrame::OneDay => write!(f, "1D"),
            TimeFrame::OneWeek => write!(f, "1W"),
            TimeFrame::FourteenDays => write!(f, "14D"),
            TimeFrame::OneMonth => write!(f, "1M"),
        }
    }
}

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
        format!("candles/{}/{}", self.candles, self.section)
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

pub type LastCandlesResp = CandlesResp;
pub type HistCandlesResp = Vec<CandlesResp>;

#[derive(Debug)]
pub struct CandlesResp {
    pub mts: u64,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

impl<'de> Deserialize<'de> for CandlesResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct CandlesRespRaw(u64, f64, f64, f64, f64, f64);

        impl From<CandlesRespRaw> for CandlesResp {
            fn from(value: CandlesRespRaw) -> Self {
                Self {
                    mts: value.0,
                    open: value.1,
                    close: value.2,
                    high: value.3,
                    low: value.4,
                    volume: value.5,
                }
            }
        }

        let raw = CandlesRespRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use std::fmt::Display;

use crate::public::PUB_ENDPOINT;

use super::BitfinexRequest;

#[derive(Debug, Deserialize)]
struct CandlesRaw(u64, f64, f64, f64, f64, f64);

#[derive(Debug)]
pub struct Candles {
    pub mts: u64,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

impl From<CandlesRaw> for Candles {
    fn from(value: CandlesRaw) -> Self {
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

#[derive(Debug)]
pub enum TimeFrame {
    OneMin,
    FiveMins,
    FifteenMins,
    ThirthyMins,
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
        f.write_str(match self {
            TimeFrame::OneMin => "1m",
            TimeFrame::FiveMins => "5m",
            TimeFrame::FifteenMins => "15m",
            TimeFrame::ThirthyMins => "30m",
            TimeFrame::OneHour => "1h",
            TimeFrame::ThreeHours => "3h",
            TimeFrame::SixHours => "6h",
            TimeFrame::TwelveHours => "12h",
            TimeFrame::OneDay => "1D",
            TimeFrame::OneWeek => "1W",
            TimeFrame::FourteenDays => "14D",
            TimeFrame::OneMonth => "1M",
        })
    }
}

#[derive(Debug)]
pub enum CandleType<'a> {
    TradingPair {
        time_frame: TimeFrame,
        pair: &'a str,
    },
    FundingCurrency {
        time_frame: TimeFrame,
        currency: &'a str,
        period: u8,
    },
}

impl<'a> Display for CandleType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CandleType::TradingPair { time_frame, pair } => write!(f, "{time_frame}:{pair}"),
            CandleType::FundingCurrency {
                time_frame,
                currency,
                period,
            } => write!(f, "{time_frame}:{currency}:p{period}"),
        }
    }
}

#[derive(Debug, Serialize_repr)]
#[repr(i8)]
pub enum SortType {
    Desc = -1,
    Asc = 1,
}

#[derive(Debug, Serialize)]
pub struct GetCandlesReqFilter {
    #[serde(rename = "sort")]
    pub sort_mts: Option<SortType>,
    #[serde(rename = "start")]
    pub start_mts: Option<u64>,
    #[serde(rename = "end")]
    pub end_mts: Option<u64>,
    pub limit: Option<u32>,
}

#[derive(Debug)]
pub struct GetCandlesReq<'a> {
    pub candle_type: CandleType<'a>,
    pub filter: Option<GetCandlesReqFilter>,
}

impl<'a> BitfinexRequest for GetCandlesReq<'a> {
    fn path(&self) -> String {
        let candle_type = &self.candle_type;
        format!("candles/trade:{candle_type}/hist")
    }

    fn url_params(&self) -> Option<String> {
        Some(serde_qs::to_string(&self.filter).unwrap())
    }
}

impl<'a> Display for GetCandlesReq<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}?{}", self.path(), self.url_params().unwrap())
    }
}

pub async fn get_candles(request: GetCandlesReq<'_>) -> Vec<Candles> {
    reqwest::get(format!("{PUB_ENDPOINT}/{request}"))
        .await
        .unwrap()
        .json::<Vec<CandlesRaw>>()
        .await
        .unwrap()
        .into_iter()
        .map(|raw| raw.into())
        .collect()
}

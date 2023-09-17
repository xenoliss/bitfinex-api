use std::fmt::Display;

use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{
    common::{Section, Sort},
    endpoint::Endpoint,
    params::QueryParams,
};

#[derive(Debug, Clone, Copy)]
pub enum Size {
    OneMin,
    ThirtyMins,
    OneDay,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::OneMin => write!(f, "1m"),
            Size::ThirtyMins => write!(f, "30m"),
            Size::OneDay => write!(f, "1d"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Long,
    Short,
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Long => write!(f, "long"),
            Side::Short => write!(f, "short"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum KeyArgs<'a> {
    PosSize { sym: &'a str, side: Side },
    FundingSize { sym: &'a str },
    CreditsSize { sym: &'a str },
    CreditsSizeSym { sym: &'a str, pair: &'a str },
    VolOneDay { platform: &'a str },
    VolSevenDay { platform: &'a str },
    VolThirtyDay { platform: &'a str },
    Vwamp { sym: &'a str },
}

impl<'a> Display for KeyArgs<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyArgs::PosSize { sym, side } => write!(f, "pos.size:1m:{}:{}", sym, side),
            KeyArgs::FundingSize { sym } => write!(f, "funding.size:1m:{}", sym),
            KeyArgs::CreditsSize { sym } => write!(f, "credits.size:1m:{}", sym),
            KeyArgs::CreditsSizeSym { sym, pair } => {
                write!(f, "credits.size.sym:1m:{}:{}", sym, pair)
            }
            KeyArgs::VolOneDay { platform } => write!(f, "vol.1d:30m:{}", platform),
            KeyArgs::VolSevenDay { platform } => write!(f, "vol.7d:30m:{}", platform),
            KeyArgs::VolThirtyDay { platform } => write!(f, "vol.30d:30m:{}", platform),
            KeyArgs::Vwamp { sym } => write!(f, "vwap:1d:{}", sym),
        }
    }
}

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Stats<'a> {
    key_args: KeyArgs<'a>,
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

impl<'a> Stats<'a> {
    pub fn builder() -> StatsBuilder<'a> {
        StatsBuilder::default()
    }
}

impl<'a> Endpoint for Stats<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("v2/stats1/{}/{}", self.key_args, self.section)
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

pub type LastStatsResp = StatsResp;
pub type HistStatsResp = Vec<StatsResp>;

#[derive(Debug)]
pub struct StatsResp {
    pub mts: u64,
    pub value: f64,
}

impl<'de> Deserialize<'de> for StatsResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct StatsRespRaw(u64, f64);

        impl From<StatsRespRaw> for StatsResp {
            fn from(value: StatsRespRaw) -> Self {
                let StatsRespRaw(mts, value) = value;

                Self { mts, value }
            }
        }

        let raw = StatsRespRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

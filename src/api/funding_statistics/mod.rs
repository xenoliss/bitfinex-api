use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use super::{endpoint::Endpoint, params::QueryParams};

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct FundingStatistics<'a> {
    symbol: &'a str,
    #[builder(default)]
    start: Option<u64>,
    #[builder(default)]
    end: Option<u64>,
    #[builder(default)]
    limit: Option<u64>,
}

impl<'a> FundingStatistics<'a> {
    pub fn builder() -> FundingStatisticsBuilder<'a> {
        FundingStatisticsBuilder::default()
    }
}

impl<'a> Endpoint for FundingStatistics<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("funding/stats/{}/hist", self.symbol)
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params
            .push_opt("start", self.start)
            .push_opt("end", self.end)
            .push_opt("limit", self.limit);
        params
    }
}

pub type FundingStatisticsResp = Vec<FundingStatisticsRespItem>;

#[derive(Debug)]
pub struct FundingStatisticsRespItem {
    pub mts: u64,
    pub frr: f64,
    pub avg_period: f64,
    pub funding_amount: f64,
    pub funding_amount_used: f64,
    pub funding_below_threshold: f64,
}

impl<'de> Deserialize<'de> for FundingStatisticsRespItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct FundingStatisticsRawRespItem(
            u64,
            Option<()>,
            Option<()>,
            f64,
            f64,
            Option<()>,
            Option<()>,
            f64,
            f64,
            Option<()>,
            Option<()>,
            f64,
        );

        impl From<FundingStatisticsRawRespItem> for FundingStatisticsRespItem {
            fn from(value: FundingStatisticsRawRespItem) -> Self {
                Self {
                    mts: value.0,
                    frr: value.3,
                    avg_period: value.4,
                    funding_amount: value.7,
                    funding_amount_used: value.8,
                    funding_below_threshold: value.11,
                }
            }
        }

        let raw = FundingStatisticsRawRespItem::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

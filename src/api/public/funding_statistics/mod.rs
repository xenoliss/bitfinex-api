use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{common::PlaceHolder, endpoint::Endpoint, params::QueryParams};

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
        format!("v2/funding/stats/{}/hist", self.symbol)
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
            PlaceHolder,
            PlaceHolder,
            f64,
            f64,
            PlaceHolder,
            PlaceHolder,
            f64,
            f64,
            PlaceHolder,
            PlaceHolder,
            f64,
        );

        impl From<FundingStatisticsRawRespItem> for FundingStatisticsRespItem {
            fn from(value: FundingStatisticsRawRespItem) -> Self {
                let FundingStatisticsRawRespItem(
                    mts,
                    _,
                    _,
                    frr,
                    avg_period,
                    _,
                    _,
                    funding_amount,
                    funding_amount_used,
                    _,
                    _,
                    funding_below_threshold,
                ) = value;

                Self {
                    mts,
                    frr,
                    avg_period,
                    funding_amount,
                    funding_amount_used,
                    funding_below_threshold,
                }
            }
        }

        let raw = FundingStatisticsRawRespItem::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

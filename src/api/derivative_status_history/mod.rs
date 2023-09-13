use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use super::{common::Sort, endpoint::Endpoint, params::QueryParams};

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct DerivativesStatusHistory<'a> {
    key: &'a str,
    #[builder(default)]
    sort: Option<Sort>,
    #[builder(default)]
    start: Option<u64>,
    #[builder(default)]
    end: Option<u64>,
    #[builder(default)]
    limit: Option<u64>,
}

impl<'a> DerivativesStatusHistory<'a> {
    pub fn builder() -> DerivativesStatusHistoryBuilder<'a> {
        DerivativesStatusHistoryBuilder::default()
    }
}

impl<'a> Endpoint for DerivativesStatusHistory<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("status/deriv/{}/hist", self.key)
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

pub type DerivativesStatusesHistoryResp = Vec<DerivativesStatusHistoryResp>;

#[derive(Debug)]
pub struct DerivativesStatusHistoryResp {
    pub mts: u64,
    pub derive_price: f64,
    pub spot_price: f64,
    pub insurrance_fund_balance: f64,
    pub next_funding_evt_mts: u64,
    pub next_funding_accrued: f64,
    pub next_funding_step: u64,
    pub current_funding: f64,
    pub mark_price: f64,
    pub open_interest: f64,
    pub clamp_min: f64,
    pub clamp_max: f64,
}

impl<'de> Deserialize<'de> for DerivativesStatusHistoryResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct DerivativesStatusHistoryRawResp(
            u64,
            Option<()>,
            f64,
            f64,
            Option<()>,
            f64,
            Option<()>,
            u64,
            f64,
            u64,
            Option<()>,
            f64,
            Option<()>,
            Option<()>,
            f64,
            Option<()>,
            Option<()>,
            f64,
            Option<()>,
            Option<()>,
            Option<()>,
            f64,
            f64,
        );

        impl From<DerivativesStatusHistoryRawResp> for DerivativesStatusHistoryResp {
            fn from(value: DerivativesStatusHistoryRawResp) -> Self {
                Self {
                    mts: value.0,
                    derive_price: value.2,
                    spot_price: value.3,
                    insurrance_fund_balance: value.5,
                    next_funding_evt_mts: value.7,
                    next_funding_accrued: value.8,
                    next_funding_step: value.9,
                    current_funding: value.11,
                    mark_price: value.14,
                    open_interest: value.17,
                    clamp_min: value.21,
                    clamp_max: value.22,
                }
            }
        }

        let raw: DerivativesStatusHistoryRawResp =
            DerivativesStatusHistoryRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

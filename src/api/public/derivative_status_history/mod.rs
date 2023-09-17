use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{common::Sort, endpoint::Endpoint, params::QueryParams};

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
        format!("v2/status/deriv/{}/hist", self.key)
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

pub type DerivativesStatusHistoryResp = Vec<DerivativeStatusHistoryResp>;

#[derive(Debug)]
pub struct DerivativeStatusHistoryResp {
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

impl<'de> Deserialize<'de> for DerivativeStatusHistoryResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct DerivativeStatusHistoryRawResp(
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

        impl From<DerivativeStatusHistoryRawResp> for DerivativeStatusHistoryResp {
            fn from(value: DerivativeStatusHistoryRawResp) -> Self {
                let DerivativeStatusHistoryRawResp(
                    mts,
                    _,
                    derive_price,
                    spot_price,
                    _,
                    insurrance_fund_balance,
                    _,
                    next_funding_evt_mts,
                    next_funding_accrued,
                    next_funding_step,
                    _,
                    current_funding,
                    _,
                    _,
                    mark_price,
                    _,
                    _,
                    open_interest,
                    _,
                    _,
                    _,
                    clamp_min,
                    clamp_max,
                ) = value;

                Self {
                    mts,
                    derive_price,
                    spot_price,
                    insurrance_fund_balance,
                    next_funding_evt_mts,
                    next_funding_accrued,
                    next_funding_step,
                    current_funding,
                    mark_price,
                    open_interest,
                    clamp_min,
                    clamp_max,
                }
            }
        }

        let raw: DerivativeStatusHistoryRawResp =
            DerivativeStatusHistoryRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{common::Symbols, endpoint::Endpoint, params::QueryParams};

#[derive(Debug, Clone, Builder)]
pub struct DerivativesStatus<'a> {
    keys: Symbols<'a>,
}

impl<'a> DerivativesStatus<'a> {
    pub fn builder() -> DerivativesStatusBuilder<'a> {
        DerivativesStatusBuilder::default()
    }
}

impl<'a> Endpoint for DerivativesStatus<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        String::from("v2/status/deriv")
    }

    fn is_authenticated(&self) -> bool {
        false
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push("keys", self.keys.as_query_string());
        params
    }
}

pub type DerivativesStatusResp = Vec<DerivativeStatusResp>;

#[derive(Debug)]
pub struct DerivativeStatusResp {
    pub key: String,
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

impl<'de> Deserialize<'de> for DerivativeStatusResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct DerivativeStatusRawResp(
            String,
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

        impl From<DerivativeStatusRawResp> for DerivativeStatusResp {
            fn from(value: DerivativeStatusRawResp) -> Self {
                let DerivativeStatusRawResp(
                    key,
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
                    key,
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

        let raw: DerivativeStatusRawResp = DerivativeStatusRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

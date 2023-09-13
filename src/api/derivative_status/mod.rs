use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use super::{common::Symbols, endpoint::Endpoint, params::QueryParams};

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
        String::from("status/deriv")
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
                Self {
                    key: value.0,
                    mts: value.1,
                    derive_price: value.3,
                    spot_price: value.4,
                    insurrance_fund_balance: value.6,
                    next_funding_evt_mts: value.8,
                    next_funding_accrued: value.9,
                    next_funding_step: value.10,
                    current_funding: value.12,
                    mark_price: value.15,
                    open_interest: value.18,
                    clamp_min: value.22,
                    clamp_max: value.23,
                }
            }
        }

        let raw: DerivativeStatusRawResp = DerivativeStatusRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

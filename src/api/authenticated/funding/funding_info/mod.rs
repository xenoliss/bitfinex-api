use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::endpoint::Endpoint;

#[derive(Debug, Clone, Copy, Builder)]
pub struct FundingInfo<'a> {
    symbol: &'a str,
}

impl<'a> FundingInfo<'a> {
    pub fn builder() -> FundingInfoBuilder<'a> {
        FundingInfoBuilder::default()
    }
}

impl<'a> Endpoint for FundingInfo<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("v2/auth/r/info/funding/{}", self.symbol)
    }

    fn is_authenticated(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct FundingInfoResp {
    pub symbol: String,
    pub yield_loan: f64,
    pub yield_lend: f64,
    pub duration_loan: f64,
    pub duration_lend: f64,
}

impl<'de> Deserialize<'de> for FundingInfoResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        pub struct FundingInfoRespRaw(String, String, (f64, f64, f64, f64));

        impl From<FundingInfoRespRaw> for FundingInfoResp {
            fn from(value: FundingInfoRespRaw) -> Self {
                let FundingInfoRespRaw(
                    _,
                    symbol,
                    (yield_loan, yield_lend, duration_loan, duration_lend),
                ) = value;

                Self {
                    symbol,
                    yield_loan,
                    yield_lend,
                    duration_loan,
                    duration_lend,
                }
            }
        }

        let raw = FundingInfoRespRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

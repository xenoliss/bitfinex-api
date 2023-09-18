use derive_builder::Builder;
use http::Method;
use serde::Deserialize;
use serde_json::Value;

use crate::api::endpoint::Endpoint;

use super::types::{LoanSide, RateType};

#[derive(Debug, Clone, Copy, Builder)]
pub struct FundingLoans<'a> {
    symbol: &'a str,
}

impl<'a> FundingLoans<'a> {
    pub fn builder() -> FundingLoansBuilder<'a> {
        FundingLoansBuilder::default()
    }
}

impl<'a> Endpoint for FundingLoans<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("v2/auth/r/funding/loans/{}", self.symbol)
    }

    fn is_authenticated(&self) -> bool {
        true
    }
}

pub type FundingLoansResp = Vec<FundingLoanResp>;

#[derive(Debug)]
pub struct FundingLoanResp {
    pub id: u64,
    pub symbol: String,
    pub side: LoanSide,
    pub mts_create: u64,
    pub mts_update: u64,
    pub amount: f64,
    pub status: String,
    pub rate_type: RateType,
    pub rate: f64,
    pub period: u8,
    pub mts_opening: u64,
    pub mts_last_payout: u64,
    pub notify: bool,
    pub hidden: bool,
    pub renew: bool,
    pub no_close: bool,
}

impl<'de> Deserialize<'de> for FundingLoanResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        pub struct FundingLoanRespRaw(
            u64,
            String,
            LoanSide,
            u64,
            u64,
            f64,
            Option<Value>,
            String,
            RateType,
            Option<()>,
            Option<()>,
            f64,
            u8,
            u64,
            u64,
            u8,
            u8,
            Option<()>,
            u8,
            Option<()>,
            u8,
        );

        impl From<FundingLoanRespRaw> for FundingLoanResp {
            fn from(value: FundingLoanRespRaw) -> Self {
                let FundingLoanRespRaw(
                    id,
                    symbol,
                    side,
                    mts_create,
                    mts_update,
                    amount,
                    _flags,
                    status,
                    rate_type,
                    _,
                    _,
                    rate,
                    period,
                    mts_opening,
                    mts_last_payout,
                    notify,
                    hidden,
                    _,
                    renew,
                    _,
                    no_close,
                ) = value;

                Self {
                    id,
                    symbol,
                    side,
                    mts_create,
                    mts_update,
                    amount,
                    status,
                    rate_type,
                    rate,
                    period,
                    mts_opening,
                    mts_last_payout,
                    notify: notify == 1,
                    hidden: hidden == 1,
                    renew: renew == 1,
                    no_close: no_close == 1,
                }
            }
        }

        let raw = FundingLoanRespRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

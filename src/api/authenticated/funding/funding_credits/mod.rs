use derive_builder::Builder;
use http::Method;
use serde::Deserialize;
use serde_json::Value;

use crate::api::{common::PlaceHolder, endpoint::Endpoint};

use super::types::{LoanSide, RateType};

#[derive(Debug, Clone, Copy, Builder)]
pub struct FundingCredits<'a> {
    symbol: &'a str,
}

impl<'a> FundingCredits<'a> {
    pub fn builder() -> FundingCreditsBuilder<'a> {
        FundingCreditsBuilder::default()
    }
}

impl<'a> Endpoint for FundingCredits<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("v2/auth/r/funding/credits/{}", self.symbol)
    }

    fn is_authenticated(&self) -> bool {
        true
    }
}

pub type FundingCreditsResp = Vec<FundingCreditResp>;

#[derive(Debug)]
pub struct FundingCreditResp {
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
    pub notify: Option<bool>,
    pub hidden: bool,
    pub renew: bool,
    pub no_close: bool,
    pub position_pair: String,
}

impl<'de> Deserialize<'de> for FundingCreditResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        pub struct FundingCreditRespRaw(
            u64,
            String,
            LoanSide,
            u64,
            u64,
            f64,
            Option<Value>,
            String,
            RateType,
            PlaceHolder,
            PlaceHolder,
            f64,
            u8,
            u64,
            u64,
            Option<u8>,
            u8,
            PlaceHolder,
            u8,
            PlaceHolder,
            u8,
            String,
        );

        impl From<FundingCreditRespRaw> for FundingCreditResp {
            fn from(value: FundingCreditRespRaw) -> Self {
                let FundingCreditRespRaw(
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
                    position_pair,
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
                    notify: notify.map(|v| v == 1),
                    hidden: hidden == 1,
                    renew: renew == 1,
                    no_close: no_close == 1,
                    position_pair,
                }
            }
        }

        let raw = FundingCreditRespRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

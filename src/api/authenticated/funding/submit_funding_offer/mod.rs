use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::api::endpoint::Endpoint;

use super::types::{FundingOffer, FundingOfferRaw};

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FundingOrderType {
    Limit,
    FrrDeltaFix,
    FrrDeltaVar,
}

#[serde_as]
#[derive(Debug, Clone, Copy, Builder, Serialize)]
pub struct SubmitFundingOffer<'a> {
    #[serde(rename(serialize = "type"))]
    ty: FundingOrderType,
    symbol: &'a str,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    amount: f64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    rate: f64,
    period: u8,
}

impl<'a> SubmitFundingOffer<'a> {
    pub fn builder() -> SubmitFundingOfferBuilder<'a> {
        SubmitFundingOfferBuilder::default()
    }
}

impl<'a> Endpoint for SubmitFundingOffer<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("v2/auth/w/funding/offer/submit")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn body(&self) -> Option<(&'static str, Vec<u8>)> {
        let body = serde_json::to_string(self).unwrap();
        Some(("application/json", body.into_bytes()))
    }
}

#[derive(Debug)]
pub struct SubmitFundingOfferResp {
    pub mts: u64,
    pub ty: String,
    pub message_id: u64,
    pub offer: FundingOffer,
    pub code: u64,
    pub status: String,
    pub text: String,
}

impl<'de> Deserialize<'de> for SubmitFundingOfferResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct SubmitFundingOfferRawResp(u64, String, u64, FundingOfferRaw, u64, String, String);

        impl From<SubmitFundingOfferRawResp> for SubmitFundingOfferResp {
            fn from(value: SubmitFundingOfferRawResp) -> Self {
                let SubmitFundingOfferRawResp(mts, ty, message_id, offer, code, status, text) =
                    value;

                Self {
                    mts,
                    ty,
                    message_id,
                    offer: offer.into(),
                    code,
                    status,
                    text,
                }
            }
        }

        let raw = SubmitFundingOfferRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::api::endpoint::Endpoint;

use super::types::{FundingOffer, FundingOfferRaw, FundingOfferType};

#[derive(Debug, Clone, Copy, Builder)]
pub struct SubmitFundingOffer<'a> {
    ty: FundingOfferType,
    symbol: &'a str,
    amount: f64,
    rate: f64,
    period: u8,
}

impl<'a> SubmitFundingOffer<'a> {
    pub fn builder() -> SubmitFundingOfferBuilder<'a> {
        SubmitFundingOfferBuilder::default()
    }

    fn json_body(&self) -> String {
        #[serde_as]
        #[derive(Debug, Serialize)]
        pub struct JsonParams<'a> {
            #[serde(rename(serialize = "type"))]
            ty: FundingOfferType,
            symbol: &'a str,
            #[serde_as(as = "serde_with::DisplayFromStr")]
            amount: f64,
            #[serde_as(as = "serde_with::DisplayFromStr")]
            rate: f64,
            period: u8,
        }

        let p = JsonParams {
            ty: self.ty,
            symbol: self.symbol,
            amount: self.amount,
            rate: self.rate,
            period: self.period,
        };

        serde_json::to_string(&p).unwrap()
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
        Some(("application/json", self.json_body().into_bytes()))
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

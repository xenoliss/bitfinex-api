use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::api::endpoint::Endpoint;

use super::types::{FundingOffer, FundingOfferRaw};

#[derive(Debug, Clone, Copy, Builder, Serialize)]
pub struct CancelFundingOffer {
    id: u64,
}

impl CancelFundingOffer {
    pub fn builder() -> CancelFundingOfferBuilder {
        CancelFundingOfferBuilder::default()
    }
}

impl Endpoint for CancelFundingOffer {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("v2/auth/w/funding/offer/cancel")
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
pub struct CancelFundingOfferResp {
    pub mts: u64,
    pub ty: String,
    pub message_id: u64,
    pub offer: FundingOffer,
    pub code: Option<u64>,
    pub status: String,
    pub text: String,
}

impl<'de> Deserialize<'de> for CancelFundingOfferResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct CancelFundingOfferRawResp(
            u64,
            String,
            u64,
            FundingOfferRaw,
            Option<u64>,
            String,
            String,
        );

        impl From<CancelFundingOfferRawResp> for CancelFundingOfferResp {
            fn from(value: CancelFundingOfferRawResp) -> Self {
                let CancelFundingOfferRawResp(mts, ty, message_id, offer, code, status, text) =
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

        let raw = CancelFundingOfferRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

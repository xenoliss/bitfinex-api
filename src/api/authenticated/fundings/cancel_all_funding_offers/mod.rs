use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::api::endpoint::Endpoint;

#[derive(Debug, Clone, Copy, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CancelAllFundingOffers<'a> {
    #[builder(default)]
    currency: Option<&'a str>,
}

impl<'a> CancelAllFundingOffers<'a> {
    pub fn builder() -> CancelAllFundingOffersBuilder<'a> {
        CancelAllFundingOffersBuilder::default()
    }
}

impl<'a> Endpoint for CancelAllFundingOffers<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("v2/auth/w/funding/offer/cancel/all")
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
pub struct CancelAllFundingOffersResp {
    pub mts: u64,
    pub ty: String,
    pub status: String,
    pub text: String,
}

impl<'de> Deserialize<'de> for CancelAllFundingOffersResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct CancelAllFundingOffersRawResp(
            u64,
            String,
            Option<()>,
            Option<()>,
            Option<()>,
            Option<()>,
            String,
            String,
        );

        impl From<CancelAllFundingOffersRawResp> for CancelAllFundingOffersResp {
            fn from(value: CancelAllFundingOffersRawResp) -> Self {
                let CancelAllFundingOffersRawResp(mts, ty, _, _, _, _, status, text) = value;

                Self {
                    mts,
                    ty,
                    status,
                    text,
                }
            }
        }

        let raw = CancelAllFundingOffersRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

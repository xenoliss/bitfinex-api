use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::api::{common::PlaceHolder, endpoint::Endpoint};

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct CancelAllFundingOffers<'a> {
    #[builder(default)]
    currency: Option<&'a str>,
}

impl<'a> CancelAllFundingOffers<'a> {
    pub fn builder() -> CancelAllFundingOffersBuilder<'a> {
        CancelAllFundingOffersBuilder::default()
    }

    fn json_body(&self) -> String {
        #[derive(Debug, Serialize)]
        pub struct JsonParams<'a> {
            currency: &'a Option<&'a str>,
        }

        let p = JsonParams {
            currency: &self.currency,
        };

        serde_json::to_string(&p).unwrap()
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
        Some(("application/json", self.json_body().into_bytes()))
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
            PlaceHolder,
            PlaceHolder,
            PlaceHolder,
            PlaceHolder,
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

use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::api::endpoint::Endpoint;

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
    pub id: u64,
    pub symbol: String,
    pub mts_created: u64,
    pub mts_updated: u64,
    pub amount: f64,
    pub amount_orig: f64,
    pub offer_type: String,
    pub offer_status: String,
    pub rate: f64,
    pub period: u8,
    pub notify: bool,
    pub hidden: bool,
    pub renew: bool,
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
            Option<()>,
            Option<()>,
            SubmitFundingOfferInternalRawResp,
            Option<()>,
            String,
            String,
        );

        #[derive(Debug, Deserialize)]
        struct SubmitFundingOfferInternalRawResp(
            u64,
            u64,
            String,
            u64,
            u64,
            f64,
            f64,
            String,
            Option<()>,
            Option<()>,
            Option<()>,
            String,
            Option<()>,
            Option<()>,
            Option<()>,
            f64,
            u8,
            bool,
            u8,
            Option<()>,
            bool,
            Option<u64>,
        );

        impl From<CancelFundingOfferRawResp> for CancelFundingOfferResp {
            fn from(value: CancelFundingOfferRawResp) -> Self {
                let CancelFundingOfferRawResp(
                    mts,
                    ty,
                    _,
                    _,
                    SubmitFundingOfferInternalRawResp(
                        message_id,
                        id,
                        symbol,
                        mts_created,
                        mts_updated,
                        amount,
                        amount_orig,
                        offer_type,
                        _,
                        _,
                        _,
                        offer_status,
                        _,
                        _,
                        _,
                        rate,
                        period,
                        notify,
                        hidden,
                        _,
                        renew,
                        code,
                    ),
                    _,
                    status,
                    text,
                ) = value;

                Self {
                    mts,
                    ty,
                    message_id,
                    id,
                    symbol,
                    mts_created,
                    mts_updated,
                    amount,
                    amount_orig,
                    offer_type,
                    offer_status,
                    rate,
                    period,
                    notify,
                    hidden: hidden == 1,
                    renew,
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

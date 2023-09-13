use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::endpoint::Endpoint;

#[derive(Debug, Clone, Copy, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct ActiveFundingOffers<'a> {
    #[builder(default)]
    symbol: Option<&'a str>,
}

impl<'a> ActiveFundingOffers<'a> {
    pub fn builder() -> ActiveFundingOffersBuilder<'a> {
        ActiveFundingOffersBuilder::default()
    }
}

impl<'a> Endpoint for ActiveFundingOffers<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        if let Some(symbol) = self.symbol {
            format!("v2/auth/r/funding/offers/{symbol}")
        } else {
            String::from("v2/auth/r/funding/offers/")
        }
    }

    fn is_authenticated(&self) -> bool {
        true
    }
}

pub type ActiveFundingOffersResp = Vec<ActiveFundingOfferResp>;

#[derive(Debug)]
pub struct ActiveFundingOfferResp {
    pub id: u64,
    pub symbol: String,
    pub mts_created: u64,
    pub mts_updated: u64,
    pub amount: f64,
    pub amount_orig: f64,
    pub ty: String,
    pub status: String,
    pub rate: f64,
    pub period: u8,
    pub notify: bool,
    pub hidden: bool,
    pub renew: bool,
}

impl<'de> Deserialize<'de> for ActiveFundingOfferResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct ActiveFundingOfferRawResp(
            u64,
            String,
            u64,
            u64,
            f64,
            f64,
            String,
            Value,
            String,
            f64,
            u8,
            u8,
            u8,
            u8,
        );

        impl From<ActiveFundingOfferRawResp> for ActiveFundingOfferResp {
            fn from(value: ActiveFundingOfferRawResp) -> Self {
                let ActiveFundingOfferRawResp(
                    id,
                    symbol,
                    mts_created,
                    mts_updated,
                    amount,
                    amount_orig,
                    ty,
                    _,
                    status,
                    rate,
                    period,
                    notify,
                    hidden,
                    renew,
                ) = value;

                Self {
                    id,
                    symbol,
                    mts_created,
                    mts_updated,
                    amount,
                    amount_orig,
                    ty,
                    status,
                    rate,
                    period,
                    notify: notify == 1,
                    hidden: hidden == 1,
                    renew: renew == 1,
                }
            }
        }

        let raw = ActiveFundingOfferRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

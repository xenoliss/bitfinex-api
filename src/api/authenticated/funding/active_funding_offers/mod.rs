use derive_builder::Builder;
use http::Method;

use crate::api::endpoint::Endpoint;

use super::types::FundingOffer;

#[derive(Debug, Clone, Copy, Builder)]
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

pub type ActiveFundingOffersResp = Vec<FundingOffer>;

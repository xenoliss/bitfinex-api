use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{
    common::{PlaceHolder, Sort},
    endpoint::Endpoint,
    params::QueryParams,
};

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Liquidations {
    #[builder(default)]
    sort: Option<Sort>,
    #[builder(default)]
    start: Option<u64>,
    #[builder(default)]
    end: Option<u64>,
    #[builder(default)]
    limit: Option<u64>,
}

impl Liquidations {
    pub fn builder() -> LiquidationsBuilder {
        LiquidationsBuilder::default()
    }
}

impl Endpoint for Liquidations {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        String::from("v2/liquidations/hist")
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params
            .push_opt("sort", self.sort.map(|sort| sort as i8))
            .push_opt("start", self.start)
            .push_opt("end", self.end)
            .push_opt("limit", self.limit);
        params
    }
}

pub type LiquidationsResp = Vec<LiquidationResp>;

#[derive(Debug)]
pub struct LiquidationResp {
    pub pos_id: u64,
    pub mts: u64,
    pub symbol: String,
    pub amount: f64,
    pub base_price: f64,
    pub is_match: bool,
    pub is_market_sold: bool,
    pub price_acquired: Option<f64>,
}

impl<'de> Deserialize<'de> for LiquidationResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct LiquidationRawResp(
            String,
            u64,
            u64,
            PlaceHolder,
            String,
            f64,
            f64,
            PlaceHolder,
            u8,
            u8,
            PlaceHolder,
            Option<f64>,
        );

        impl From<LiquidationRawResp> for LiquidationResp {
            fn from(value: LiquidationRawResp) -> Self {
                let LiquidationRawResp(
                    _,
                    pos_id,
                    mts,
                    _,
                    symbol,
                    amount,
                    base_price,
                    _,
                    is_match,
                    is_market_sold,
                    _,
                    price_acquired,
                ) = value;

                Self {
                    pos_id,
                    mts,
                    symbol,
                    amount,
                    base_price,
                    is_match: is_match == 1,
                    is_market_sold: is_market_sold == 1,
                    price_acquired,
                }
            }
        }

        let [raw] = <[LiquidationRawResp; 1]>::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

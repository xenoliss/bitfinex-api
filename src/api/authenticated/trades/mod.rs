use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::common::Sort;
use crate::api::endpoint::Endpoint;
use crate::api::params::QueryParams;

use super::orders::types::OrderType;

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Trades {
    #[builder(default)]
    start: Option<u64>,
    #[builder(default)]
    end: Option<u64>,
    #[builder(default)]
    limit: Option<u64>,
    #[builder(default)]
    sort: Option<Sort>,
}

impl Trades {
    pub fn builder() -> TradesBuilder {
        TradesBuilder::default()
    }
}

impl Endpoint for Trades {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("v2/auth/r/trades/hist")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params
            .push_opt("start", self.start)
            .push_opt("end", self.end)
            .push_opt("limit", self.limit)
            .push_opt("sort", self.sort.map(|sort| sort as i8));
        params
    }
}

pub type TradesResp = Vec<TradeResp>;

/// https://docs.bitfinex.com/reference/rest-auth-trades
///
/// [0]  ID	        int     Trade database id
/// [1]	SYMBOL	        string	Symbol (BTCUSD, …)
/// [2]	MTS	        int	Execution timestamp
/// [3]	ORDER_ID	int	Order id
/// [4]	EXEC_AMOUNT	float	Positive means buy, negative means sell
/// [5]	EXEC_PRICE	float	Execution price
/// [6]	ORDER_TYPE	string	Order type
/// [7]	ORDER_PRICE	float	Order price
/// [8]	MAKER	        int	1 if true, -1 if false
/// [9]	FEE	        float	Fee
/// [10]FEE_CURRENCY	string	Fee currency
/// [11]CID	        int	Client Order ID
#[derive(Debug)]
pub struct TradeResp {
    pub id: u64,
    pub symbol: String,
    pub mts: u64,
    pub order_id: u64,
    pub exec_amount: f64,
    pub exec_price: f64,
    pub order_type: OrderType,
    pub order_price: f64,
    pub maker: i32,
    pub fee: f64,
    pub fee_currency: String,
    pub cid: u64,
}

impl<'de> Deserialize<'de> for TradeResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct TradeRawResp(
            u64,
            String,
            u64,
            u64,
            f64,
            f64,
            OrderType,
            f64,
            i32,
            f64,
            String,
            u64,
        );

        impl From<TradeRawResp> for TradeResp {
            fn from(value: TradeRawResp) -> Self {
                let TradeRawResp(
                    id,
                    symbol,
                    mts,
                    order_id,
                    exec_amount,
                    exec_price,
                    order_type,
                    order_price,
                    maker,
                    fee,
                    fee_currency,
                    cid,
                ) = value;

                Self {
                    id,
                    symbol,
                    mts,
                    order_id,
                    exec_amount,
                    exec_price,
                    order_type,
                    order_price,
                    maker,
                    fee,
                    fee_currency,
                    cid,
                }
            }
        }

        let raw = TradeRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

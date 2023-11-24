use serde::{Deserialize, Serialize};

use crate::api::common::PlaceHolder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum OrderFlag {
    Hidden = 64,
    Close = 512,
    ReduceOnly = 1024,
    PostOnly = 4096,
    OCO = 16384,
    NoVarRates = 524288,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "EXCHANGE LIMIT")]
    ExchangeLimit,
    #[serde(rename = "MARKET")]
    Market,
    #[serde(rename = "EXCHANGE MARKET")]
    ExchangeMarket,
    #[serde(rename = "STOP")]
    Stop,
    #[serde(rename = "EXCHANGE STOP")]
    ExchangeStop,
    #[serde(rename = "STOP LIMIT")]
    StopLimit,
    #[serde(rename = "EXCHANGE STOP LIMIT")]
    ExchangeStopLimit,
    #[serde(rename = "TRAILING STOP")]
    TrailingStop,
    #[serde(rename = "EXCHANGE TRAILING STOP")]
    ExchangeTrailingStop,
    #[serde(rename = "FOK")]
    Fok,
    #[serde(rename = "EXCHANGE FOK")]
    ExchangeFok,
    #[serde(rename = "IOC")]
    Ioc,
    #[serde(rename = "EXCHANGE IOC")]
    ExchangeIoc,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub gid: Option<u64>,
    pub cid: u64,
    pub symbol: String,
    pub mts_created: u64,
    pub mts_updated: u64,
    pub amount: f64,
    pub amount_orig: f64,
    pub order_type: OrderType,
    pub type_prev: Option<OrderType>,
    pub mts_tif: Option<u64>,
    pub flags: Option<u64>,
    pub status: String,
    pub price: f64,
    pub price_avg: f64,
    pub price_trailling: f64,
    pub price_aux_limit: f64,
    pub notify: bool,
    pub hidden: bool,
    pub placed_id: Option<u64>,
    pub routing: String,
    pub meta: Option<serde_json::Value>,
}

impl<'de> Deserialize<'de> for Order {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = OrderRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderRaw(
    u64,
    Option<u64>,
    u64,
    String,
    u64,
    u64,
    f64,
    f64,
    OrderType,
    Option<OrderType>,
    Option<u64>,
    PlaceHolder,
    Option<u64>,
    String,
    PlaceHolder,
    PlaceHolder,
    f64,
    f64,
    f64,
    f64,
    PlaceHolder,
    PlaceHolder,
    PlaceHolder,
    u8,
    u8,
    Option<u64>,
    PlaceHolder,
    PlaceHolder,
    String,
    PlaceHolder,
    PlaceHolder,
    Option<serde_json::Value>,
);

impl From<OrderRaw> for Order {
    fn from(value: OrderRaw) -> Self {
        let OrderRaw(
            id,
            gid,
            cid,
            symbol,
            mts_created,
            mts_updated,
            amount,
            amount_orig,
            order_type,
            type_prev,
            mts_tif,
            _,
            flags,
            status,
            _,
            _,
            price,
            price_avg,
            price_trailling,
            price_aux_limit,
            _,
            _,
            _,
            notify,
            hidden,
            placed_id,
            _,
            _,
            routing,
            _,
            _,
            meta,
        ) = value;

        Self {
            id,
            gid,
            cid,
            symbol,
            mts_created,
            mts_updated,
            amount,
            amount_orig,
            order_type,
            type_prev,
            mts_tif,
            flags,
            status,
            price,
            price_avg,
            price_trailling,
            price_aux_limit,
            notify: notify == 1,
            hidden: hidden == 1,
            placed_id,
            routing,
            meta,
        }
    }
}

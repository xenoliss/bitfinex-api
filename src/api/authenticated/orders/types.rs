use serde::{Deserialize, Serialize, Serializer};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Limit,
    ExchangeLimit,
    Market,
    ExchangeMarket,
    Stop,
    ExchangeStop,
    StopLimit,
    ExchangeStopLimit,
    TrailingStop,
    ExchangeTrailingStop,
    Fok,
    ExchangeFok,
    Ioc,
    ExchangeIoc,
}

impl Serialize for OrderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            OrderType::Limit => serializer.serialize_unit_variant("OrderType", 0, "LIMIT"),
            OrderType::ExchangeLimit => {
                serializer.serialize_unit_variant("OrderType", 1, "EXCHANGE LIMIT")
            }
            OrderType::Market => serializer.serialize_unit_variant("OrderType", 2, "MARKET"),
            OrderType::ExchangeMarket => {
                serializer.serialize_unit_variant("OrderType", 3, "EXCHANGE MARKET")
            }
            OrderType::Stop => serializer.serialize_unit_variant("OrderType", 4, "STOP"),
            OrderType::ExchangeStop => {
                serializer.serialize_unit_variant("OrderType", 5, "EXCHANGE STOP")
            }
            OrderType::StopLimit => serializer.serialize_unit_variant("OrderType", 6, "STOP LIMIT"),
            OrderType::ExchangeStopLimit => {
                serializer.serialize_unit_variant("OrderType", 7, "EXCHANGE STOP LIMIT")
            }
            OrderType::TrailingStop => {
                serializer.serialize_unit_variant("OrderType", 8, "TRAILING STOP")
            }
            OrderType::ExchangeTrailingStop => {
                serializer.serialize_unit_variant("OrderType", 9, "EXCHANGE TRAILING STOP")
            }
            OrderType::Fok => serializer.serialize_unit_variant("OrderType", 10, "FOK"),
            OrderType::ExchangeFok => {
                serializer.serialize_unit_variant("OrderType", 11, "EXCHANGE FOK")
            }
            OrderType::Ioc => serializer.serialize_unit_variant("OrderType", 12, "IOC"),
            OrderType::ExchangeIoc => {
                serializer.serialize_unit_variant("OrderType", 13, "EXCHANGE IOC")
            }
        }
    }
}

impl<'de> Deserialize<'de> for OrderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_uppercase();
        let state = match s.as_str() {
            "LIMIT" => OrderType::Limit,
            "EXCHANGE LIMIT" => OrderType::ExchangeLimit,
            "MARKET" => OrderType::Market,
            "EXCHANGE MARKET" => OrderType::ExchangeMarket,
            "STOP" => OrderType::Stop,
            "EXCHANGE STOP" => OrderType::ExchangeStop,
            "STOP LIMIT" => OrderType::StopLimit,
            "EXCHANGE STOP LIMIT" => OrderType::ExchangeStopLimit,
            "TRAILING STOP" => OrderType::TrailingStop,
            "EXCHANGE TRAILING STOP" => OrderType::ExchangeTrailingStop,
            "FOK" => OrderType::Fok,
            "EXCHANGE FOK" => OrderType::ExchangeFok,
            "IOC" => OrderType::Ioc,
            "EXCHANGE IOC" => OrderType::ExchangeIoc,
            other => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid state '{}'",
                    other
                )));
            }
        };
        Ok(state)
    }
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

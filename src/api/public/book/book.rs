use std::fmt::Display;

use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use super::common::Len;
use crate::api::{endpoint::Endpoint, params::QueryParams};

#[derive(Debug, Clone, Copy)]
pub enum Precision {
    P0,
    P1,
    P2,
    P3,
    P4,
}

impl Display for Precision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Precision::P0 => write!(f, "P0"),
            Precision::P1 => write!(f, "P1"),
            Precision::P2 => write!(f, "P2"),
            Precision::P3 => write!(f, "P3"),
            Precision::P4 => write!(f, "P4"),
        }
    }
}

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Book<'a> {
    symbol: &'a str,
    precision: Precision,
    #[builder(default)]
    len: Option<Len>,
}

impl<'a> Book<'a> {
    pub fn builder() -> BookBuilder<'a> {
        BookBuilder::default()
    }
}

impl<'a> Endpoint for Book<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!("v2/book/{}/{}", self.symbol, self.precision)
    }

    fn is_authenticated(&self) -> bool {
        false
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("len", self.len.map(|len| len as u8));
        params
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BookResp {
    BookTradingResp(Vec<BookTradingResp>),
    BookFundingResp(Vec<BookFundingResp>),
}

#[derive(Debug)]
pub struct BookTradingResp {
    pub price: f64,
    pub count: u64,
    pub amount: f64,
}

#[derive(Debug)]
pub struct BookFundingResp {
    pub rate: f64,
    pub period: u64,
    pub count: u64,
    pub amount: f64,
}

impl<'de> Deserialize<'de> for BookTradingResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct BookTradingRawResp(f64, u64, f64);

        impl From<BookTradingRawResp> for BookTradingResp {
            fn from(value: BookTradingRawResp) -> Self {
                let BookTradingRawResp(price, count, amount) = value;

                Self {
                    price,
                    count,
                    amount,
                }
            }
        }

        let raw = BookTradingRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

impl<'de> Deserialize<'de> for BookFundingResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct BookFundingRawResp(f64, u64, u64, f64);

        impl From<BookFundingRawResp> for BookFundingResp {
            fn from(value: BookFundingRawResp) -> Self {
                let BookFundingRawResp(rate, period, count, amount) = value;

                Self {
                    rate,
                    period,
                    count,
                    amount,
                }
            }
        }

        let raw = BookFundingRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

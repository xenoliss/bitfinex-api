use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use serde_with::serde_as;

use crate::api::endpoint::Endpoint;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BalanceType {
    Exchange,
    Margin,
    Deriv,
    Funding,
}

#[derive(Debug, Clone, Copy, Serialize_repr)]
#[repr(i8)]
pub enum OrderDir {
    Short = -1,
    Long = 1,
}

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct BalanceAvailable<'a> {
    symbol: &'a str,
    ty: BalanceType,
    #[builder(default)]
    dir: Option<OrderDir>,
    #[builder(default)]
    rate: Option<f64>,
    #[builder(default)]
    lev: Option<f64>,
}

impl<'a> BalanceAvailable<'a> {
    pub fn builder() -> BalanceAvailableBuilder<'a> {
        BalanceAvailableBuilder::default()
    }

    fn json_body(&self) -> String {
        #[serde_as]
        #[derive(Debug, Serialize)]
        pub struct JsonParams<'a> {
            symbol: &'a str,
            #[serde(rename(serialize = "type"))]
            ty: BalanceType,
            dir: Option<OrderDir>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
            rate: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
            lev: Option<f64>,
        }

        let p = JsonParams {
            symbol: self.symbol,
            ty: self.ty,
            dir: self.dir,
            rate: self.rate,
            lev: self.lev,
        };

        serde_json::to_string(&p).unwrap()
    }
}

impl<'a> Endpoint for BalanceAvailable<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        String::from("v2/auth/calc/order/avail")
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn body(&self) -> Option<(&'static str, Vec<u8>)> {
        Some(("application/json", self.json_body().into_bytes()))
    }
}

#[derive(Debug)]
pub struct BalanceAvailableResp {
    pub amount_avail: f64,
}

impl<'de> Deserialize<'de> for BalanceAvailableResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        pub struct BalanceAvailableRespRaw(f64);

        impl From<BalanceAvailableRespRaw> for BalanceAvailableResp {
            fn from(value: BalanceAvailableRespRaw) -> Self {
                let BalanceAvailableRespRaw(amount_avail) = value;

                Self { amount_avail }
            }
        }

        let [raw] = <[BalanceAvailableRespRaw; 1]>::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

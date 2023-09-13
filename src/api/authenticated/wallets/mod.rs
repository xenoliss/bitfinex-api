use derive_builder::Builder;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::api::endpoint::Endpoint;

#[serde_as]
#[derive(Debug, Clone, Copy, Builder, Serialize)]
pub struct Wallets {}

impl Wallets {
    pub fn builder() -> WalletsBuilder {
        WalletsBuilder::default()
    }
}

impl Endpoint for Wallets {
    fn method(&self) -> Method {
        Method::POST
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn endpoint(&self) -> String {
        String::from("v2/auth/r/wallets")
    }
}

pub type WalletsResp = Vec<WalletResp>;

#[derive(Debug)]
pub struct WalletResp {
    pub ty: String,
    pub currency: String,
    pub balance: f64,
    pub unsettled_interest: f64,
    pub available_balance: f64,
    pub last_change: Option<String>,
    pub trade_details: Option<serde_json::Value>,
}

impl<'de> Deserialize<'de> for WalletResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct WalletRawResp(
            String,
            String,
            f64,
            f64,
            f64,
            Option<String>,
            Option<serde_json::Value>,
        );

        impl From<WalletRawResp> for WalletResp {
            fn from(value: WalletRawResp) -> Self {
                let WalletRawResp(
                    ty,
                    currency,
                    balance,
                    unsettled_interest,
                    available_balance,
                    last_change,
                    trade_details,
                ) = value;

                Self {
                    ty,
                    currency,
                    balance,
                    unsettled_interest,
                    available_balance,
                    last_change,
                    trade_details,
                }
            }
        }

        let raw = WalletRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

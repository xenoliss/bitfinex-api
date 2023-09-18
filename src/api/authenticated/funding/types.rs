use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RateType {
    Fixed,
    Var,
}

#[derive(Debug, Clone, Copy, Deserialize_repr)]
#[repr(i8)]
pub enum LoanSide {
    Borrower = -1,
    Both = 0,
    Lender = 1,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FundingOfferType {
    Limit,
    FrrDeltaFix,
    FrrDeltaVar,
}

#[derive(Debug)]
pub struct FundingOffer {
    pub id: u64,
    pub symbol: String,
    pub mts_created: u64,
    pub mts_updated: u64,
    pub amount: f64,
    pub amount_orig: f64,
    pub offer_type: FundingOfferType,
    pub flags: u64,
    pub offer_status: String,
    pub rate: f64,
    pub period: u8,
    pub notify: bool,
    pub hidden: bool,
    pub renew: bool,
}

impl<'de> Deserialize<'de> for FundingOffer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = FundingOfferRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

#[derive(Debug, Deserialize)]
pub struct FundingOfferRaw(
    u64,
    String,
    u64,
    u64,
    f64,
    f64,
    FundingOfferType,
    Option<()>,
    Option<()>,
    u64,
    String,
    Option<()>,
    Option<()>,
    Option<()>,
    f64,
    u8,
    u8,
    u8,
    Option<()>,
    u8,
);

impl From<FundingOfferRaw> for FundingOffer {
    fn from(value: FundingOfferRaw) -> Self {
        let FundingOfferRaw(
            id,
            symbol,
            mts_created,
            mts_updated,
            amount,
            amount_orig,
            offer_type,
            _,
            _,
            flags,
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
        ) = value;

        Self {
            id,
            symbol,
            mts_created,
            mts_updated,
            amount,
            amount_orig,
            offer_type,
            flags,
            offer_status,
            rate,
            period,
            notify: notify == 1,
            hidden: hidden == 1,
            renew: renew == 1,
        }
    }
}

use serde::Deserialize;

#[derive(Debug)]
pub struct FundingOffer {
    pub id: u64,
    pub symbol: String,
    pub mts_created: u64,
    pub mts_updated: u64,
    pub amount: f64,
    pub amount_orig: f64,
    pub offer_type: String,
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
    String,
    u64,
    String,
    f64,
    u8,
    u8,
    u8,
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
            flags,
            offer_status,
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

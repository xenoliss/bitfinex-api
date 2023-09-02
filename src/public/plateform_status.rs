use std::fmt::Display;

use serde::Deserialize;

use super::{BitfinexRequest, PUB_ENDPOINT};

#[derive(Debug)]
pub enum PlatformStatus {
    Maintenance = 0,
    Operative = 1,
}

impl<'de> Deserialize<'de> for PlatformStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        pub struct PlatformStatusRaw(u8);

        impl From<PlatformStatusRaw> for PlatformStatus {
            fn from(value: PlatformStatusRaw) -> Self {
                match value.0 {
                    0 => Self::Maintenance,
                    _ => Self::Operative,
                }
            }
        }

        let raw = Vec::<PlatformStatusRaw>::deserialize(deserializer)?
            .pop()
            .unwrap();
        Ok(raw.into())
    }
}

pub struct GetPlatformStatusReq;

impl BitfinexRequest for GetPlatformStatusReq {
    fn path(&self) -> String {
        "platform/status".into()
    }
}

impl Display for GetPlatformStatusReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}

pub async fn get_platform_status() -> PlatformStatus {
    let req = GetPlatformStatusReq;

    reqwest::get(format!("{PUB_ENDPOINT}/{req}"))
        .await
        .unwrap()
        .json::<PlatformStatus>()
        .await
        .unwrap()
}

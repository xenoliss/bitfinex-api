use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::endpoint::Endpoint;

#[derive(Debug, Clone, Copy, Builder)]
pub struct PlatformStatus {}

impl PlatformStatus {
    pub fn builder() -> PlatformStatusBuilder {
        PlatformStatusBuilder::default()
    }
}

impl Endpoint for PlatformStatus {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        String::from("v2/platform/status")
    }

    fn is_authenticated(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub enum PlatformStatusResp {
    Maintenance = 0,
    Operative = 1,
}

impl<'de> Deserialize<'de> for PlatformStatusResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct PlatformStatusRespRaw(u8);

        impl From<PlatformStatusRespRaw> for PlatformStatusResp {
            fn from(value: PlatformStatusRespRaw) -> Self {
                match value.0 {
                    0 => Self::Maintenance,
                    _ => Self::Operative,
                }
            }
        }

        let [raw] = <[PlatformStatusRespRaw; 1]>::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

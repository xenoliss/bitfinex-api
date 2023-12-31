use std::fmt::Display;

use derive_builder::Builder;
use http::Method;
use serde::Deserialize;

use crate::api::{
    common::{PlaceHolder, Section, Sort, TimeFrame},
    endpoint::Endpoint,
    params::QueryParams,
};

#[derive(Debug, Clone, Copy)]
pub enum Key {
    PluDiff,
    Plu,
    Plr,
    Vol,
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::PluDiff => write!(f, "plu_diff"),
            Key::Plu => write!(f, "plu"),
            Key::Plr => write!(f, "plr"),
            Key::Vol => write!(f, "vol"),
        }
    }
}

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Leaderboards<'a> {
    key: Key,
    time_frame: TimeFrame,
    symbol: &'a str,
    section: Section,
    #[builder(default)]
    sort: Option<Sort>,
    #[builder(default)]
    start: Option<u64>,
    #[builder(default)]
    end: Option<u64>,
    #[builder(default)]
    limit: Option<u64>,
}

impl<'a> Leaderboards<'a> {
    pub fn builder() -> LeaderboardsBuilder<'a> {
        LeaderboardsBuilder::default()
    }
}

impl<'a> Endpoint for Leaderboards<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        format!(
            "v2/rankings/{}:{}:{}/{}",
            self.key, self.time_frame, self.symbol, self.section
        )
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params
            .push_opt("sort", self.sort.map(|sort| sort as i8))
            .push_opt("start", self.start)
            .push_opt("end", self.end)
            .push_opt("limit", self.limit);
        params
    }
}

pub type LastLeaderBoardResp = LeaderBoardResp;
pub type HistLeaderBoardsResp = Vec<LeaderBoardResp>;

#[derive(Debug)]
pub struct LeaderBoardResp {
    pub mts: u64,
    pub username: String,
    pub ranking: u64,
    pub value: f64,
}

impl<'de> Deserialize<'de> for LeaderBoardResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct LeaderBoardRawResp(
            u64,
            PlaceHolder,
            String,
            u64,
            PlaceHolder,
            PlaceHolder,
            f64,
            PlaceHolder,
            Option<u64>,
            PlaceHolder,
            PlaceHolder,
            PlaceHolder,
            Option<u64>,
        );

        impl From<LeaderBoardRawResp> for LeaderBoardResp {
            fn from(value: LeaderBoardRawResp) -> Self {
                let LeaderBoardRawResp(mts, _, username, ranking, _, _, value, _, _, _, _, _, _) =
                    value;

                Self {
                    mts,
                    username,
                    ranking,
                    value,
                }
            }
        }

        let raw = LeaderBoardRawResp::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

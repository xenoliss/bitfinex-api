use std::fmt::Display;

pub type PlaceHolder = Option<()>;

#[derive(Debug, Clone)]
pub enum Symbols<'a> {
    All,
    Only(Vec<&'a str>),
}

impl<'a> Symbols<'a> {
    pub fn as_query_string(&self) -> String {
        match self {
            Symbols::All => String::from("ALL"),
            Symbols::Only(symbols) => symbols.to_vec().join(","),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(i8)]
pub enum Sort {
    Desc = -1,
    Asc = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum Section {
    Hist,
    Last,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Section::Hist => write!(f, "hist"),
            Section::Last => write!(f, "last"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TimeFrame {
    OneMin,
    FiveMins,
    FifteenMins,
    ThirtyMins,
    OneHour,
    ThreeHours,
    SixHours,
    TwelveHours,
    OneDay,
    OneWeek,
    FourteenDays,
    OneMonth,
}

impl Display for TimeFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeFrame::OneMin => write!(f, "1m"),
            TimeFrame::FiveMins => write!(f, "5m"),
            TimeFrame::FifteenMins => write!(f, "15m"),
            TimeFrame::ThirtyMins => write!(f, "30m"),
            TimeFrame::OneHour => write!(f, "1h"),
            TimeFrame::ThreeHours => write!(f, "3h"),
            TimeFrame::SixHours => write!(f, "6h"),
            TimeFrame::TwelveHours => write!(f, "12h"),
            TimeFrame::OneDay => write!(f, "1D"),
            TimeFrame::OneWeek => write!(f, "1W"),
            TimeFrame::FourteenDays => write!(f, "14D"),
            TimeFrame::OneMonth => write!(f, "1M"),
        }
    }
}

use std::fmt::Display;

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

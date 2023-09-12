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

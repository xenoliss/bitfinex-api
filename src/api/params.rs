use url::Url;

/// A structure for query parameters.
#[derive(Debug, Default, Clone)]
pub struct QueryParams<'a> {
    params: Vec<(&'a str, &'a str)>,
}

impl<'a> QueryParams<'a> {
    /// Push a single parameter.
    pub fn push(&mut self, key: impl Into<&'a str>, value: impl Into<&'a str>) -> &mut Self {
        self.params.push((key.into(), value.into()));
        self
    }

    /// Push a single parameter.
    pub fn push_opt(
        &mut self,
        key: impl Into<&'a str>,
        value: Option<impl Into<&'a str>>,
    ) -> &mut Self {
        if let Some(value) = value {
            self.params.push((key.into(), value.into()));
        }
        self
    }

    /// Add the parameters to a URL.
    pub fn add_to_url(&self, url: &mut Url) {
        let mut pairs = url.query_pairs_mut();
        pairs.extend_pairs(self.params.iter());
    }
}

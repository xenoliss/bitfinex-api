use url::Url;

/// A structure for query parameters.
#[derive(Debug, Default, Clone)]
pub struct QueryParams<'a> {
    params: Vec<(&'a str, String)>,
}

impl<'a> QueryParams<'a> {
    /// Push a single parameter.
    pub fn push<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<&'a str>,
        V: ToString,
    {
        self.params.push((key.into(), value.to_string()));
        self
    }

    /// Push a single parameter.
    pub fn push_opt<K, V>(&mut self, key: K, value: Option<V>) -> &mut Self
    where
        K: Into<&'a str>,
        V: ToString,
    {
        if let Some(value) = value {
            self.params.push((key.into(), value.to_string()));
        }
        self
    }

    /// Add the parameters to a URL.
    pub fn add_to_url(&self, url: &mut Url) {
        let mut pairs = url.query_pairs_mut();
        pairs.extend_pairs(self.params.iter());
    }
}

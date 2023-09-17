use std::time::{SystemTime, UNIX_EPOCH};

use hmac::{Hmac, Mac};
use http::{HeaderMap, HeaderValue};
use sha2::Sha384;

#[derive(Debug)]
pub struct Auth {
    api_key: String,
    secret_key: String,
}

impl Auth {
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            api_key,
            secret_key,
        }
    }

    /// Adds the appropriate headers to perform authenticated calls.
    pub fn set_headers(&self, headers: &mut HeaderMap<HeaderValue>, path: &str, body: &[u8]) {
        let nonce = self.generate_nonce();
        let signature_payload = format!("/api/{path}{nonce}{}", std::str::from_utf8(body).unwrap());

        let mut mac = Hmac::<Sha384>::new_from_slice(self.secret_key.as_bytes()).unwrap();
        mac.update(signature_payload.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        let nonce_header_value = HeaderValue::from(nonce);
        let api_key_header_value = HeaderValue::from_str(&self.api_key).unwrap();
        let signature_header_value = HeaderValue::from_str(&signature).unwrap();

        headers.insert("bfx-nonce", nonce_header_value);
        headers.insert("bfx-apikey", api_key_header_value);
        headers.insert("bfx-signature", signature_header_value);
    }

    fn generate_nonce(&self) -> u64 {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let timestamp =
            since_epoch.as_secs() * 1000 + since_epoch.subsec_nanos() as u64 / 1_000_000;

        timestamp + 1
    }
}

use reqwest::Url;
use serde::{Deserialize, Serialize};

pub const HELLO_PATH: &str = "/v1/hello";
pub const IDENTITY_PATH: &str = "/v1/identity";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityResponse {
    pub service_version: String,
}

pub fn hello_body(text: Option<&str>) -> Result<String, ()> {
    match text {
        Some(value) => Ok(value.to_string()),
        None => Err(()),
    }
}

pub fn build_hello_url(base: &str, text: &str) -> Result<String, String> {
    let mut url = Url::parse(base).map_err(|err| format!("Invalid base URL: {err}"))?;
    url.set_path(HELLO_PATH);
    url.query_pairs_mut().clear().append_pair("text", text);
    Ok(url.to_string())
}

pub fn build_identity_url(base: &str) -> Result<String, String> {
    let mut url = Url::parse(base).map_err(|err| format!("Invalid base URL: {err}"))?;
    url.set_path(IDENTITY_PATH);
    url.set_query(None);
    Ok(url.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_hello_url_appends_path_and_query() {
        let url = build_hello_url("http://127.0.0.1:8080", "hello").expect("url");
        assert_eq!(url, "http://127.0.0.1:8080/v1/hello?text=hello");
    }

    #[test]
    fn build_identity_url_appends_path() {
        let url = build_identity_url("http://127.0.0.1:8080").expect("url");
        assert_eq!(url, "http://127.0.0.1:8080/v1/identity");
    }
}

use crate::error::ClientError;

use crate::state::WebDynproState;

use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap, HeaderValue};

use url::Url;

/// Helper function to create headers for XHR requests
pub fn wd_xhr_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"),
    );
    headers.insert(
        "X-Requested-With",
        HeaderValue::from_static("XMLHttpRequest"),
    );
    headers
}

/// Trait for WebDynpro HTTP requests
pub trait Requests {
    /// Navigate to a WebDynpro application
    fn wd_navigate(&self, base_url: &Url, name: &str) -> reqwest::RequestBuilder;

    /// Send an XHR request to WebDynpro
    fn wd_xhr(&self, url: &str, body: String) -> reqwest::RequestBuilder;
}

impl Requests for reqwest::Client {
    fn wd_navigate(&self, base_url: &Url, name: &str) -> reqwest::RequestBuilder {
        let mut url = base_url.to_string();
        if !url.ends_with('/') {
            url.push('/');
        }
        url.push_str(name);
        url.push_str("?sap-wd-stableids=X#");

        self.get(url)
    }

    fn wd_xhr(&self, url: &str, body: String) -> reqwest::RequestBuilder {
        self.post(url).headers(wd_xhr_header()).body(body)
    }
}

/// Navigate to a WebDynpro application and return the HTML body
pub async fn navigate(
    base_url: &Url,
    name: &str,
    client: &reqwest::Client,
) -> Result<String, ClientError> {
    let response = client.wd_navigate(base_url, name).send().await?;

    if !response.status().is_success() {
        return Err(ClientError::InvalidResponse(Box::new(response)));
    }

    Ok(response.text().await?)
}

/// Send events to the WebDynpro server and return the response
pub async fn send_events(
    state: &WebDynproState,
    client: &reqwest::Client,
    serialized_events: &str,
) -> Result<String, ClientError> {
    let response = client
        .wd_xhr(&state.client_url(), serialized_events.to_string())
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(ClientError::InvalidResponse(Box::new(response)));
    }

    Ok(response.text().await?)
}

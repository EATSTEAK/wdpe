use crate::body::Body;
use crate::error::ClientError;
use crate::{body::BodyUpdate, requests::WebDynproRequests};
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

impl WebDynproRequests for reqwest::Client {
    async fn navigate(&self, base_url: &Url, name: &str) -> Result<Body, ClientError> {
        let mut url = base_url.to_string();
        if !url.ends_with('/') {
            url.push('/');
        }
        url.push_str(name);
        url.push_str("?sap-wd-stableids=X#");

        let response = self
            .get(&url)
            .send()
            .await
            .map_err(|e| ClientError::FailedRequest(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ClientError::InvalidResponse(response.status().to_string()));
        }

        Ok(Body::new(
            response
                .text()
                .await
                .map_err(|e| ClientError::FailedRequest(e.to_string()))?,
        )?)
    }

    async fn send_events(
        &self,
        base_url: &Url,
        name: &str,
        serialized_events: &str,
    ) -> Result<BodyUpdate, ClientError> {
        let mut url = base_url.to_string();
        if !url.ends_with('/') {
            url.push('/');
        }
        url.push_str(name);
        url.push_str("?sap-wd-stableids=X#");

        let response = self
            .post(&url)
            .headers(wd_xhr_header())
            .body(serialized_events.to_string())
            .send()
            .await
            .map_err(|e| ClientError::FailedRequest(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ClientError::InvalidResponse(response.status().to_string()));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| ClientError::FailedRequest(e.to_string()))?;
        Ok(BodyUpdate::new(&response_text)?)
    }
}

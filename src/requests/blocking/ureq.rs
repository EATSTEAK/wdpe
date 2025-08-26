use crate::{
    body::{Body, BodyUpdate},
    error::ClientError,
    requests::blocking::WebDynproRequests,
};
use url::Url;

impl WebDynproRequests for ureq::Agent {
    fn navigate(&self, base_url: &Url, name: &str) -> Result<Body, ClientError> {
        let mut url = base_url.to_string();
        if !url.ends_with('/') {
            url.push('/');
        }
        url.push_str(name);
        url.push_str("?sap-wd-stableids=X#");

        let response = self
            .get(&url)
            .call()
            .map_err(|e| ClientError::FailedRequest(format!("Failed to navigate: {e}")))?;

        let status = response.status();
        if !status.is_success() {
            return Err(ClientError::FailedRequest(format!(
                "Navigation failed with status: {status}"
            )));
        }

        let body_str = response
            .into_body()
            .read_to_string()
            .map_err(|e| ClientError::InvalidResponse(format!("Failed to read response: {e}")))?;

        Ok(Body::new(body_str)?)
    }

    fn send_events(
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
            .header("Accept", "*/*")
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded; charset=UTF-8",
            )
            .header("X-Requested-With", "XMLHttpRequest")
            .send(serialized_events)
            .map_err(|e| ClientError::FailedRequest(format!("Failed to send events: {e}")))?;

        let status = response.status();
        if !status.is_success() {
            return Err(ClientError::FailedRequest(format!(
                "Send events failed with status: {status}"
            )));
        }

        let response_text = response
            .into_body()
            .read_to_string()
            .map_err(|e| ClientError::InvalidResponse(format!("Failed to read response: {e}")))?;
        Ok(crate::body::BodyUpdate::new(&response_text)?)
    }
}

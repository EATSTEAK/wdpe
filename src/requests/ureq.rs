use crate::requests::WebDynproRequestsSync;
use crate::{body::BodyUpdate, error::ClientError};
use ureq::{Agent, AgentBuilder};
use url::Url;

impl WebDynproRequestsSync for ureq::Agent {
    fn navigate(&self, base_url: &Url, name: &str) -> Result<String, ClientError> {
        let mut url = base_url.to_string();
        if !url.ends_with('/') {
            url.push('/');
        }
        url.push_str(name);
        url.push_str("?sap-wd-stableids=X#");

        let response = self
            .get(&url)
            .call()
            .map_err(|e| ClientError::NetworkError(format!("Failed to navigate: {e}")))?;

        let status = response.status();
        if !(200..=299).contains(&status) {
            return Err(ClientError::NetworkError(format!(
                "Navigation failed with status: {status}"
            )));
        }

        response
            .into_string()
            .map_err(|e| ClientError::NetworkError(format!("Failed to read response: {e}")))
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
            .set("Accept", "*/*")
            .set(
                "Content-Type",
                "application/x-www-form-urlencoded; charset=UTF-8",
            )
            .set("X-Requested-With", "XMLHttpRequest")
            .send_string(serialized_events)
            .map_err(|e| ClientError::NetworkError(format!("Failed to send events: {e}")))?;

        let status = response.status();
        if !(200..=299).contains(&status) {
            return Err(ClientError::NetworkError(format!(
                "Send events failed with status: {status}"
            )));
        }

        let response_text = response
            .into_string()
            .map_err(|e| ClientError::NetworkError(format!("Failed to read response: {e}")))?;
        Ok(crate::body::BodyUpdate::new(&response_text)?)
    }
}

/// Create a new ureq Agent with default configuration
pub fn create_ureq_agent() -> Agent {
    AgentBuilder::new()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
}

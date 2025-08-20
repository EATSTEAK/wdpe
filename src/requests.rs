use crate::body::BodyUpdate;
use crate::error::ClientError;
use crate::state::WebDynproState;
use url::Url;

/// Submodule for reqwest-based async HTTP requests
#[cfg(feature = "reqwest")]
pub mod reqwest;
/// Submodule for ureq-based sync HTTP requests
pub mod ureq;

/// Generic WebDynpro client that combines HTTP functionality with state management
pub struct WebDynproClientAsync<T> {
    state: WebDynproState,
    http_client: T,
}

impl<T: WebDynproRequestsAsync> WebDynproClientAsync<T> {
    /// Create a new WebDynproClient with the given state and HTTP client
    pub fn new(state: WebDynproState, http_client: T) -> Self {
        Self { state, http_client }
    }

    /// Get reference to the current state
    pub fn state(&self) -> &WebDynproState {
        &self.state
    }

    /// Get mutable reference to the state
    pub fn state_mut(&mut self) -> &mut WebDynproState {
        &mut self.state
    }

    /// Get reference to the HTTP client
    pub fn http_client(&self) -> &T {
        &self.http_client
    }
}

/// Async trait for WebDynpro HTTP requests using reqwest
pub trait WebDynproRequestsAsync {
    /// Navigate to a WebDynpro application and return the HTML body
    async fn navigate(&self, base_url: &Url, name: &str) -> Result<String, ClientError>;

    /// Send events to the WebDynpro server and return the response
    async fn send_events(
        &self,
        base_url: &Url,
        name: &str,
        serialized_events: &str,
    ) -> Result<BodyUpdate, ClientError>;
}

/// Sync trait for WebDynpro HTTP requests using ureq
pub trait WebDynproRequestsSync {
    /// Navigate to a WebDynpro application and return the HTML body
    fn navigate(&self, base_url: &Url, name: &str) -> Result<String, ClientError>;

    /// Send events to the WebDynpro server and return the response
    fn send_events(
        &self,
        base_url: &Url,
        name: &str,
        serialized_events: &str,
    ) -> Result<BodyUpdate, ClientError>;
}

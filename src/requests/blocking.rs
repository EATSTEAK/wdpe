use url::Url;

use crate::{
    body::{Body, BodyUpdate},
    error::ClientError,
    state::SapSsrClient,
};

#[cfg(feature = "ureq")]
pub mod ureq;

/// WebDynpro 서버에 요청하여 응답을 동기적으로 반환하는 트레이트
pub trait WebDynproRequests {
    /// WebDynpro 애플리케이션으로 이동하고 HTML 본문을 반환합니다.
    fn navigate(&self, base_url: &Url, name: &str) -> Result<Body, ClientError>;

    /// WebDynpro 서버에 이벤트를 전송하고 응답을 반환합니다.
    fn send_events(
        &self,
        base_url: &Url,
        ssr_client: &SapSsrClient,
        serialized_events: &str,
    ) -> Result<BodyUpdate, ClientError>;
}

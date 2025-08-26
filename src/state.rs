use crate::body::{Body, BodyUpdate};
use crate::error::{ClientError, WebDynproError};
use crate::event::{
    Event,
    event_queue::{EnqueueEventResult, EventQueue},
};
use tokio::sync::Mutex;
use url::Url;

/// WebDynpro 애플리케이션의 상태를 관리하는 구조체
#[derive(Debug)]
pub struct WebDynproState {
    base_url: Url,
    name: String,
    body: Body,
    event_queue: Mutex<EventQueue>,
}

impl WebDynproState {
    /// 새로운 `WebDynproState`를 생성합니다.
    pub fn new(base_url: Url, name: String, body: Body) -> Self {
        WebDynproState {
            base_url,
            name,
            body,
            event_queue: Mutex::new(EventQueue::new()),
        }
    }

    /// WebDynpro 애플리케이션의 이름을 반환합니다.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// WebDynpro 애플리케이션의 기본 URL을 반환합니다.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// WebDynpro 애플리케이션의 페이지 문서를 반환합니다.
    pub fn body(&self) -> &Body {
        &self.body
    }

    /// 실제로 요청하는 애플리케이션의 URL을 반환합니다.
    pub fn client_url(&self) -> String {
        let mut url = "".to_owned();
        url.push_str(self.base_url().as_str());
        if !url.ends_with('/') {
            url.push('/');
        }
        url.push_str(self.name());
        url.push_str("?sap-wd-stableids=X#");
        url
    }

    /// Body에 BodyUpdate를 적용합니다.
    pub fn mutate_body(&mut self, update: BodyUpdate) -> Result<(), WebDynproError> {
        Ok(self.body.apply(update)?)
    }

    /// 이벤트를 이벤트 큐에 추가합니다.
    pub async fn add_event(&self, event: Event) -> EnqueueEventResult {
        self.event_queue.lock().await.add(event)
    }

    /// 이벤트 큐의 내용을 직렬화하고 큐를 비웁니다.
    pub async fn serialize_and_clear(&self) -> String {
        self.event_queue.lock().await.serialize_and_clear()
    }

    /// 이벤트 큐의 내용을 Form 이벤트와 함께 직렬화하고 큐를 비웁니다.
    pub async fn serialize_and_clear_with_form_event(&self) -> Result<String, ClientError> {
        self.event_queue
            .lock()
            .await
            .serialize_and_clear_with_form_event()
    }
}

/// SSR 클라이언트 정보를 담는 구조체
#[derive(Debug)]
pub struct SapSsrClient {
    pub action: String,
    pub charset: String,
    pub wd_secure_id: String,
    pub app_name: String,
    pub use_beacon: bool,
}

impl SapSsrClient {
    pub fn build_action_url(&self, base_url: &Url) -> Result<String, ClientError> {
        let mut url = "".to_owned();
        url.push_str(base_url.scheme());
        url.push_str("://");
        if let Some(host_str) = base_url.host_str() {
            url.push_str(host_str);
        } else {
            return Err(ClientError::InvalidBaseUrl(base_url.to_string()));
        }
        if let Some(port) = base_url.port() {
            url.push(':');
            url.push_str(port.to_string().as_str());
        }
        url.push_str(self.action.as_str());
        Ok(url)
    }
}

/// 전달받은 이벤트가 어떻게 처리되었는지 표현합니다.
pub enum EventProcessResult {
    /// 전달받은 이벤트가 큐에 추가되었을 경우
    Enqueued,
    /// 전달받은 이벤트가 큐에 추가된 후 서버에 전송되었을 경우
    Sent,
}

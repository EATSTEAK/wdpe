# WDPE - WebDynpro Parse Engine

WDPE는 WebDynpro 페이지 스크래핑과 입력 조작을 위한 기반 엔진입니다.
WebDynpro 의 Lightspeed 라이브러리를 통해 구현되는 페이지에 대한 분석을 수행할 수 있습니다.

## 엘리먼트 정의 시스템

WDPE는 `wdpe-macros` proc-macro crate를 통해 WebDynpro 엘리먼트를 선언적으로 정의합니다.

| Derive / Attribute        | 용도                                                                            |
| ------------------------- | ------------------------------------------------------------------------------- |
| `#[derive(WdElement)]`    | 엘리먼트 struct, Definition, `Element` trait, `inventory` 자동 등록 생성        |
| `#[derive(WdLsData)]`     | LsData struct의 serde `Deserialize`, `Clone`, `Debug`, `Default`, accessor 생성 |
| `#[derive(WdSubElement)]` | 서브 엘리먼트 struct, Definition, `SubElement` trait 생성                       |
| `#[wd_event]`             | 이벤트 발생 메서드 body 자동 생성                                               |

### 새 엘리먼트 추가 예시

```rust,ignore
use std::{borrow::Cow, cell::OnceCell};
use wdpe::{WdElement, WdLsData, wd_event};

#[derive(WdLsData)]
#[allow(unused)]
pub struct ButtonLSData {
    #[wd_lsdata(index = "0")]
    text: Option<String>,
    #[wd_lsdata(index = "1")]
    enabled: Option<bool>,
}

#[derive(WdElement)]
#[wd_element(control_id = "B", element_name = "Button")]
#[wd_element(interactable)]
#[wd_element(def = "ButtonDef", def_doc = "[`Button`]의 정의")]
#[wd_element(lsdata = "ButtonLSData")]
pub struct Button<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<ButtonLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<wdpe::element::EventParameterMap>>,
}

impl<'a> Button<'a> {
    #[wd_event(name = "Press")]
    pub fn press(&self) {}
}
```

파일을 생성하고 모듈에 추가하면 `inventory` crate를 통해 자동으로 등록됩니다. 수동 등록이 필요하지 않습니다.

## WebDynpro 페이지 분석하기

직접 WebDynpro 페이지를 분석하여 이에 대한 자동화된 애플리케이션을 구현하려면, 애플리케이션에서 조작하고자 하는 주요 요소들에 대해 정의하는 작업이 선행되어야 합니다.

대부분의 조작하고자 하는 요소는 WebDynpro 엘리먼트로 구성되어 있으므로, 조작/분석하고자 하는 엘리먼트의 종류, ID를 미리 컴파일 타임에 정의하여 파싱/조작 작업을 원활히 수행할 수 있습니다.

### 페이지 내 엘리먼트가 정적인 ID를 가지도록 하기

기본적으로 WebDynpro 페이지의 엘리먼트 ID는 동적으로 구성되도록 되어 있습니다. 대략적으로 엘리먼트가 렌더링되는 순서대로 ID가 부여되며, 따라서 동일한 위치에 있는 동일한 엘리먼트임에도 불구하고 수행한 동작의 순서나 종류에 따라 다른 ID를 가질 수 있습니다.

이러한 현상을 방지하려면 WebDynpro 에서 자동 테스팅 목적으로 제공하는 파라메터 `SAP-WD-STABLEIDS`를 `X` 로 표시하면 정적인 ID를 가진 페이지를 확인할 수 있습니다.
[예시](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2100?sap-wd-stableids=x)

### 엘리먼트의 종류 알아내기

엘리먼트의 종류는 해당 엘리먼트 태그의 `ct` HTML 어트리뷰트 값을 rusaint WebDynpro 엘리먼트의 [`CONTROL_ID`]와 매칭시켜 알아낼 수 있습니다.

> **예시:** `ct="B"` 어트리뷰트의 경우 WebDynpro 엘리먼트의 [`Button`]과 매칭됩니다.

엘리먼트의 종류나 렌더링 방법에 따라 엘리먼트 자체를 감싸는 태그가 있을 수 있습니다. 이런 태그는 보통 해당 엘리먼트의 ID 뒤에 `-r` 접미사가 붙는것으로 확인할 수 있습니다.

## 애플리케이션 정의 예시

[예시](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2100?sap-wd-stableids=x&sap-language=ko) 애플리케이션 상단의 "담당자문의 정보" 텍스트를 파싱하는 예시 애플리케이션입니다.

추가 정보는 [`LoadingPlaceholder`], [`ClientInspector`] 와 [`Custom`] 엘리먼트를 참고하십시오.

```rust
use futures::executor::block_on;
use wdpe::event::event_queue::EnqueueEventResult;
use wdpe::requests::WebDynproRequests as _;
use wdpe::requests::{EventProcessResult, WebDynproState};
use wdpe::{define_elements, element::{text::Caption, system::{ClientInspector, Custom, CustomClientInfo, LoadingPlaceholder}}};

// 상태 관리를 위한 `WebDynproState`, 웹 요청을 위한 `reqwest::Client`를 가진 ExampleApplication
pub struct ExampleApplication {
  client: WebDynproState,
  client: reqwest::Client
};

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36";
const INITIAL_CLIENT_DATA_WD01: &str = "ClientWidth:1920px;ClientHeight:1000px;ScreenWidth:1920px;ScreenHeight:1080px;ScreenOrientation:landscape;ThemedTableRowHeight:33px;ThemedFormLayoutRowHeight:32px;ThemedSvgLibUrls:{\"SAPGUI-icons\":\"https://ecc.ssu.ac.kr:8443/sap/public/bc/ur/nw5/themes/~cache-20210223121230/Base/baseLib/sap_fiori_3/svg/libs/SAPGUI-icons.svg\",\"SAPWeb-icons\":\"https://ecc.ssu.ac.kr:8443/sap/public/bc/ur/nw5/themes/~cache-20210223121230/Base/baseLib/sap_fiori_3/svg/libs/SAPWeb-icons.svg\"};ThemeTags:Fiori_3,Touch;ThemeID:sap_fiori_3;SapThemeID:sap_fiori_3;DeviceType:DESKTOP";
const INITIAL_CLIENT_DATA_WD02: &str = "ThemedTableRowHeight:25px";

impl<'a> ExampleApplication {
    // 엘리먼트를 정의하는 매크로
    define_elements! {
        // 담당자문의 정보에 해당하는 캡션의 ID 정의
        CAPTION: Caption<'a> = "ZCMW_DEVINFO_RE.ID_D080C16F227F4D68751326DC40BB6BE0:MAIN.CAPTION";
        CLIENT_INSPECTOR_WD01: ClientInspector<'a> = "WD01";
        CLIENT_INSPECTOR_WD02: ClientInspector<'a> = "WD02";
        LOADING_PLACEHOLDER: LoadingPlaceholder<'a> = "_loadingPlaceholder_";
    }

    // 애플리케이션의 생성자
    pub async fn new() -> Result<ExampleApplication, WebDynproError> {
        let client = reqwest::Client::builder()
                .user_agent(DEFAULT_USER_AGENT)
                .build()
                .unwrap();
        let body = client.navigate(&base_url, name).await?;
        let state = WebDynproState::new(base_url, name.to_string(), body);
        let mut app = ExampleApplication { state, client };
        app.load_placeholder().await?;
        Ok(app)
    }


    // 캡션의 데이터를 읽는 함수
    pub fn read_caption(&self) -> Result<String, WebDynproError> {
        // 엘리먼트 파서를 생성
        let parser = ElementParser::new(self.client.body());
        // 캡션 정의와 현재 애플리케이션 바디로부터 엘리먼트 객체 생성
        let caption = parser.element_from_def(&Self::CAPTION)?;
        // 캡션 엘리먼트로부터 텍스트를 반환
        Ok(caption.text().to_string())
    }

    // 이벤트를 처리합니다.
    pub async fn process_event(
        &mut self,
        force_send: bool,
        event: Event,
    ) -> Result<EventProcessResult, WebDynproError> {
        let enqueue_result = self.state.add_event(event).await;

        if (matches!(enqueue_result, EnqueueEventResult::ShouldProcess)) || force_send {
            let serialized_events = self.state.serialize_and_clear_with_form_event().await?;
            let update = {
                self.client
                    .send_events(
                        self.state.base_url(),
                        self.state.body().ssr_client(),
                        &serialized_events,
                    )
                    .await?
            };
            let result = self.state.mutate_body(update)?;
            Ok(EventProcessResult::Sent(result))
        } else {
            Ok(EventProcessResult::Enqueued)
        }
    }

    // 페이지 플레이스홀더 로드
    async fn load_placeholder(&mut self) -> Result<(), WebDynproError> {
        let parser = ElementParser::new(self.body());
        let notify_wd01 = parser.read(ClientInspectorNotifyEventCommand::new(
            Self::CLIENT_INSPECTOR_WD01,
            INITIAL_CLIENT_DATA_WD01,
        ))?;
        let notify_wd02 = parser.read(ClientInspectorNotifyEventCommand::new(
            Self::CLIENT_INSPECTOR_WD02,
            INITIAL_CLIENT_DATA_WD02,
        ))?;
        let load = parser.read(LoadingPlaceholderLoadEventCommand::new(
            Self::LOADING_PLACEHOLDER,
        ))?;
        let custom = parser.read(CustomClientInfoEventCommand::new(
            Self::CUSTOM,
            CustomClientInfo {
                client_url: self.client_url(),
                document_domain: "ssu.ac.kr".to_owned(),
                ..CustomClientInfo::default()
            },
        ))?;
        self.process_event(false, notify_wd01).await?;
        self.process_event(false, notify_wd02).await?;
        self.process_event(false, load).await?;
        self.process_event(false, custom).await?;
        Ok(())
    }
}

async fn test_caption() {
    let app = ExampleApplication::new().await.unwrap();
    let text = app.read_caption().unwrap();
    assert_eq!(text, "담당자문의 정보");
}

fn main() {
    block_on(test_caption());
}
```

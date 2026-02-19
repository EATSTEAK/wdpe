use lol_html::{RewriteStrSettings, element, html_content::ContentType, rewrite_str};
use roxmltree::Node;
use std::collections::HashMap;
use std::hash::Hash;

use crate::error::{BodyError, UpdateBodyError};

use crate::state::SapSsrClient;

type BodyUpdateWindowId = String;
type BodyUpdateContentId = String;
type BodyUpdateControlId = String;

/// 바디 업데이트 유형 구조체
#[derive(Debug)]
pub enum BodyUpdateType {
    /// 제공된 BodyUpdate가 페이지 전체를 업데이트 할 경우
    Full(BodyUpdateWindowId, BodyUpdateContentId, String),
    /// 제공된 BodyUpdate가 일부 컨트롤만 업데이트 할 경우
    Delta(BodyUpdateWindowId, HashMap<BodyUpdateControlId, String>),
}

/// [`Body`] 업데이트 후 반환되는 부가 정보 구조체
#[derive(Debug, Clone, Default)]
pub struct BodyUpdateResult {
    /// 서버에서 반환된 스크립트 호출 목록
    pub script_calls: Option<Vec<String>>,
    /// 서버에서 반환된 초기화 ID
    pub initialize_ids: Option<String>,
    /// 서버에서 반환된 모델 업데이트 목록
    pub model_updates: Option<Vec<String>>,
    /// 서버에서 반환된 애니메이션 업데이트 목록
    pub animation_updates: Option<Vec<String>>,
}

/// [`Body`]를 업데이트 하기 위한 데이터 구조체
#[derive(Debug)]
pub struct BodyUpdate {
    update: Option<BodyUpdateType>,
    auxiliary: BodyUpdateResult,
}

/// 빈 `Vec`를 `None`으로, 비어있지 않은 `Vec`를 `Some`으로 변환하는 헬퍼 함수
fn non_empty(v: Vec<String>) -> Option<Vec<String>> {
    if v.is_empty() { None } else { Some(v) }
}

/// `<updates>` 노드의 자식 중 auxiliary 노드의 텍스트를 수집하는 헬퍼 함수
fn collect_auxiliary_node(
    node: &Node,
    script_calls: &mut Vec<String>,
    initialize_ids: &mut Option<String>,
    model_updates: &mut Vec<String>,
    animation_updates: &mut Vec<String>,
) {
    match node.tag_name().name() {
        "script-call" => {
            if let Some(text) = node.text()
                && !text.is_empty()
            {
                script_calls.push(text.to_owned());
            }
        }
        "initialize-ids" => {
            if let Some(text) = node.text()
                && !text.is_empty()
            {
                *initialize_ids = Some(text.to_owned());
            }
        }
        "model-update" => {
            if let Some(text) = node.text()
                && !text.is_empty()
            {
                model_updates.push(text.to_owned());
            }
        }
        "animation-update" => {
            if let Some(text) = node.text()
                && !text.is_empty()
            {
                animation_updates.push(text.to_owned());
            }
        }
        _ => {}
    }
}

impl BodyUpdate {
    /// 새로운 `BodyUpdate`를 생성합니다.
    pub fn new(response: &str) -> Result<BodyUpdate, UpdateBodyError> {
        let response_xml = roxmltree::Document::parse(response)?;
        let updates = response_xml
            .root()
            .first_child()
            .ok_or(UpdateBodyError::NoSuchNode("<updates>".to_string()))?;

        let mut update_type: Option<BodyUpdateType> = None;
        let mut initialize_ids: Option<String> = None;
        let mut script_calls: Vec<String> = Vec::new();
        let mut model_updates: Vec<String> = Vec::new();
        let mut animation_updates: Vec<String> = Vec::new();

        for child in updates.children() {
            match child.tag_name().name() {
                "full-update" => {
                    if update_type.is_some() {
                        tracing::warn!(
                            "Multiple update nodes found in <updates>, overwriting previous update."
                        );
                    }
                    let windowid =
                        child
                            .attribute("windowid")
                            .ok_or(UpdateBodyError::NoSuchAttribute {
                                node: "full-update".to_string(),
                                attribute: "windowid".to_string(),
                            })?;
                    
                    let mut content_id: Option<String> = None;
                    let mut content_text: Option<String> = None;
                    
                    for full_child in child.children() {
                        let tag_name = full_child.tag_name().name();
                        match tag_name {
                            "content-update" => {
                                let contentid =
                                    full_child
                                        .attribute("id")
                                        .ok_or(UpdateBodyError::NoSuchAttribute {
                                            node: "content-update".to_string(),
                                            attribute: "id".to_string(),
                                        })?;
                                let text = full_child
                                    .text()
                                    .ok_or(UpdateBodyError::NoSuchContent("content-update".to_string()))?;
                                content_id = Some(contentid.to_owned());
                                content_text = Some(text.to_owned());
                            }
                            "script-call" | "initialize-ids" | "model-update"
                            | "animation-update" => {
                                collect_auxiliary_node(
                                    &full_child,
                                    &mut script_calls,
                                    &mut initialize_ids,
                                    &mut model_updates,
                                    &mut animation_updates,
                                );
                            }
                            "" => {
                                // Text-only node (whitespace between elements), skip
                            }
                            unknown => {
                                tracing::warn!(
                                    "Unknown full-update child {unknown} is found, ignore."
                                );
                            }
                        }
                    }
                    
                    let content_id = content_id
                        .ok_or(UpdateBodyError::NoSuchContent("full-update".to_string()))?;
                    let content_text = content_text
                        .ok_or(UpdateBodyError::NoSuchContent("full-update".to_string()))?;
                    
                    update_type = Some(BodyUpdateType::Full(
                        windowid.to_owned(),
                        content_id,
                        content_text,
                    ));
                }
                "delta-update" => {
                    if update_type.is_some() {
                        tracing::warn!(
                            "Multiple update nodes found in <updates>, overwriting previous update."
                        );
                    }
                    let windowid =
                        child
                            .attribute("windowid")
                            .ok_or(UpdateBodyError::NoSuchAttribute {
                                node: "delta-update".to_string(),
                                attribute: "windowid".to_string(),
                            })?;
                    let delta_children = child.children().collect::<Vec<Node>>();
                    let mut update_map: HashMap<BodyUpdateControlId, String> =
                        HashMap::with_capacity(delta_children.len());
                    for delta_child in delta_children {
                        let tag_name = delta_child.tag_name().name();
                        match tag_name {
                            "control-update" => {
                                let control_id = delta_child.attribute("id").ok_or(
                                    UpdateBodyError::NoSuchAttribute {
                                        node: "control-update".to_string(),
                                        attribute: "id".to_string(),
                                    },
                                )?;
                                let content = delta_child.first_child().ok_or(
                                    UpdateBodyError::NoSuchContent("control-update".to_string()),
                                )?;
                                update_map.insert(
                                    control_id.to_owned(),
                                    content
                                        .text()
                                        .ok_or(UpdateBodyError::NoSuchContent(
                                            "content".to_string(),
                                        ))?
                                        .to_owned(),
                                );
                            }
                            "script-call" | "initialize-ids" | "model-update"
                            | "animation-update" => {
                                collect_auxiliary_node(
                                    &delta_child,
                                    &mut script_calls,
                                    &mut initialize_ids,
                                    &mut model_updates,
                                    &mut animation_updates,
                                );
                            }
                            _ => {
                                tracing::warn!(
                                    "Unknown delta-update child {tag_name} is found, ignore."
                                );
                            }
                        };
                    }
                    update_type = Some(BodyUpdateType::Delta(windowid.to_owned(), update_map));
                }
                "script-call" | "initialize-ids" | "model-update" | "animation-update" => {
                    collect_auxiliary_node(
                        &child,
                        &mut script_calls,
                        &mut initialize_ids,
                        &mut model_updates,
                        &mut animation_updates,
                    );
                }
                "" => {
                    // Text-only node (whitespace between elements), skip
                }
                unknown => {
                    tracing::warn!("Unknown update node {unknown} is found, ignore.");
                }
            }
        }

        Ok(BodyUpdate {
            update: update_type,
            auxiliary: BodyUpdateResult {
                initialize_ids,
                script_calls: non_empty(script_calls),
                model_updates: non_empty(model_updates),
                animation_updates: non_empty(animation_updates),
            },
        })
    }

    /// 업데이트 후 반환되는 부가 정보를 반환합니다.
    pub fn auxiliary(&self) -> &BodyUpdateResult {
        &self.auxiliary
    }
}

/// WebDynpro 페이지의 상태를 관리하는 구조체
#[derive(custom_debug_derive::Debug)]
pub struct Body {
    #[debug(skip)]
    raw_body: String,
    sap_ssr_client: SapSsrClient,
}

impl Hash for Body {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw_body.hash(state);
    }
}

impl Body {
    /// 새로운 `Body`를 Raw HTML 문자열으로 부터 생성합니다.
    pub fn new(raw_body: String) -> Result<Body, BodyError> {
        let sap_ssr_client = parse_sap_ssr_client(&raw_body)?;
        Ok(Body {
            raw_body,
            sap_ssr_client,
        })
    }

    /// 페이지 도큐먼트의 HTML 텍스트를 반환합니다.
    pub fn raw_body(&self) -> &str {
        &self.raw_body
    }

    pub fn ssr_client(&self) -> &SapSsrClient {
        &self.sap_ssr_client
    }

    pub(super) fn apply(
        &mut self,
        updates: BodyUpdate,
    ) -> Result<BodyUpdateResult, UpdateBodyError> {
        if let Some(update) = updates.update {
            let output: String = match update {
                BodyUpdateType::Full(_, contentid, content) => {
                    let element_content_handlers =
                        vec![element!(format!(r#"[id="{}"]"#, contentid), |el| {
                            el.set_inner_content(&content, ContentType::Html);
                            Ok(())
                        })];
                    rewrite_str(
                        &self.raw_body,
                        RewriteStrSettings {
                            element_content_handlers,
                            ..RewriteStrSettings::default()
                        },
                    )?
                }
                BodyUpdateType::Delta(windowid, controls) => {
                    let element_content_handlers = controls
                        .iter()
                        .map(|(control_id, content)| {
                            element!(
                                format!(r#"[id="{}_root_"] [id="{}"]"#, windowid, control_id),
                                move |el| {
                                    el.replace(content, ContentType::Html);
                                    Ok(())
                                }
                            )
                        })
                        .collect();
                    rewrite_str(
                        &self.raw_body,
                        RewriteStrSettings {
                            element_content_handlers,
                            ..RewriteStrSettings::default()
                        },
                    )?
                }
            };
            self.raw_body = output;
        }
        Ok(updates.auxiliary)
    }
}

fn parse_sap_ssr_client(document: &str) -> Result<SapSsrClient, BodyError> {
    let form_regex = regex_lite::Regex::new(r"<form\b[^>]*>(.|\n)*?<\/form>").unwrap();
    let mut forms = form_regex.find_iter(document);
    let form_match = forms
        .find(|form| form.as_str().contains("sap.client.SsrClient.form"))
        .ok_or(BodyError::Invalid(
            "Cannot find SSR Client form".to_string(),
        ))?
        .as_str();

    // Create closing tag to match xml structures
    let input_regex = regex_lite::Regex::new(r"(<input\b[^>]*)(/?>)").unwrap();
    let form_xml = input_regex.replace_all(form_match, "$1></input>");

    let client_form = roxmltree::Document::parse(&form_xml).unwrap();

    let mut data = HashMap::<String, String>::new();
    data.insert(
        "action".to_owned(),
        client_form
            .root_element()
            .attribute("action")
            .expect("Attribute not found or malformed")
            .to_string(),
    );

    let children_iter = client_form.root_element().children();
    children_iter.for_each(|item| {
        let id = item
            .attribute("id")
            .expect("id Attribute not found or malformed")
            .to_string();
        let value = item
            .attribute("value")
            .expect("value Attribute not found or malformed")
            .to_string();
        data.insert(id, value);
    });
    Ok(SapSsrClient {
        action: html_escape::decode_html_entities(data.get("action").ok_or(
            BodyError::NoSuchAttribute("'action' field of SSR Form".to_string()),
        )?)
        .to_string(),
        charset: data
            .get("sap-charset")
            .ok_or(BodyError::NoSuchAttribute(
                "'sap-charset' field of SSR Form".to_string(),
            ))?
            .to_owned(),
        wd_secure_id: data
            .get("sap-wd-secure-id")
            .ok_or(BodyError::NoSuchAttribute(
                "'sap-wd-secure-id' field of SSR Form".to_string(),
            ))?
            .to_owned(),
        app_name: data
            .get("fesrAppName")
            .ok_or(BodyError::NoSuchAttribute(
                "'fesrAppName' field of SSR Form".to_string(),
            ))?
            .to_owned(),
        use_beacon: data
            .get("fesrUseBeacon")
            .ok_or(BodyError::NoSuchAttribute(
                "'fesrUseBeacon' field of SSR Form".to_string(),
            ))?
            .to_owned()
            .as_str()
            == "true",
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "reqwest")]
    #[tokio::test]
    async fn test_ssr_form() {
        use crate::requests::WebDynproRequests as _;
        use reqwest::cookie::Jar;
        use std::sync::Arc;
        use url::Url;

        const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36";

        let jar = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(jar)
            .cookie_store(true)
            .user_agent(DEFAULT_USER_AGENT)
            .build()
            .unwrap();
        let result = client
            .navigate(
                &Url::parse("https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/").unwrap(),
                "ZCMW2100",
            )
            .await
            .unwrap();
        let ssr_client = parse_sap_ssr_client(result.raw_body()).unwrap();
        dbg!(ssr_client);
    }

    #[test]
    fn test_body_update_full_with_script_call() {
        let xml = r#"<updates>
            <full-update windowid="WID1"><content-update id="content1">Hello World</content-update></full-update>
            <script-call>alert('hello');</script-call>
        </updates>"#;

        let update = BodyUpdate::new(xml).unwrap();
        assert!(update.update.is_some());
        assert!(
            matches!(update.update.as_ref().unwrap(), BodyUpdateType::Full(w, c, _) if w == "WID1" && c == "content1")
        );
        assert_eq!(
            update.auxiliary.script_calls.as_deref(),
            Some(["alert('hello');".to_string()].as_slice())
        );
        assert!(update.auxiliary.initialize_ids.is_none());
        assert!(update.auxiliary.model_updates.is_none());
        assert!(update.auxiliary.animation_updates.is_none());
    }

    #[test]
    fn test_body_update_delta_with_auxiliary_nodes() {
        let xml = r#"<updates>
            <delta-update windowid="WID2">
                <control-update id="ctrl1"><content>Updated</content></control-update>
                <script-call>doSomething();</script-call>
            </delta-update>
            <initialize-ids>ID123</initialize-ids>
            <model-update>model_data_1</model-update>
            <model-update>model_data_2</model-update>
            <animation-update>anim_data</animation-update>
        </updates>"#;

        let update = BodyUpdate::new(xml).unwrap();
        assert!(update.update.is_some());
        assert!(
            matches!(update.update.as_ref().unwrap(), BodyUpdateType::Delta(w, _) if w == "WID2")
        );
        // script-call inside delta-update + none at top level
        assert_eq!(
            update.auxiliary.script_calls.as_deref(),
            Some(["doSomething();".to_string()].as_slice())
        );
        assert_eq!(update.auxiliary.initialize_ids.as_deref(), Some("ID123"));
        assert_eq!(
            update.auxiliary.model_updates.as_deref(),
            Some(["model_data_1".to_string(), "model_data_2".to_string()].as_slice())
        );
        assert_eq!(
            update.auxiliary.animation_updates.as_deref(),
            Some(["anim_data".to_string()].as_slice())
        );
    }

    #[test]
    fn test_body_update_no_update_node_only_auxiliary() {
        let xml = r#"<updates>
            <script-call>script1();</script-call>
            <script-call>script2();</script-call>
        </updates>"#;

        let update = BodyUpdate::new(xml).unwrap();
        assert!(update.update.is_none());
        assert_eq!(
            update.auxiliary.script_calls.as_deref(),
            Some(["script1();".to_string(), "script2();".to_string()].as_slice())
        );
    }

    #[test]
    fn test_body_update_empty_text_ignored() {
        let xml = r#"<updates>
            <script-call></script-call>
            <initialize-ids></initialize-ids>
            <model-update></model-update>
            <animation-update></animation-update>
        </updates>"#;

        let update = BodyUpdate::new(xml).unwrap();
        assert!(update.update.is_none());
        assert!(update.auxiliary.script_calls.is_none());
        assert!(update.auxiliary.initialize_ids.is_none());
        assert!(update.auxiliary.model_updates.is_none());
        assert!(update.auxiliary.animation_updates.is_none());
    }

    #[test]
    fn test_body_update_accessor() {
        let xml = r#"<updates>
            <script-call>test();</script-call>
            <initialize-ids>INIT1</initialize-ids>
        </updates>"#;

        let update = BodyUpdate::new(xml).unwrap();
        let aux = update.auxiliary();
        assert_eq!(
            aux.script_calls.as_deref(),
            Some(["test();".to_string()].as_slice())
        );
        assert_eq!(aux.initialize_ids.as_deref(), Some("INIT1"));
    }

    #[test]
    fn test_body_update_full_with_auxiliary_nodes_inside() {
        let xml = r#"<updates>
            <full-update windowid="WID1">
                <content-update id="content1">Hello World</content-update>
                <script-call>insideScript();</script-call>
                <initialize-ids>INIT123</initialize-ids>
            </full-update>
            <script-call>outsideScript();</script-call>
        </updates>"#;

        let update = BodyUpdate::new(xml).unwrap();
        assert!(update.update.is_some());
        assert!(
            matches!(update.update.as_ref().unwrap(), BodyUpdateType::Full(w, c, _) if w == "WID1" && c == "content1")
        );
        // Should collect both script-calls: one inside full-update and one outside
        assert_eq!(
            update.auxiliary.script_calls.as_deref(),
            Some(
                [
                    "insideScript();".to_string(),
                    "outsideScript();".to_string()
                ]
                .as_slice()
            )
        );
        assert_eq!(update.auxiliary.initialize_ids.as_deref(), Some("INIT123"));
    }
}

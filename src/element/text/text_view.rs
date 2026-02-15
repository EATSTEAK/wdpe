use std::{borrow::Cow, cell::OnceCell};

use scraper::Node;

use crate::{WdElement, WdLsData, element::property::Visibility};

#[doc = "[`TextView`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct TextViewLSData {
    #[wd_lsdata(index = "0")]
    tooltip: Option<String>,
    #[wd_lsdata(index = "1")]
    required: Option<bool>,
    #[wd_lsdata(index = "2")]
    enabled: Option<bool>,
    #[wd_lsdata(index = "3")]
    design: Option<String>,
    #[wd_lsdata(index = "4")]
    layout: Option<String>,
    #[wd_lsdata(index = "5")]
    semantic_color: Option<String>,
    #[wd_lsdata(index = "6")]
    semantic_bg_color: Option<String>,
    #[wd_lsdata(index = "7")]
    is_nested: Option<bool>,
    #[wd_lsdata(index = "8")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "9")]
    text_overflow: Option<bool>,
}

#[doc = "텍스트 표시 뷰"]
#[derive(WdElement)]
#[wd_element(control_id = "TV", element_name = "TextView")]
#[wd_element(interactable, textisable)]
#[wd_element(def = "TextViewDef", def_doc = "[`TextView`]의 정의")]
#[wd_element(lsdata = "TextViewLSData")]
pub struct TextView<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<TextViewLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
    text: OnceCell<String>,
}

impl<'a> TextView<'a> {
    /// 내부 텍스트를 반환합니다.
    pub fn text(&self) -> &str {
        use crate::element::Element as _;
        self.text.get_or_init(|| {
            self.element_ref()
                .children()
                .filter_map(|node| match node.value() {
                    Node::Text(text) => Some(text.to_string()),
                    Node::Element(elem) => {
                        if elem.name() == "br" {
                            Some("\n".to_string())
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .collect::<String>()
        })
    }
}

impl std::fmt::Display for TextView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

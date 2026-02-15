use std::{borrow::Cow, cell::OnceCell};

use scraper::Node;

use crate::{WdElement, WdLsData, element::property::Visibility};

#[doc = "[`Caption`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct CaptionLSData {
    #[wd_lsdata(index = "0")]
    tooltip: Option<String>,
    #[wd_lsdata(index = "1")]
    text: Option<String>,
    #[wd_lsdata(index = "2")]
    image_src: Option<String>,
    #[wd_lsdata(index = "3")]
    image_first: Option<bool>,
    #[wd_lsdata(index = "4")]
    image_width: Option<String>,
    #[wd_lsdata(index = "5")]
    image_height: Option<String>,
    #[wd_lsdata(index = "6")]
    is_nested: Option<bool>,
    #[wd_lsdata(index = "7")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "8")]
    is_drag_handle: Option<bool>,
    #[wd_lsdata(index = "9")]
    hover_image_src: Option<String>,
    #[wd_lsdata(index = "10")]
    drag_source_info: Option<String>,
    #[wd_lsdata(index = "11")]
    editable: Option<bool>,
    #[wd_lsdata(index = "12")]
    semantic_color: Option<String>,
    #[wd_lsdata(index = "13")]
    design: Option<String>,
    #[wd_lsdata(index = "14")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "15")]
    labelled_by: Option<String>,
}

#[doc = "엘리먼트 제목 부분 등에서 사용되는 캡션"]
#[doc = ""]
#[doc = "이 엘리먼트는 단독 엘리먼트로 존재하지 않고, [`SapTableHeaderCell`]이나 [`Tray`]같은 엘리먼트의 제목 부분에 활용됩니다."]
#[doc = ""]
#[doc = "[`SapTableHeaderCell`]: crate::element::complex::sap_table::cell::SapTableHeaderCell"]
#[doc = "[`Tray`]: crate::element::layout::Tray"]
#[derive(WdElement)]
#[wd_element(control_id = "CP", element_name = "Caption")]
#[wd_element(interactable)]
#[wd_element(def = "CaptionDef", def_doc = "[`Caption`]의 정의")]
#[wd_element(lsdata = "CaptionLSData")]
pub struct Caption<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<CaptionLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
    text: OnceCell<String>,
}

impl<'a> Caption<'a> {
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

impl std::fmt::Display for Caption<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

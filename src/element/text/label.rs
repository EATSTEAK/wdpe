use std::{borrow::Cow, cell::OnceCell};

use crate::{
    element::property::Visibility,
    WdElement, WdLsData,
};

// TODO: Implement additional events and data
#[doc = "[`Label`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct LabelLSData {
    #[wd_lsdata(index = "0")]
    tooltip: Option<String>,
    #[wd_lsdata(index = "1")]
    label_for: Option<String>,
    #[wd_lsdata(index = "2")]
    wrapping: Option<bool>,
    #[wd_lsdata(index = "3")]
    text: Option<String>,
    #[wd_lsdata(index = "4")]
    required: Option<bool>,
    #[wd_lsdata(index = "5")]
    enabled: Option<bool>,
    #[wd_lsdata(index = "6")]
    design_bar: Option<String>,
    #[wd_lsdata(index = "7")]
    width: Option<String>,
    #[wd_lsdata(index = "8")]
    has_icon: Option<bool>,
    #[wd_lsdata(index = "9")]
    image_first: Option<bool>,
    #[wd_lsdata(index = "10")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "11")]
    show_help: Option<bool>,
    #[wd_lsdata(index = "12")]
    access_key: Option<String>,
    #[wd_lsdata(index = "13")]
    align: Option<String>,
    #[wd_lsdata(index = "14")]
    text_overflow: Option<bool>,
    #[wd_lsdata(index = "15")]
    required_indicator_at_front: Option<bool>,
    #[wd_lsdata(index = "16")]
    interaction_behavior: Option<String>,
    #[wd_lsdata(index = "17")]
    is_link: Option<bool>,
    #[wd_lsdata(index = "18")]
    editable: Option<bool>,
    #[wd_lsdata(index = "19")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "20")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "21")]
    height: Option<String>,
    #[wd_lsdata(index = "22")]
    labelled_by: Option<String>,
}

#[doc = "버튼 등의 엘리먼트를 부연하는 라벨"]
#[derive(WdElement)]
#[wd_element(control_id = "L", element_name = "Label")]
#[wd_element(interactable)]
#[wd_element(def = "LabelDef", def_doc = "[`Label`]의 정의")]
#[wd_element(lsdata = "LabelLSData")]
pub struct Label<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<LabelLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
    text: OnceCell<String>,
}

impl<'a> Label<'a> {
    /// 내부 텍스트를 반환합니다.
    pub fn text(&self) -> &str {
        use crate::element::Element as _;
        self.text
            .get_or_init(|| self.element_ref().text().collect::<String>())
    }
}

impl std::fmt::Display for Label<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

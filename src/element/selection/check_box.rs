use std::{borrow::Cow, cell::OnceCell};

use crate::{WdElement, WdLsData, element::property::Visibility};

#[doc = "[`CheckBox`]의 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct CheckBoxLSData {
    #[wd_lsdata(index = "0")]
    name: Option<String>,
    #[wd_lsdata(index = "1")]
    checked: Option<bool>,
    #[wd_lsdata(index = "2")]
    enabled: Option<bool>,
    #[wd_lsdata(index = "3")]
    readonly: Option<bool>,
    #[wd_lsdata(index = "4")]
    text: Option<String>,
    #[wd_lsdata(index = "5")]
    tooltip: Option<String>,
    #[wd_lsdata(index = "6")]
    invalid: Option<bool>,
    #[wd_lsdata(index = "7")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "8")]
    show_help: Option<bool>,
    #[wd_lsdata(index = "9")]
    input_state: Option<String>,
    #[wd_lsdata(index = "10")]
    access_key: Option<String>,
    #[wd_lsdata(index = "11")]
    arrangement: Option<String>,
    #[wd_lsdata(index = "12")]
    associated_edit_context: Option<String>,
    #[wd_lsdata(index = "13")]
    text_design: Option<String>,
    #[wd_lsdata(index = "14")]
    used_in_sap_table: Option<bool>,
    #[wd_lsdata(index = "15")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "16")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "17")]
    text_overflow: Option<bool>,
    #[wd_lsdata(index = "18")]
    height: Option<String>,
    #[wd_lsdata(index = "19")]
    is_text_label: Option<bool>,
    #[wd_lsdata(index = "20")]
    labelled_by: Option<String>,
}

#[doc = "체크박스"]
#[derive(WdElement)]
#[wd_element(control_id = "C_standards", element_name = "CheckBox")]
#[wd_element(interactable, textisable)]
#[wd_element(def = "CheckBoxDef", def_doc = "[`CheckBox`]의 정의")]
#[wd_element(lsdata = "CheckBoxLSData")]
pub struct CheckBox<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<CheckBoxLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
}

impl<'a> CheckBox<'a> {
    /// 이 [`CheckBox`]가 체크되었는지 여부를 반환합니다.
    pub fn checked(&self) -> bool {
        use crate::element::Element as _;
        self.element_ref()
            .attr("aria-checked")
            .is_some_and(|str| str == "true")
    }

    /// 이 [`CheckBox`]가 읽기 전용인지 여부를 반환합니다.
    pub fn readonly(&self) -> bool {
        use crate::element::Element as _;
        self.element_ref()
            .attr("aria-readonly")
            .is_some_and(|str| str == "true")
    }

    /// 이 [`CheckBox`]가 비활성화 상태인지 여부를 반환합니다.
    pub fn disabled(&self) -> bool {
        use crate::element::Element as _;
        self.element_ref()
            .attr("aria-disabled")
            .is_some_and(|str| str == "true")
    }

    /// 이 [`CheckBox`]가 올바르지 않은 상태인지 여부를 반환합니다.
    pub fn invalid(&self) -> bool {
        use crate::element::Element as _;
        self.element_ref()
            .attr("aria-invalid")
            .is_some_and(|str| str == "true")
    }
}

impl std::fmt::Display for CheckBox<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.checked())
    }
}

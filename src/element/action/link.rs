use std::{borrow::Cow, cell::OnceCell};

use crate::{
    WdElement, WdLsData,
    element::property::{HotkeyValue, Visibility},
    wd_event,
};

#[doc = "[`Link`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct LinkLSData {
    #[wd_lsdata(index = "0")]
    tooltip: Option<String>,
    #[wd_lsdata(index = "1")]
    text: Option<String>,
    #[wd_lsdata(index = "2")]
    has_reference: Option<bool>,
    #[wd_lsdata(index = "3")]
    enabled: Option<bool>,
    #[wd_lsdata(index = "4")]
    has_link_caption: Option<bool>,
    #[wd_lsdata(index = "5")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "6")]
    label_text: Option<String>,
    #[wd_lsdata(index = "7")]
    emphasized: Option<bool>,
    #[wd_lsdata(index = "8")]
    access_key: Option<String>,
    #[wd_lsdata(index = "9")]
    hotkey: Option<HotkeyValue>,
    #[wd_lsdata(index = "10")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "11")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "12")]
    labelled_by: Option<String>,
}

#[doc = "액션을 수행하거나 링크로 이동하는 하이퍼링크"]
#[derive(WdElement)]
#[wd_element(control_id = "LN", element_name = "Link")]
#[wd_element(interactable, textisable)]
#[wd_element(def = "LinkDef", def_doc = "[`Link`]의 정의")]
#[wd_element(lsdata = "LinkLSData")]
pub struct Link<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<LinkLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
    text: OnceCell<String>,
}

impl<'a> Link<'a> {
    /// 내부 텍스트를 반환합니다.
    pub fn text(&self) -> &str {
        use crate::element::Element as _;
        self.text
            .get_or_init(|| self.element_ref().text().collect::<String>())
    }

    /// 링크 활성화 이벤트를 반환합니다. `ctrl` 이나 `shift` 가 참일 경우 각 버튼을 누른 채로 클릭한 것으로 간주합니다.
    #[wd_event(name = "Activate", params(ctrl: bool => "Ctrl", shift: bool => "Shift"))]
    pub fn activate(&self, ctrl: bool, shift: bool) {}

    /// 더블 클릭 이벤트를 반환합니다.
    #[wd_event(name = "DoubleClick")]
    pub fn double_click(&self) {}
}

impl std::fmt::Display for Link<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

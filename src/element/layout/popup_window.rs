use std::{borrow::Cow, cell::OnceCell};

use crate::element::property::{Mode, QuickViewDesign};
use crate::{WdElement, WdLsData, wd_event};

// TODO: Implement additional events and data
#[doc = "[`PopupWindow`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct PopupWindowLSData {
    #[wd_lsdata(index = "0")]
    is_resizable: Option<bool>,
    #[wd_lsdata(index = "1")]
    has_close_button: Option<bool>,
    #[wd_lsdata(index = "2")]
    x: Option<String>,
    #[wd_lsdata(index = "3")]
    y: Option<String>,
    #[wd_lsdata(index = "4")]
    width: Option<String>,
    #[wd_lsdata(index = "5")]
    height: Option<String>,
    #[wd_lsdata(index = "6")]
    window_size: Option<bool>,
    #[wd_lsdata(index = "7")]
    default_button_id: Option<String>,
    #[wd_lsdata(index = "8")]
    hotkeys_id: Option<String>,
    #[wd_lsdata(index = "9")]
    override_minimum_size: Option<bool>,
    #[wd_lsdata(index = "10")]
    is_maximized: Option<bool>,
    #[wd_lsdata(index = "11")]
    has_help_button: Option<bool>,
    #[wd_lsdata(index = "12")]
    mode: Option<Mode>,
    #[wd_lsdata(index = "13")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "14")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "15")]
    no_content_scrolling: Option<bool>,
    #[wd_lsdata(index = "16")]
    context_menu_event: Option<bool>,
    #[wd_lsdata(index = "17")]
    design: Option<QuickViewDesign>,
}

#[doc = "브라우저 창 내부에 모달 등의 팝업으로 표시되는 창"]
#[derive(WdElement)]
#[wd_element(control_id = "PW", element_name = "PopupWindow")]
#[wd_element(interactable)]
#[wd_element(def = "PopupWindowDef", def_doc = "[`PopupWindow`]의 정의")]
#[wd_element(lsdata = "PopupWindowLSData")]
pub struct PopupWindow<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<PopupWindowLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
}

impl<'a> PopupWindow<'a> {
    /// 창을 닫는 이벤트를 반환합니다.
    #[wd_event(name = "Close")]
    pub fn close(&self) {}

    /// 도움 버튼을 누르는 이벤트를 반환합니다.
    #[wd_event(name = "Help")]
    pub fn help(&self) {}
}

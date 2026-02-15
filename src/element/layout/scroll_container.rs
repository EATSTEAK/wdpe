use std::{borrow::Cow, cell::OnceCell};

use crate::{
    WdElement, WdLsData,
    element::property::{ScrollingMode, Visibility},
};

// TODO: Implement additional events and data
#[doc = "[`ScrollContainer`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct ScrollContainerLSData {
    #[wd_lsdata(index = "0")]
    scrolling_mode: Option<ScrollingMode>,
    #[wd_lsdata(index = "1")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "2")]
    accessibility_description: Option<String>,
    #[wd_lsdata(index = "3")]
    is_layout: Option<bool>,
    #[wd_lsdata(index = "4")]
    default_button_id: Option<String>,
    #[wd_lsdata(index = "5")]
    tooltip: Option<String>,
    #[wd_lsdata(index = "6")]
    scroll_top: Option<i32>,
    #[wd_lsdata(index = "7")]
    scroll_left: Option<i32>,
    #[wd_lsdata(index = "8")]
    hotkeys_id: Option<String>,
    #[wd_lsdata(index = "9")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "10")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "11")]
    labelled_by: Option<String>,
}

#[doc = "스크롤을 처리하는 컨테이너"]
#[derive(WdElement)]
#[wd_element(control_id = "SC", element_name = "ScrollContainer")]
#[wd_element(interactable)]
#[wd_element(def = "ScrollContainerDef", def_doc = "[`ScrollContainer`]의 정의")]
#[wd_element(lsdata = "ScrollContainerLSData")]
pub struct ScrollContainer<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<ScrollContainerLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
}

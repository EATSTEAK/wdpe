use std::{borrow::Cow, cell::OnceCell};

use crate::{
    WdElement, WdLsData,
    element::property::{ScrollingMode, Visibility},
};

use self::property::TrayDesign;

pub mod property {
    use serde::Deserialize;
    #[derive(Clone, Deserialize, Debug)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum TrayDesign {
        Transparent,
        Plain,
        Fill,
    }
}

// TODO: Implement additional events and data
#[doc = "[`Tray`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct TrayLSData {
    #[wd_lsdata(index = "0")]
    title: Option<String>,
    #[wd_lsdata(index = "1")]
    design: Option<TrayDesign>,
    #[wd_lsdata(index = "2")]
    collapsed: Option<bool>,
    #[wd_lsdata(index = "3")]
    enabled: Option<bool>,
    #[wd_lsdata(index = "4")]
    tooltip: Option<String>,
    #[wd_lsdata(index = "5")]
    height: Option<String>,
    #[wd_lsdata(index = "6")]
    content_height: Option<String>,
    #[wd_lsdata(index = "7")]
    has_option_menu: Option<bool>,
    #[wd_lsdata(index = "8")]
    option_menu_id: Option<String>,
    #[wd_lsdata(index = "9")]
    has_close_button: Option<bool>,
    #[wd_lsdata(index = "10")]
    scrolling_mode: Option<ScrollingMode>,
    #[wd_lsdata(index = "11")]
    has_toolbar: Option<bool>,
    #[wd_lsdata(index = "12")]
    is_collapsible: Option<bool>,
    #[wd_lsdata(index = "13")]
    accessibility_description: Option<String>,
    #[wd_lsdata(index = "14")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "15")]
    default_button_id: Option<String>,
    #[wd_lsdata(index = "16")]
    scroll_top: Option<i32>,
    #[wd_lsdata(index = "17")]
    scroll_left: Option<i32>,
    #[wd_lsdata(index = "18")]
    access_key: Option<String>,
    #[wd_lsdata(index = "19")]
    hotkeys_id: Option<String>,
    #[wd_lsdata(index = "20")]
    is_drag_handle: Option<bool>,
    #[wd_lsdata(index = "21")]
    client_select: Option<bool>,
    #[wd_lsdata(index = "22")]
    heading_level: Option<i32>,
    #[wd_lsdata(index = "23")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "24")]
    custom_style: Option<String>,
}

#[doc = "열고 닫을 수 있는 트레이"]
#[derive(WdElement)]
#[wd_element(control_id = "TY", element_name = "Tray")]
#[wd_element(interactable)]
#[wd_element(def = "TrayDef", def_doc = "[`Tray`]의 정의")]
#[wd_element(lsdata = "TrayLSData")]
pub struct Tray<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<TrayLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
}

use std::{borrow::Cow, cell::OnceCell};

use crate::{WdElement, WdLsData, element::property::Visibility};

// Note: This element renders as "TSITM_ie6" if >= IE6
#[doc = "[`TabStripItem`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct TabStripItemLSData {
    #[wd_lsdata(index = "0")]
    id: Option<String>,
    #[wd_lsdata(index = "1")]
    index: Option<i32>,
    #[wd_lsdata(index = "2")]
    caption: Option<String>,
    #[wd_lsdata(index = "3")]
    has_title_caption: Option<bool>,
    #[wd_lsdata(index = "4")]
    tooltip: Option<String>,
    #[wd_lsdata(index = "5")]
    enabled: Option<bool>,
    #[wd_lsdata(index = "6")]
    scrolling_mode: Option<String>,
    #[wd_lsdata(index = "7")]
    has_toolbar: Option<bool>,
    #[wd_lsdata(index = "8")]
    default_button_id: Option<String>,
    #[wd_lsdata(index = "9")]
    is_closable: Option<bool>,
    #[wd_lsdata(index = "10")]
    scroll_top: Option<i32>,
    #[wd_lsdata(index = "11")]
    scroll_left: Option<i32>,
    #[wd_lsdata(index = "12")]
    client_tab_select: Option<bool>,
    #[wd_lsdata(index = "13")]
    hotkeys_id: Option<String>,
    #[wd_lsdata(index = "14")]
    access_key: Option<String>,
    #[wd_lsdata(index = "15")]
    has_editable_title: Option<bool>,
    #[wd_lsdata(index = "16")]
    area_design: Option<String>,
    #[wd_lsdata(index = "17")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "18")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "19")]
    visibility: Option<Visibility>,
}

#[doc = "[`TabStrip`](crate::element::layout::TabStrip) 내부 아이템"]
#[derive(WdElement)]
#[wd_element(control_id = "TSITM_standards", element_name = "TabStripTab")]
#[wd_element(def = "TabStripItemDef", def_doc = "[`TabStripItem`]의 정의")]
#[wd_element(lsdata = "TabStripItemLSData")]
pub struct TabStripItem<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<TabStripItemLSData>,
}

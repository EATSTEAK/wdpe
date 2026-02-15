use std::{borrow::Cow, cell::OnceCell};

use crate::{WdElement, WdLsData, element::property::Visibility};

use self::property::ScrollDirection;

pub mod property {
    use serde::Deserialize;

    #[derive(Clone, Deserialize, Debug)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum ScrollDirection {
        Vertical,
        Horizontal,
    }
}

// TODO: Implement additional events and data
#[doc = "[`Scrollbar`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct ScrollbarLSData {
    #[wd_lsdata(index = "0")]
    value: Option<i32>,
    #[wd_lsdata(index = "1")]
    maximum: Option<i32>,
    #[wd_lsdata(index = "2")]
    minimum: Option<i32>,
    #[wd_lsdata(index = "3")]
    large_change: Option<i32>,
    #[wd_lsdata(index = "4")]
    small_change: Option<i32>,
    #[wd_lsdata(index = "5")]
    scroll_direction: Option<ScrollDirection>,
    #[wd_lsdata(index = "6")]
    scrolled_element_id: Option<String>,
    #[wd_lsdata(index = "7")]
    show_scroll_tip: Option<bool>,
    #[wd_lsdata(index = "8")]
    scroll_tip_value_description: Option<String>,
    #[wd_lsdata(index = "9")]
    enabled: Option<bool>,
    #[wd_lsdata(index = "10")]
    item_count: Option<i32>,
    #[wd_lsdata(index = "11")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "12")]
    visibility: Option<Visibility>,
}

#[doc = "스크롤을 수행하는 스크롤 바"]
#[derive(WdElement)]
#[wd_element(control_id = "SCB", element_name = "Scrollbar")]
#[wd_element(interactable)]
#[wd_element(def = "ScrollbarDef", def_doc = "[`Scrollbar`]의 정의")]
#[wd_element(lsdata = "ScrollbarLSData")]
pub struct Scrollbar<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<ScrollbarLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
}

use std::{borrow::Cow, cell::OnceCell};

use crate::{WdElement, WdLsData, element::property::Visibility};

// TODO: Implement additional events and data
#[doc = "[`GridLayout`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct GridLayoutLSData {
    #[wd_lsdata(index = "0")]
    height: Option<String>,
    #[wd_lsdata(index = "1")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "2")]
    drag_source_info: Option<String>,
    #[wd_lsdata(index = "3")]
    drop_target_info: Option<String>,
    #[wd_lsdata(index = "4")]
    drop_decorator_type: Option<String>,
    #[wd_lsdata(index = "5")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "6")]
    custom_data: Option<String>,
}

#[doc = "HTML `grid` 레이아웃"]
#[derive(WdElement)]
#[wd_element(control_id = "GL", element_name = "GridLayout")]
#[wd_element(interactable)]
#[wd_element(def = "GridLayoutDef", def_doc = "[`GridLayout`]의 정의")]
#[wd_element(lsdata = "GridLayoutLSData")]
pub struct GridLayout<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<GridLayoutLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
}

/// [`GridLayout`] 내부 셀
pub mod cell;

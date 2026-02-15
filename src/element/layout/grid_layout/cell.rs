use std::{borrow::Cow, cell::OnceCell};

use crate::{WdElement, WdLsData};

// TODO: Implement additional events and data
#[doc = "[`GridLayoutCell`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct GridLayoutCellLSData {
    #[wd_lsdata(index = "0")]
    drag_data: Option<String>,
    #[wd_lsdata(index = "1")]
    semantic_color: Option<String>,
    #[wd_lsdata(index = "2")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "3")]
    layout_cell_position: Option<String>,
    #[wd_lsdata(index = "4")]
    custom_style: Option<String>,
}

#[doc = "[`GridLayout`](crate::element::layout::GridLayout) 내부 셀"]
#[derive(WdElement)]
#[wd_element(control_id = "GLC", element_name = "GridLayoutCell")]
#[wd_element(interactable)]
#[wd_element(def = "GridLayoutCellDef", def_doc = "[`GridLayoutCell`]의 정의")]
#[wd_element(lsdata = "GridLayoutCellLSData")]
pub struct GridLayoutCell<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<GridLayoutCellLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
}

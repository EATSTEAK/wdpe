use std::{borrow::Cow, cell::OnceCell};

use crate::{
    WdElement, WdLsData,
    element::property::{LockedDesign, Visibility},
};

#[doc = "[`FlowLayout`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct FlowLayoutLSData {
    #[wd_lsdata(index = "0")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "1")]
    custom_data: Option<String>,
}

#[doc = "HTML `flow` 레이아웃"]
#[derive(WdElement)]
#[wd_element(control_id = "FL", element_name = "FlowLayout")]
#[wd_element(def = "FlowLayoutDef", def_doc = "[`FlowLayout`]의 정의")]
#[wd_element(lsdata = "FlowLayoutLSData")]
pub struct FlowLayout<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<FlowLayoutLSData>,
}

#[doc = "[`Container`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct ContainerLSData {
    #[wd_lsdata(index = "0")]
    locked: Option<bool>,
    #[wd_lsdata(index = "1")]
    printable: Option<bool>,
    #[wd_lsdata(index = "2")]
    print_area: Option<bool>,
    #[wd_lsdata(index = "3")]
    locked_design: Option<LockedDesign>,
    #[wd_lsdata(index = "4")]
    locked_message: Option<String>,
    #[wd_lsdata(index = "5")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "6")]
    custom_style: Option<String>,
}

#[doc = "가상 컨테이너"]
#[derive(WdElement)]
#[wd_element(control_id = "CO", element_name = "Container")]
#[wd_element(def = "ContainerDef", def_doc = "[`Container`]의 정의")]
#[wd_element(lsdata = "ContainerLSData")]
pub struct Container<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<ContainerLSData>,
}

/// [`GridLayout`] 엘리먼트 모듈
pub mod grid_layout;
/// [`TabStrip`] 엘리먼트 모듈
pub mod tab_strip;

#[doc(inline)]
pub use self::grid_layout::{GridLayout, GridLayoutDef, GridLayoutLSData};
#[doc(inline)]
pub use self::tab_strip::{TabStrip, TabStripDef, TabStripLSData};

mod button_row;
mod form;
mod popup_window;
mod scroll_container;
mod scrollbar;
mod tray;

pub use self::button_row::{ButtonRow, ButtonRowDef, ButtonRowLSData};
pub use self::form::{Form, FormData, FormDef, FormLSData};
pub use self::popup_window::{PopupWindow, PopupWindowDef, PopupWindowLSData};
pub use self::scroll_container::{ScrollContainer, ScrollContainerDef, ScrollContainerLSData};
pub use self::scrollbar::{Scrollbar, ScrollbarDef, ScrollbarLSData};
pub use self::tray::{Tray, TrayDef, TrayLSData};

use std::{borrow::Cow, cell::OnceCell};

use crate::{
    WdElement, WdLsData,
    element::property::{HorizontalTextAlign, VerticalTextAlign, Visibility},
};

use self::property::ItsDisplayMode;

pub mod property {
    use serde::Deserialize;

    /// 이미지 표시 모드
    #[allow(missing_docs)]
    #[derive(Clone, Deserialize, Debug)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum ItsDisplayMode {
        Normal,
        Stretch,
        Fit,
        NormalCenter,
        FitCenter,
        Fill,
    }
}

// TODO: Implement additional events and data
#[doc = "[`Image`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct ImageLSData {
    #[wd_lsdata(index = "0")]
    tooltip: Option<String>,
    #[wd_lsdata(index = "1")]
    width: Option<String>,
    #[wd_lsdata(index = "2")]
    height: Option<String>,
    #[wd_lsdata(index = "3")]
    src: Option<String>,
    #[wd_lsdata(index = "4")]
    is_interactive: Option<bool>,
    #[wd_lsdata(index = "5")]
    has_image_map: Option<bool>,
    #[wd_lsdata(index = "6")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "7")]
    is_nested: Option<bool>,
    #[wd_lsdata(index = "8")]
    label_text: Option<String>,
    #[wd_lsdata(index = "9")]
    adjust_image_size: Option<bool>,
    #[wd_lsdata(index = "10")]
    drag_source_info: Option<String>,
    #[wd_lsdata(index = "11")]
    is_drag_handle: Option<bool>,
    #[wd_lsdata(index = "12")]
    enabled: Option<bool>,
    #[wd_lsdata(index = "13")]
    error_image_src: Option<String>,
    #[wd_lsdata(index = "14")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "15")]
    its_mode: Option<bool>,
    #[wd_lsdata(index = "16")]
    its_display_mode: Option<ItsDisplayMode>,
    #[wd_lsdata(index = "17")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "18")]
    drop_target_info: Option<String>,
    #[wd_lsdata(index = "19")]
    vertical_text_align: Option<VerticalTextAlign>,
    #[wd_lsdata(index = "20")]
    horizontal_text_align: Option<HorizontalTextAlign>,
    #[wd_lsdata(index = "21")]
    used_in_sap_table: Option<bool>,
    #[wd_lsdata(index = "22")]
    labelled_by: Option<String>,
}

#[doc = "HTML 이미지"]
#[derive(WdElement)]
#[wd_element(control_id = "IMG", element_name = "Image")]
#[wd_element(interactable)]
#[wd_element(def = "ImageDef", def_doc = "[`Image`]의 정의")]
#[wd_element(lsdata = "ImageLSData")]
pub struct Image<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<ImageLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
}

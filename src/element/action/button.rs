use std::{borrow::Cow, cell::OnceCell};

use crate::{
    WdElement, WdLsData,
    element::property::{ContentVisibility, HotkeyValue, SemanticColor, TextDesign, Visibility},
    wd_event,
};

use super::{ButtonDesign, ButtonInteractionBehaviour, ButtonType};

pub mod property {
    use serde::Deserialize;

    /// 버튼의 외형 종류
    #[allow(missing_docs)]
    #[derive(Clone, Deserialize, Debug)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum ButtonDesign {
        Emphasized,
        Standard,
        Previous,
        Next,
        Transparent,
        Accept,
        Reject,
        Toggle,
    }

    /// 버튼의 동작 분류
    #[allow(missing_docs)]
    #[derive(Clone, Deserialize, Debug)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum ButtonType {
        None,
        Menu,
        Help,
        Personalize,
        Close,
        ExpandAll,
        CollapseAll,
        ScrollTop,
        Minimize,
        Maximize,
        Restore,
        CollapseBegin,
        CollapseEnd,
        ExpandBegin,
        ExpandEnd,
        Back,
        Forward,
        VariantManagement,
        Rte,
    }

    /// 버튼의 상호작용 동작
    #[allow(missing_docs)]
    #[derive(Clone, Deserialize, Debug)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum ButtonInteractionBehaviour {
        Push,
        Toggle,
    }
}

#[doc = "[`Button`]의 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct ButtonLSData {
    #[wd_lsdata(index = "0")]
    text: Option<String>,
    #[wd_lsdata(index = "1")]
    text_design: Option<TextDesign>,
    #[wd_lsdata(index = "2")]
    design: Option<ButtonDesign>,
    #[wd_lsdata(index = "3")]
    width: Option<String>,
    #[wd_lsdata(index = "4")]
    tooltip: Option<String>,
    #[wd_lsdata(index = "5")]
    enabled: Option<bool>,
    #[wd_lsdata(index = "6")]
    has_button_caption: Option<bool>,
    #[wd_lsdata(index = "7")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "8")]
    show_help: Option<bool>,
    #[wd_lsdata(index = "9")]
    down: Option<bool>,
    #[wd_lsdata(index = "10")]
    has_icon: Option<bool>,
    #[wd_lsdata(index = "11")]
    disabled_icon_src: Option<String>,
    #[wd_lsdata(index = "12")]
    up_icon_src: Option<String>,
    #[wd_lsdata(index = "13")]
    down_icon_src: Option<String>,
    #[wd_lsdata(index = "14")]
    has_popup_menu: Option<bool>,
    #[wd_lsdata(index = "15")]
    popup_menu_id: Option<String>,
    #[wd_lsdata(index = "16")]
    has_popup_menu_section: Option<bool>,
    #[wd_lsdata(index = "17")]
    image_first: Option<bool>,
    #[wd_lsdata(index = "18")]
    access_key: Option<String>,
    #[wd_lsdata(index = "19")]
    hotkey: Option<HotkeyValue>,
    #[wd_lsdata(index = "20")]
    up: Option<bool>,
    #[wd_lsdata(index = "21")]
    text_overflow: Option<bool>,
    #[wd_lsdata(index = "22")]
    fixed_height: Option<bool>,
    #[wd_lsdata(index = "23")]
    button_type: Option<ButtonType>,
    #[wd_lsdata(index = "24")]
    drag_source_info: Option<String>,
    #[wd_lsdata(index = "25")]
    semantic_color: Option<SemanticColor>,
    #[wd_lsdata(index = "26")]
    interaction_behaviour: Option<ButtonInteractionBehaviour>,
    #[wd_lsdata(index = "27")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "28")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "29")]
    wrapping: Option<bool>,
    #[wd_lsdata(index = "30")]
    height: Option<String>,
    #[wd_lsdata(index = "31")]
    content_visibility: Option<ContentVisibility>,
}

#[doc = "누를 수 있는 버튼"]
#[derive(WdElement)]
#[wd_element(control_id = "B", element_name = "Button")]
#[wd_element(interactable)]
#[wd_element(def = "ButtonDef", def_doc = "[`Button`]의 정의")]
#[wd_element(lsdata = "ButtonLSData")]
pub struct Button<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<ButtonLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
}

impl<'a> Button<'a> {
    /// 버튼 누름 이벤트를 반환합니다.
    #[wd_event(name = "Press")]
    pub fn press(&self) {}
}

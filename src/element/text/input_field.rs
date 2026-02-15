use std::{borrow::Cow, cell::OnceCell};

use crate::{
    WdElement, WdLsData,
    element::property::{
        EmbeddingBehaviour, IMEMode, InputFieldTextStyle, InputFieldType, SemanticColor,
        TabBehaviour, TableFieldDesign, Visibility,
    },
};

// TODO: Implement additional events and data
#[doc = "[`InputField`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct InputFieldLSData {
    #[wd_lsdata(index = "0")]
    value: Option<String>,
    #[wd_lsdata(index = "1")]
    show_help_button: Option<bool>,
    #[wd_lsdata(index = "2")]
    input_field_type: Option<InputFieldType>,
    #[wd_lsdata(index = "3")]
    width: Option<String>,
    #[wd_lsdata(index = "4")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "5")]
    label_text: Option<String>,
    #[wd_lsdata(index = "6")]
    hide_field_help: Option<bool>,
    #[wd_lsdata(index = "7")]
    container_width_set: Option<bool>,
    #[wd_lsdata(index = "8")]
    ime_mode: Option<IMEMode>,
    #[wd_lsdata(index = "9")]
    auto_complete: Option<bool>,
    #[wd_lsdata(index = "10")]
    format_string: Option<String>,
    #[wd_lsdata(index = "11")]
    show_help_button_always: Option<bool>,
    #[wd_lsdata(index = "12")]
    date_picker_start_ref_id: Option<String>,
    #[wd_lsdata(index = "13")]
    access_key: Option<String>,
    #[wd_lsdata(index = "14")]
    display_as_text: Option<bool>,
    #[wd_lsdata(index = "15")]
    text_style: Option<InputFieldTextStyle>,
    #[wd_lsdata(index = "16")]
    spinner_increment: Option<i32>,
    #[wd_lsdata(index = "17")]
    spinner_bounds_check: Option<bool>,
    #[wd_lsdata(index = "18")]
    spinner_max: Option<i32>,
    #[wd_lsdata(index = "19")]
    spinner_min: Option<i32>,
    #[wd_lsdata(index = "20")]
    sap_table_field_design: Option<TableFieldDesign>,
    #[wd_lsdata(index = "21")]
    validation_trigger: Option<String>,
    #[wd_lsdata(index = "22")]
    tab_behaviour: Option<TabBehaviour>,
    #[wd_lsdata(index = "23")]
    semantic_color: Option<SemanticColor>,
    #[wd_lsdata(index = "24")]
    embedding_behaviour: Option<EmbeddingBehaviour>,
    #[wd_lsdata(index = "25")]
    field_help_floating: Option<bool>,
    #[wd_lsdata(index = "26")]
    first_day_of_week: Option<i32>,
    #[wd_lsdata(index = "27")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "28")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "29")]
    field_help_embedding: Option<bool>,
    #[wd_lsdata(index = "30")]
    height: Option<String>,
    #[wd_lsdata(index = "31")]
    labelled_by: Option<String>,
    #[wd_lsdata(index = "32")]
    described_by: Option<String>,
}

#[doc = "입력 필드"]
#[derive(WdElement)]
#[wd_element(control_id = "I", element_name = "InputField")]
#[wd_element(interactable, textisable)]
#[wd_element(def = "InputFieldDef", def_doc = "[`InputField`]의 정의")]
#[wd_element(lsdata = "InputFieldLSData")]
pub struct InputField<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<InputFieldLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
}

impl<'a> InputField<'a> {
    /// 이 [`InputField`]의 값을 가져옵니다.
    pub fn value(&self) -> Option<&str> {
        use crate::element::Element as _;
        self.element_ref().attr("value")
    }
}

impl std::fmt::Display for InputField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value().unwrap_or_default())
    }
}

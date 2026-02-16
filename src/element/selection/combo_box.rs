use std::{borrow::Cow, cell::OnceCell};

use crate::element::property::{
    EmbeddingBehaviour, IMEMode, InputFieldTextStyle, InputFieldType, SemanticColor,
    SuggestFilterCondition, SuggestFilterType, TabBehaviour, TableFieldDesign, Visibility,
};
use crate::error::{BodyError, ElementError, WebDynproError};

use self::property::ComboBoxBehavior;
use crate::element::parser::ElementParser;
use crate::element::{Element, ElementDefWrapper};

use super::list_box::ListBoxDefWrapper;
use crate::{WdElement, WdLsData, wd_event};

pub mod property {
    use serde::Deserialize;

    #[allow(missing_docs)]
    #[derive(Clone, Deserialize, Debug)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum ComboBoxBehavior {
        DropdownSelect,
        FreeText,
        SuggestList,
        SuggestFieldHelp,
    }
}

#[doc = "[`ComboBox`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct ComboBoxLSData {
    #[wd_lsdata(index = "0")]
    width: Option<String>,
    #[wd_lsdata(index = "1")]
    behavior: Option<ComboBoxBehavior>,
    #[wd_lsdata(index = "2")]
    allow_virtual_typing: Option<String>,
    #[wd_lsdata(index = "3")]
    item_list_box_id: Option<String>,
    #[wd_lsdata(index = "4")]
    key: Option<String>,
    #[wd_lsdata(index = "5")]
    value: Option<String>,
    #[wd_lsdata(index = "6")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "7")]
    container_width_set: Option<bool>,
    #[wd_lsdata(index = "8")]
    label_text: Option<String>,
    #[wd_lsdata(index = "9")]
    label_for: Option<String>,
    #[wd_lsdata(index = "10")]
    ime_mode: Option<IMEMode>,
    #[wd_lsdata(index = "11")]
    component_type: Option<InputFieldType>,
    #[wd_lsdata(index = "12")]
    show_help_button_always: Option<String>,
    #[wd_lsdata(index = "13")]
    access_key: Option<String>,
    #[wd_lsdata(index = "14")]
    suggest_filter: Option<SuggestFilterType>,
    #[wd_lsdata(index = "15")]
    display_as_text: Option<bool>,
    #[wd_lsdata(index = "16")]
    hide_field_help: Option<bool>,
    #[wd_lsdata(index = "17")]
    show_help_button: Option<bool>,
    #[wd_lsdata(index = "18")]
    suggest_auto_complete: Option<bool>,
    #[wd_lsdata(index = "19")]
    suggest_filter_condition: Option<SuggestFilterCondition>,
    #[wd_lsdata(index = "20")]
    field_help_floating: Option<bool>,
    #[wd_lsdata(index = "21")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "22")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "23")]
    text_style: Option<InputFieldTextStyle>,
    #[wd_lsdata(index = "24")]
    semantic_color: Option<SemanticColor>,
    #[wd_lsdata(index = "25")]
    embedding_behaviour: Option<EmbeddingBehaviour>,
    #[wd_lsdata(index = "26")]
    sap_table_field_design: Option<TableFieldDesign>,
    #[wd_lsdata(index = "27")]
    field_help_embedding: Option<bool>,
    #[wd_lsdata(index = "28")]
    height: Option<String>,
    #[wd_lsdata(index = "29")]
    labelled_by: Option<String>,
    #[wd_lsdata(index = "30")]
    tab_behaviour: Option<TabBehaviour>,
    #[wd_lsdata(index = "31")]
    described_by: Option<String>,
}

#[doc = "목록 혹은 직접 입력하여 선택할 수 있는 콤보 박스"]
#[derive(WdElement)]
#[wd_element(control_id = "CB", element_name = "ComboBox")]
#[wd_element(interactable, textisable)]
#[wd_element(def = "ComboBoxDef", def_doc = "[`ComboBox`]의 정의")]
#[wd_element(lsdata = "ComboBoxLSData")]
pub struct ComboBox<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<ComboBoxLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
}

impl<'a> ComboBox<'a> {
    /// [`ComboBox`]의 선택지 역할을 하는 [`ListBox`](super::list_box::ListBox) 엘리먼트를 가져옵니다.
    pub fn item_list_box(
        &self,
        parser: &ElementParser,
    ) -> Result<ListBoxDefWrapper, WebDynproError> {
        let listbox_id = self
            .lsdata()
            .item_list_box_id()
            .ok_or(ElementError::NoSuchData {
                element: self.id().to_string(),
                field: "item_list_box_id".to_string(),
            })?;
        let selector = scraper::Selector::parse(format!(r#"[id="{listbox_id}"]"#).as_str())
            .or(Err(ElementError::InvalidId(listbox_id.to_owned())))?;
        let elem = parser
            .document()
            .select(&selector)
            .next()
            .ok_or(BodyError::NoSuchElement(listbox_id.to_owned()))?;
        Ok(
            ListBoxDefWrapper::from_def(ElementDefWrapper::from_ref(elem)?)
                .ok_or(BodyError::NoSuchElement(listbox_id.to_owned()))?,
        )
    }

    /// 선택지를 선택하는 이벤트를 반환합니다. `by_enter`가 참일 경우 엔터를 눌러 선택한 것으로 취급합니다.
    #[wd_event(name = "Select", params(key: &str => "Key", by_enter: bool => "ByEnter"))]
    pub fn select(&self, key: &str, by_enter: bool) {}

    /// 내용을 변경하는 이벤트를 반환합니다.
    #[wd_event(name = "Change", params(value: &str => "Value"))]
    pub fn change(&self, value: &str) {}

    /// 이 [`ComboBox`]의 값을 가져옵니다.
    pub fn value(&self) -> Option<&str> {
        self.element_ref.attr("value")
    }
}

impl std::fmt::Display for ComboBox<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value().unwrap_or_default())
    }
}

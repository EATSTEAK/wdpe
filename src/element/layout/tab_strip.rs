use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::{
    WdElement, WdLsData,
    element::{Element, definition::ElementDefinition, property::Visibility},
    error::BodyError,
    wd_event,
};

use self::item::TabStripItem;

#[doc = "[`TabStrip`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct TabStripLSData {
    #[wd_lsdata(index = "0")]
    current_index: Option<i32>,
    #[wd_lsdata(index = "1")]
    height: Option<String>,
    #[wd_lsdata(index = "2")]
    width: Option<String>,
    #[wd_lsdata(index = "3")]
    accessibility_description: Option<String>,
    #[wd_lsdata(index = "4")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "5")]
    first_visible_item_idx: Option<i32>,
    #[wd_lsdata(index = "6")]
    scrollable: Option<bool>,
    #[wd_lsdata(index = "7")]
    exact_tab_alignment: Option<bool>,
    #[wd_lsdata(index = "8")]
    client_tab_select: Option<bool>,
    #[wd_lsdata(index = "9")]
    drag_source_info: Option<String>,
    #[wd_lsdata(index = "10")]
    drop_target_info: Option<String>,
    #[wd_lsdata(index = "11")]
    tab_items_position: Option<String>,
    #[wd_lsdata(index = "12")]
    custom_data: Option<String>,
    #[wd_lsdata(index = "13")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "14")]
    tab_items_design: Option<String>,
    #[wd_lsdata(index = "15")]
    heading_level: Option<i32>,
}

// Note: This element renders as "TS_ie6" if >= IE6
#[doc = "상단 버튼으로 선택할 수 있는 탭 레이아웃"]
#[doc = ""]
#[doc = "> |**참고**| 이 엘리먼트는 실제 구현에서 >= IE6 용 구현과 기본 구현으로 나누어져 있지만, rusaint에서는 최신의 브라우저를 기준으로 하므로 전자의 구현은 구현되어있지 않습니다."]
#[derive(WdElement)]
#[wd_element(control_id = "TS_standards", element_name = "TabStrip")]
#[wd_element(interactable)]
#[wd_element(def = "TabStripDef", def_doc = "[`TabStrip`]의 정의")]
#[wd_element(lsdata = "TabStripLSData")]
pub struct TabStrip<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<TabStripLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
    tab_items: OnceCell<Vec<<TabStripItem<'a> as Element<'a>>::Def>>,
}

impl<'a> TabStrip<'a> {
    /// 탭 내부 [`TabStripItem`]의 정의를 반환합니다.
    pub fn tab_items(
        &self,
    ) -> impl ExactSizeIterator<Item = &<TabStripItem<'a> as Element<'a>>::Def> {
        self.tab_items
            .get_or_init(|| {
                let Ok(items_selector) =
                    Selector::parse(format!(r#"[ct="{}"]"#, TabStripItem::CONTROL_ID).as_str())
                        .or(Err(BodyError::InvalidSelector))
                else {
                    return vec![];
                };
                self.element_ref
                    .select(&items_selector)
                    .filter_map(|eref| {
                        let id = eref.value().id()?;
                        Some(<TabStripItem<'a> as Element<'a>>::Def::new_dynamic(
                            id.to_owned(),
                        ))
                    })
                    .collect()
            })
            .iter()
    }

    /// 특정 탭을 선택하는 이벤트를 반환합니다.
    #[wd_event(name = "TabSelect", params(
        item_id: &str => "ItemId",
        item_index: u32 => "ItemIndex",
        first_visible_item_index: u32 => "FirstVisibleItemIndex"
    ))]
    pub fn tab_select(&self, item_id: &str, item_index: u32, first_visible_item_index: u32) {}
}

/// [`TabStrip`] 내부 아이템
pub mod item;

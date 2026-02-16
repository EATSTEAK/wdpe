use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::{
    WdLsData, WdSubElement,
    element::{
        ElementDefWrapper,
        complex::sap_table::property::{
            SapTableHeaderCellDesign, SapTableHeaderCellType, SapTableRowSelectionMassState,
            SapTableSelectionColumnAction,
        },
        property::SortState,
    },
    error::BodyError,
};

use super::{SapTableCell, SapTableCellWrapper};

#[doc = "[`SapTableHeaderCell`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct SapTableHeaderCellLSData {
    #[wd_lsdata(index = "0")]
    sort_state: Option<SortState>,
    #[wd_lsdata(index = "1")]
    header_cell_design: Option<SapTableHeaderCellDesign>,
    #[wd_lsdata(index = "2")]
    header_cell_type: Option<SapTableHeaderCellType>,
    #[wd_lsdata(index = "3")]
    selection_column_action: Option<SapTableSelectionColumnAction>,
    #[wd_lsdata(index = "4")]
    selection_menu_id: Option<String>,
    #[wd_lsdata(index = "5")]
    row_selection_mass_state: Option<SapTableRowSelectionMassState>,
    #[wd_lsdata(index = "6")]
    required: Option<bool>,
    #[wd_lsdata(index = "7")]
    tooltip: Option<String>,
    #[wd_lsdata(index = "8")]
    column_selected: Option<bool>,
    #[wd_lsdata(index = "9")]
    column_selectable: Option<bool>,
    #[wd_lsdata(index = "10")]
    filtered: Option<bool>,
    #[wd_lsdata(index = "11")]
    mark_totals: Option<bool>,
    #[wd_lsdata(index = "12")]
    accessibility_description: Option<String>,
    #[wd_lsdata(index = "13")]
    icon_tooltip: Option<String>,
    #[wd_lsdata(index = "14")]
    icon_first: Option<bool>,
    #[wd_lsdata(index = "15")]
    icon_enabled: Option<bool>,
    #[wd_lsdata(index = "16")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "17")]
    custom_data: Option<String>,
}

#[doc = "[`SapTable`](crate::element::complex::SapTable)의 헤더 셀"]
#[derive(WdSubElement, custom_debug_derive::Debug)]
#[wd_element(parent = "SapTable", parent_def = "SapTableDef")]
#[wd_element(subcontrol_id = "HC", element_name = "SapTableHeaderCell")]
#[wd_element(
    def = "SapTableHeaderCellDef",
    def_doc = "[`SapTableHeaderCell`]의 정의"
)]
#[wd_element(lsdata = "SapTableHeaderCellLSData")]
pub struct SapTableHeaderCell<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    #[debug(skip)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<SapTableHeaderCellLSData>,
    content: OnceCell<Option<ElementDefWrapper<'a>>>,
}

impl<'a> SapTableCell<'a> for SapTableHeaderCell<'a> {
    fn content(&self) -> Option<ElementDefWrapper<'a>> {
        self.content
            .get_or_init(|| {
                let content_selector =
                    Selector::parse(format!(r#"[id="{}-CONTENT"] [ct]"#, &self.id).as_str())
                        .or(Err(BodyError::InvalidSelector))
                        .ok()?;
                ElementDefWrapper::from_ref(
                    self.element_ref
                        .select(&content_selector)
                        .next()?
                        .to_owned(),
                )
                .ok()
            })
            .to_owned()
    }
}

impl<'a> SapTableHeaderCell<'a> {
    /// 셀을 [`SapTableCellWrapper`]로 감쌉니다.
    pub fn wrap(self) -> SapTableCellWrapper<'a> {
        SapTableCellWrapper::Header(self)
    }
}

use crate::element::complex::{SapTable, SapTableDef};

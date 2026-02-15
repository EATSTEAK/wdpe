use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::{
    WdLsData, WdSubElement,
    element::{ElementDefWrapper, complex::sap_table::property::SapTableCellType},
};

use super::{SapTableCell, SapTableCellWrapper};

#[doc = "[`SapTableSelectionCell`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct SapTableSelectionCellLSData {
    #[wd_lsdata(index = "0")]
    is_selected: Option<bool>,
    #[wd_lsdata(index = "1")]
    is_secondary_selected: Option<bool>,
    #[wd_lsdata(index = "2")]
    enabled: Option<bool>,
    #[wd_lsdata(index = "3")]
    cell_type: Option<SapTableCellType>,
    #[wd_lsdata(index = "4")]
    row_description: Option<String>,
    #[wd_lsdata(index = "5")]
    is_deselectable: Option<bool>,
    #[wd_lsdata(index = "6")]
    tooltip: Option<String>,
    #[wd_lsdata(index = "7")]
    custom_data: Option<String>,
}

#[doc = "선택 가능한 [`SapTable`](crate::element::complex::SapTable)의 셀"]
#[derive(WdSubElement, custom_debug_derive::Debug)]
#[wd_element(parent = "SapTable", parent_def = "SapTableDef")]
#[wd_element(subcontrol_id = "SC", element_name = "SapTableSelectionCell")]
#[wd_element(
    def = "SapTableSelectionCellDef",
    def_doc = "[`SapTableSelectionCell`]의 정의"
)]
#[wd_element(lsdata = "SapTableSelectionCellLSData")]
pub struct SapTableSelectionCell<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    #[debug(skip)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<SapTableSelectionCellLSData>,
    content: OnceCell<Option<ElementDefWrapper<'a>>>,
}

impl<'a> SapTableCell<'a> for SapTableSelectionCell<'a> {
    fn content(&self) -> Option<ElementDefWrapper<'a>> {
        self.content
            .get_or_init(|| {
                let content_selector = Selector::parse(":root > div > div [ct]").unwrap();
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

impl<'a> SapTableSelectionCell<'a> {
    /// 셀을 [`SapTableCellWrapper`]로 감쌉니다.
    pub fn wrap(self) -> SapTableCellWrapper<'a> {
        SapTableCellWrapper::Selection(self)
    }
}

use crate::element::complex::{SapTable, SapTableDef};

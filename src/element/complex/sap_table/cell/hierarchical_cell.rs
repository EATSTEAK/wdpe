use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::{
    WdLsData, WdSubElement,
    element::{
        ElementDefWrapper,
        complex::sap_table::property::{SapTableCellDesign, SapTableHierarchicalCellStatus},
    },
};

use super::{SapTableCell, SapTableCellWrapper};

#[doc = "[`SapTableHierarchicalCell`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct SapTableHierarchicalCellLSData {
    #[wd_lsdata(index = "0")]
    is_selected: Option<bool>,
    #[wd_lsdata(index = "1")]
    is_secondary_selected: Option<bool>,
    #[wd_lsdata(index = "2")]
    cell_design: Option<SapTableCellDesign>,
    #[wd_lsdata(index = "3")]
    header_cell_ids: Option<String>,
    #[wd_lsdata(index = "4")]
    level: Option<i32>,
    #[wd_lsdata(index = "5")]
    status: Option<SapTableHierarchicalCellStatus>,
    #[wd_lsdata(index = "6")]
    status_enabled: Option<bool>,
    #[wd_lsdata(index = "7")]
    content_type_tooltip: Option<String>,
    #[wd_lsdata(index = "8")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "9")]
    custom_data: Option<String>,
}

#[doc = "계층적 [`SapTable`](crate::element::complex::SapTable)의 셀"]
#[derive(WdSubElement, custom_debug_derive::Debug)]
#[wd_element(parent = "SapTable", parent_def = "SapTableDef")]
#[wd_element(subcontrol_id = "HIC", element_name = "SapTableHierarchicalCell")]
#[wd_element(
    def = "SapTableHierarchicalCellDef",
    def_doc = "[`SapTableHierarchicalCell`]의 정의"
)]
#[wd_element(lsdata = "SapTableHierarchicalCellLSData")]
pub struct SapTableHierarchicalCell<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    #[debug(skip)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<SapTableHierarchicalCellLSData>,
    content: OnceCell<Option<ElementDefWrapper<'a>>>,
}

impl<'a> SapTableCell<'a> for SapTableHierarchicalCell<'a> {
    fn content(&self) -> Option<ElementDefWrapper<'a>> {
        self.content
            .get_or_init(|| {
                let content_selector = Selector::parse(":root [ct]").unwrap();
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

impl<'a> SapTableHierarchicalCell<'a> {
    /// 셀을 [`SapTableCellWrapper`]로 감쌉니다.
    pub fn wrap(self) -> SapTableCellWrapper<'a> {
        SapTableCellWrapper::Hierarchical(self)
    }
}

use crate::element::complex::{SapTable, SapTableDef};

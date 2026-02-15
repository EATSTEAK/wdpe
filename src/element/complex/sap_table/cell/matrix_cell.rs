use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::{
    WdLsData, WdSubElement,
    element::{ElementDefWrapper, complex::sap_table::property::SapTableCellDesign},
};

use super::{SapTableCell, SapTableCellWrapper};

#[doc = "[`SapTableMatrixCell`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct SapTableMatrixCellLSData {
    #[wd_lsdata(index = "0")]
    cell_background_design: Option<SapTableCellDesign>,
    #[wd_lsdata(index = "1")]
    header_cell_ids: Option<String>,
    #[wd_lsdata(index = "2")]
    row_header_cell_ids: Option<String>,
    #[wd_lsdata(index = "3")]
    custom_data: Option<String>,
}

#[doc = "매트릭스 형태의 [`SapTable`](crate::element::complex::SapTable) 셀"]
#[derive(WdSubElement, custom_debug_derive::Debug)]
#[wd_element(parent = "SapTable", parent_def = "SapTableDef")]
#[wd_element(subcontrol_id = "MC", element_name = "SapTableMatrixCell")]
#[wd_element(
    def = "SapTableMatrixCellDef",
    def_doc = "[`SapTableMatrixCell`]의 정의"
)]
#[wd_element(lsdata = "SapTableMatrixCellLSData")]
pub struct SapTableMatrixCell<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    #[debug(skip)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<SapTableMatrixCellLSData>,
    content: OnceCell<Option<ElementDefWrapper<'a>>>,
}

impl<'a> SapTableCell<'a> for SapTableMatrixCell<'a> {
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

impl<'a> SapTableMatrixCell<'a> {
    /// 셀을 [`SapTableCellWrapper`]로 감쌉니다.
    pub fn wrap(self) -> SapTableCellWrapper<'a> {
        SapTableCellWrapper::Matrix(self)
    }
}

use crate::element::complex::{SapTable, SapTableDef};

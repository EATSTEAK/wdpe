use std::{borrow::Cow, cell::OnceCell};

use scraper::Selector;

use crate::{
    WdLsData, WdSubElement,
    element::{
        ElementDefWrapper,
        complex::sap_table::property::{SapTableCellDesign, SapTableCellType},
    },
};

use super::{SapTableCell, SapTableCellWrapper};

#[doc = "[`SapTableNormalCell`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct SapTableNormalCellLSData {
    #[wd_lsdata(index = "0")]
    is_selected: Option<bool>,
    #[wd_lsdata(index = "1")]
    is_secondary_selected: Option<bool>,
    #[wd_lsdata(index = "2")]
    cell_type: Option<SapTableCellType>,
    #[wd_lsdata(index = "3")]
    cell_design: Option<SapTableCellDesign>,
    #[wd_lsdata(index = "4")]
    header_cell_ids: Option<String>,
    #[wd_lsdata(index = "5")]
    row_header_cell_ids: Option<String>,
    #[wd_lsdata(index = "6")]
    custom_style: Option<String>,
    #[wd_lsdata(index = "7")]
    custom_data: Option<String>,
}

#[doc = "일반 [`SapTable`](crate::element::complex::SapTable) 셀"]
#[derive(WdSubElement, custom_debug_derive::Debug)]
#[wd_element(parent = "SapTable", parent_def = "SapTableDef")]
#[wd_element(subcontrol_id = "STC", element_name = "SapTableNormalCell")]
#[wd_element(
    def = "SapTableNormalCellDef",
    def_doc = "[`SapTableNormalCell`]의 정의"
)]
#[wd_element(lsdata = "SapTableNormalCellLSData")]
pub struct SapTableNormalCell<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    #[debug(skip)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<SapTableNormalCellLSData>,
    content: OnceCell<Option<ElementDefWrapper<'a>>>,
}

impl<'a> SapTableCell<'a> for SapTableNormalCell<'a> {
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

impl<'a> SapTableNormalCell<'a> {
    /// 셀을 [`SapTableCellWrapper`]로 감쌉니다.
    pub fn wrap(self) -> SapTableCellWrapper<'a> {
        SapTableCellWrapper::Normal(self)
    }
}

use crate::element::complex::{SapTable, SapTableDef};

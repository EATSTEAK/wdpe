use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use scraper::Selector;

use crate::{
    WdElement, WdLsData,
    element::{Interactable, definition::ElementDefinition},
    error::{BodyError, ElementError, WebDynproError},
    event::Event,
};

use self::property::AccessType;

#[doc = "[`SapTable`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct SapTableLSData {
    #[wd_lsdata(index = "0")]
    title_text: Option<String>,
    #[wd_lsdata(index = "1")]
    accessibility_description: Option<String>,
    #[wd_lsdata(index = "2")]
    row_count: Option<u32>,
    #[wd_lsdata(index = "3")]
    col_count: Option<u32>,
}

#[doc = "테이블"]
#[derive(WdElement)]
#[wd_element(control_id = "ST", element_name = "SapTable")]
#[wd_element(interactable)]
#[wd_element(def = "SapTableDef", def_doc = "[`SapTable`]의 정의")]
#[wd_element(lsdata = "SapTableLSData")]
pub struct SapTable<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<SapTableLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
    table: OnceCell<Option<SapTableBody>>,
}

impl<'a> SapTable<'a> {
    /// 테이블 내부 컨텐츠를 반환합니다.
    pub fn table(&self) -> Result<&SapTableBody, WebDynproError> {
        self.table
            .get_or_init(|| self.parse_table().ok())
            .as_ref()
            .ok_or(
                ElementError::NoSuchContent {
                    element: self.id.to_string(),
                    content: "Table body".to_string(),
                }
                .into(),
            )
    }

    fn parse_table(&self) -> Result<SapTableBody, WebDynproError> {
        let def: SapTableDef = {
            if let Cow::Borrowed(id) = self.id {
                SapTableDef::new(id)
            } else {
                SapTableDef::new_dynamic(self.id.to_string())
            }
        };
        let element = self.element_ref;
        let tbody_selector = Selector::parse(
            format!(
                r#"[id="{}-contentTBody"]"#,
                element.value().id().ok_or(ElementError::NoSuchData {
                    element: self.id.clone().into_owned(),
                    field: "id".to_string()
                })?
            )
            .as_str(),
        )
        .or(Err(BodyError::InvalidSelector))?;
        let Some(tbody) = element.select(&tbody_selector).next() else {
            return Err(ElementError::NoSuchContent {
                element: self.id.clone().into_owned(),
                content: "Table body".to_string(),
            })?;
        };
        Ok(SapTableBody::new(def, tbody)?)
    }

    /// 테이블의 행을 선택하는 이벤트를 반환합니다.
    pub fn row_select(
        &self,
        row_index: i32,
        row_user_data: &str,
        cell_user_data: &str,
        access_type: AccessType,
        trigger_cell_id: &str,
    ) -> Result<Event, WebDynproError> {
        let parameters: HashMap<String, String> = HashMap::from([
            ("Id".to_string(), self.id.clone().to_string()),
            ("RowIndex".to_string(), format!("{row_index}")),
            ("RowUserData".to_string(), row_user_data.to_owned()),
            ("CellUserData".to_string(), cell_user_data.to_owned()),
            ("AccessType".to_string(), access_type.to_string()),
            ("TriggerCellId".to_string(), trigger_cell_id.to_owned()),
        ]);
        self.fire_event("RowSelect".to_string(), parameters)
    }

    /// 테이블의 내부 셀을 선택하는 이벤트를 반환합니다.
    #[allow(clippy::too_many_arguments)]
    pub fn cell_select(
        &self,
        cell_id: &str,
        cell_type: &str,
        row_index: i32,
        col_index: i32,
        row_user_data: &str,
        cell_user_data: &str,
        access_type: AccessType,
    ) -> Result<Event, WebDynproError> {
        let parameters: HashMap<String, String> = HashMap::from([
            ("Id".to_string(), self.id.clone().to_string()),
            ("CellId".to_string(), cell_id.to_owned()),
            ("CellType".to_string(), cell_type.to_owned()),
            ("RowIndex".to_string(), format!("{row_index}")),
            ("ColIndex".to_string(), format!("{col_index}")),
            ("RowUserData".to_string(), row_user_data.to_owned()),
            ("CellUserData".to_string(), cell_user_data.to_owned()),
            ("AccessType".to_string(), access_type.to_string()),
        ]);
        self.fire_event("CellSelect".to_string(), parameters)
    }

    /// 테이블을 상하로 스크롤하는 이벤트를 반환합니다.
    #[allow(clippy::too_many_arguments)]
    pub fn vertical_scroll(
        &self,
        first_visible_item_index: u32,
        cell_id: &str,
        access_type: &str,
        selection_follow_focus: bool,
        shift: bool,
        ctrl: bool,
        alt: bool,
    ) -> Result<Event, WebDynproError> {
        let parameters: HashMap<String, String> = HashMap::from([
            ("Id".to_string(), self.id.clone().to_string()),
            (
                "FirstVisibleItemIndex".to_string(),
                first_visible_item_index.to_string(),
            ),
            ("CellId".to_string(), cell_id.to_owned()),
            ("AccessType".to_string(), access_type.to_string()),
            (
                "SelectionFollowFocus".to_string(),
                selection_follow_focus.to_string(),
            ),
            ("Shift".to_string(), shift.to_string()),
            ("Ctrl".to_string(), ctrl.to_string()),
            ("Alt".to_string(), alt.to_string()),
        ]);
        self.fire_event("VerticalScroll".to_string(), parameters)
    }
}

mod body;
mod from_sap_table;
mod header;
mod row;

/// [`SapTable`] 내부 셀
pub mod cell;
/// [`SapTable`] 내부 데이터 프로퍼티
pub mod property;

pub use self::body::SapTableBody;
pub use self::from_sap_table::FromSapTable;
pub use self::header::SapTableHeader;
pub use self::row::SapTableRow;

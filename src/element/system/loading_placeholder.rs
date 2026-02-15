use std::{borrow::Cow, cell::OnceCell};

use crate::{WdElement, WdLsData};

#[doc = "[`LoadingPlaceholder`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct LoadingPlaceholderLSData {
    #[wd_lsdata(index = "0")]
    id: Option<String>,
    #[wd_lsdata(index = "1")]
    custom_data: Option<String>,
}

#[doc = "페이지가 로드되기 전 내부 컨텐츠가 로드될 위치의 자리 표시자"]
#[doc = ""]
#[doc = "이 엘리먼트는 최초 로드 전 컨텐츠가 로드될 위치를 표시하기 위한 엘리먼트입니다."]
#[doc = "`LoadingPlaceholder.Load` 이벤트가 전송되면 사라지고, 이 엘리먼트가 있는 위치에 실제 페이지가 렌더링됩니다."]
#[doc = ""]
#[doc = "로드 이벤트가 전송되어 페이지가 렌더링되기 위해서는 [`Custom`] 및 [`ClientInspector`] 엘리먼트의 클라이언트 데이터가 전송되어야 합니다."]
#[doc = ""]
#[doc = "[`Custom`]: crate::element::system::Custom"]
#[doc = "[`ClientInspector`]: crate::element::system::ClientInspector"]
#[derive(WdElement)]
#[wd_element(control_id = "LP", element_name = "LoadingPlaceHolder")]
#[wd_element(interactable)]
#[wd_element(
    def = "LoadingPlaceholderDef",
    def_doc = "[`LoadingPlaceholder`]의 정의"
)]
#[wd_element(lsdata = "LoadingPlaceholderLSData")]
pub struct LoadingPlaceholder<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<LoadingPlaceholderLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
}

impl<'a> LoadingPlaceholder<'a> {
    /// 페이지를 로드하기 위한 이벤트를 반환합니다.
    #[crate::wd_event(name = "Load")]
    pub fn load(&self) -> Result<Event, WebDynproError> {}
}

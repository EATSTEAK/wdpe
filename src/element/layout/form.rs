use std::{borrow::Cow, cell::OnceCell};

use crate::{WdElement, WdLsData, wd_event};

#[doc = "[`Form`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct FormLSData {
    #[wd_lsdata(index = "0")]
    has_event_queue: Option<bool>,
    #[wd_lsdata(index = "1")]
    response_data: Option<String>,
    #[wd_lsdata(index = "2")]
    custom_data: Option<String>,
}

/// 서버 전송과 연관된 [`Form`] 데이터
#[derive(Debug, Default)]
#[allow(unused)]
pub struct FormData {
    name: Option<String>,
    method: Option<String>,
    action: Option<String>,
    title: Option<String>,
    accept: Option<String>,
    accept_charset: Option<String>,
    enctype: Option<String>,
    target: Option<String>,
}

#[doc = "서버에 전송하기 위한 HTML Form"]
#[derive(WdElement)]
#[wd_element(control_id = "FOR", element_name = "Form")]
#[wd_element(interactable)]
#[wd_element(def = "FormDef", def_doc = "[`Form`]의 정의")]
#[wd_element(lsdata = "FormLSData")]
pub struct Form<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<FormLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<crate::element::EventParameterMap>>,
    #[allow(dead_code)]
    data: OnceCell<FormData>,
}

impl<'a> Form<'a> {
    /// 폼 `submit`을 요청하는 이벤트를 반환합니다.
    #[wd_event(name = "Request", params(
        is_async: bool => "Async",
        focus_info: &str => "FocusInfo",
        hash: &str => "Hash",
        dom_changed: bool => "DomChanged",
        is_dirty: bool => "IsDirty"
    ))]
    pub fn request(
        &self,
        is_async: bool,
        focus_info: &str,
        hash: &str,
        dom_changed: bool,
        is_dirty: bool,
    ) {
    }
}

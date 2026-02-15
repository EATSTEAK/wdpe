use std::{borrow::Cow, cell::OnceCell};

use crate::{WdElement, WdLsData, element::Element};

#[doc = "[`ListBoxActionItem`]의 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct ListBoxActionItemLSData {
    #[wd_lsdata(index = "0")]
    custom_data: Option<String>,
}

#[doc = "실행할 수 있는 액션이 포함된 [`ListBox`](crate::element::selection::list_box::ListBox)의 아이템"]
#[derive(WdElement, custom_debug_derive::Debug)]
#[wd_element(control_id = "LIB_AI", element_name = "ListBoxActionItem")]
#[wd_element(def = "ListBoxActionItemDef", def_doc = "[`ListBoxActionItem`]의 정의")]
#[wd_element(lsdata = "ListBoxActionItemLSData")]
pub struct ListBoxActionItem<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    #[debug(skip)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<ListBoxActionItemLSData>,
    title: OnceCell<String>,
    text: OnceCell<String>,
}

impl<'a> ListBoxActionItem<'a> {
    /// 제목을 반환합니다.
    pub fn title(&self) -> &str {
        self.title.get_or_init(|| {
            self.element_ref
                .value()
                .attr("title")
                .unwrap_or("")
                .to_owned()
        })
    }

    /// 내부 텍스트를 반환합니다.
    pub fn text(&self) -> &str {
        self.text
            .get_or_init(|| self.element_ref().text().collect::<String>())
    }
}

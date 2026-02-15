use scraper::Selector;
use std::{borrow::Cow, cell::OnceCell};

use crate::{
    WdElement, WdLsData,
    element::{Element, action::Button, definition::ElementDefinition, property::Visibility},
};

#[doc = "[`ButtonRow`] 내부 데이터"]
#[derive(WdLsData)]
#[allow(unused)]
pub struct ButtonRowLSData {
    #[wd_lsdata(index = "0")]
    visibility: Option<Visibility>,
    #[wd_lsdata(index = "1")]
    custom_data: Option<String>,
}

#[doc = "[`Button`]의 나열"]
#[derive(WdElement)]
#[wd_element(control_id = "BR", element_name = "ButtonRow")]
#[wd_element(def = "ButtonRowDef", def_doc = "[`ButtonRow`]의 정의")]
#[wd_element(lsdata = "ButtonRowLSData")]
pub struct ButtonRow<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<ButtonRowLSData>,
    buttons: OnceCell<Vec<<Button<'a> as Element<'a>>::Def>>,
}

impl<'a> ButtonRow<'a> {
    /// 내부 [`Button`]을 반환합니다.
    pub fn buttons(
        &'a self,
    ) -> impl ExactSizeIterator<Item = &'a <Button<'a> as Element<'a>>::Def> {
        self.buttons
            .get_or_init(|| {
                let button_selector = &Selector::parse(r#":root [ct="B"]"#).unwrap();
                self.element_ref
                    .select(button_selector)
                    .filter_map(|elem| <Button<'a> as Element<'a>>::Def::from_ref(elem).ok())
                    .collect::<Vec<<Button<'a> as Element<'a>>::Def>>()
            })
            .iter()
    }
}

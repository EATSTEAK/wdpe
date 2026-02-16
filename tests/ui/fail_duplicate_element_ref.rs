use std::{borrow::Cow, cell::OnceCell};
use wdpe::{WdElement, WdLsData};

#[derive(WdLsData)]
#[allow(unused)]
pub struct DupLSData {
    #[wd_lsdata(index = "0")]
    text: Option<String>,
}

#[derive(WdElement)]
#[wd_element(control_id = "DUP", element_name = "DupElement")]
#[wd_element(def = "DupElementDef")]
#[wd_element(lsdata = "DupLSData")]
#[wd_element(skip_registration)]
pub struct DupElement<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(element_ref)]
    element_ref2: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<DupLSData>,
}

fn main() {}

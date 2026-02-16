use std::{borrow::Cow, cell::OnceCell};
use wdpe::{WdElement, WdLsData};

#[derive(WdLsData)]
#[allow(unused)]
pub struct BadLSData {
    #[wd_lsdata(index = "0")]
    text: Option<String>,
}

#[derive(WdElement)]
#[wd_element(control_id = "BAD", element_name = "BadElement")]
#[wd_element(def = "BadElementDef")]
#[wd_element(lsdata = "BadLSData")]
#[wd_element(skip_registration)]
pub struct BadElement<'a> {
    id: Cow<'static, str>,
    // Missing #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<BadLSData>,
}

fn main() {}

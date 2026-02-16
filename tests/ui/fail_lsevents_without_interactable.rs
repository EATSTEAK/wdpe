use std::{borrow::Cow, cell::OnceCell};
use wdpe::{WdElement, WdLsData};

#[derive(WdLsData)]
#[allow(unused)]
pub struct EvLSData {
    #[wd_lsdata(index = "0")]
    text: Option<String>,
}

#[derive(WdElement)]
#[wd_element(control_id = "EV", element_name = "EvElement")]
// Note: no `interactable` attribute, but lsevents_field is present
#[wd_element(def = "EvElementDef")]
#[wd_element(lsdata = "EvLSData")]
#[wd_element(skip_registration)]
pub struct EvElement<'a> {
    id: Cow<'static, str>,
    #[wd_element(element_ref)]
    element_ref: scraper::ElementRef<'a>,
    #[wd_element(lsdata_field)]
    lsdata: OnceCell<EvLSData>,
    #[wd_element(lsevents_field)]
    lsevents: OnceCell<Option<wdpe::element::EventParameterMap>>,
}

fn main() {}

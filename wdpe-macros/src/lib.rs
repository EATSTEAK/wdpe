extern crate proc_macro;

mod element;
mod event;
mod lsdata;
mod subelement;

use proc_macro::TokenStream;

/// Attribute macro that transforms an LsData struct.
///
/// Wraps each field in `Option<T>`, converts `#[wd_lsdata(index = "N")]`
/// to `#[serde(rename = "N")]`, injects standard derives, and generates
/// accessor methods.
///
/// # Example
///
/// ```ignore
/// #[WdLsData]
/// pub struct ButtonLsData {
///     #[wd_lsdata(index = "0")]
///     text: String,
///     #[wd_lsdata(index = "2")]
///     design: ButtonDesign,
/// }
/// ```
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn WdLsData(attr: TokenStream, item: TokenStream) -> TokenStream {
    lsdata::wd_lsdata_impl(attr.into(), item.into()).into()
}

/// Derive macro for WebDynpro elements.
///
/// Generates the definition struct, `Element` trait impl, optional
/// `Interactable` trait impl, constructor, and `inventory::submit!`
/// registration.
///
/// # Example
///
/// ```ignore
/// #[derive(WdElement)]
/// #[wd_element(control_id = "B", element_name = "Button")]
/// #[wd_element(interactable)]
/// #[wd_element(def = "ButtonDef", def_doc = "Button definition")]
/// #[wd_element(lsdata = "ButtonLsData")]
/// pub struct Button<'a> {
///     id: Cow<'static, str>,
///     #[wd_element(element_ref)]
///     element_ref: scraper::ElementRef<'a>,
///     #[wd_element(lsdata_field)]
///     lsdata: OnceCell<ButtonLsData>,
///     #[wd_element(lsevents_field)]
///     lsevents: OnceCell<Option<EventParameterMap>>,
/// }
/// ```
#[proc_macro_derive(WdElement, attributes(wd_element))]
pub fn derive_wd_element(input: TokenStream) -> TokenStream {
    element::derive_wd_element_impl(input.into()).into()
}

/// Derive macro for WebDynpro sub-elements.
///
/// Generates the sub-element definition struct, `SubElement` trait impl,
/// and constructor.
///
/// # Example
///
/// ```ignore
/// #[derive(WdSubElement)]
/// #[wd_element(parent = "SapTable", parent_def = "SapTableDef")]
/// #[wd_element(subcontrol_id = "HC", element_name = "SapTableHeaderCell")]
/// #[wd_element(def = "SapTableHeaderCellDef")]
/// #[wd_element(lsdata = "SapTableHeaderCellLsData")]
/// pub struct SapTableHeaderCell<'a> {
///     id: Cow<'static, str>,
///     #[wd_element(element_ref)]
///     element_ref: scraper::ElementRef<'a>,
///     #[wd_element(lsdata_field)]
///     lsdata: OnceCell<SapTableHeaderCellLsData>,
/// }
/// ```
#[proc_macro_derive(WdSubElement, attributes(wd_element))]
pub fn derive_wd_subelement(input: TokenStream) -> TokenStream {
    subelement::derive_wd_subelement_impl(input.into()).into()
}

/// Attribute macro for generating event-firing methods.
///
/// Replaces the function body and return type. Automatically inserts
/// the `"Id"` parameter from `self.id` and any declared params.
///
/// # Example
///
/// ```ignore
/// impl<'a> Button<'a> {
///     #[wd_event(name = "Press")]
///     pub fn press(&self) {}
///
///     #[wd_event(name = "Activate", params(ctrl: bool => "Ctrl", shift: bool => "Shift"))]
///     pub fn activate(&self, ctrl: bool, shift: bool) {}
/// }
/// ```
#[proc_macro_attribute]
pub fn wd_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    event::wd_event_impl(attr.into(), item.into()).into()
}

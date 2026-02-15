extern crate proc_macro;

mod element;
mod event;
mod lsdata;
mod subelement;
pub(crate) mod utils;

use proc_macro::TokenStream;

/// Derive macro that generates deserialization, trait impls, and accessor
/// methods for WebDynpro LsData structs.
///
/// The user writes a struct with `Option<T>` fields and
/// `#[wd_lsdata(index = "N")]` attributes. The derive macro generates:
///
/// 1. A `serde::Deserialize` impl via a private shadow struct (with
///    `#[serde(rename = "N")]` derived from `wd_lsdata(index)`)
/// 2. `Clone`, `Debug`, and `Default` trait implementations
/// 3. Accessor methods (`fn field(&self) -> Option<&T>`) for every field
///
/// # Example
///
/// ```ignore
/// #[derive(WdLsData)]
/// #[allow(unused)]
/// pub struct ButtonLsData {
///     #[wd_lsdata(index = "0")]
///     text: Option<String>,
///     #[wd_lsdata(index = "2")]
///     design: Option<ButtonDesign>,
/// }
/// ```
#[proc_macro_derive(WdLsData, attributes(wd_lsdata))]
pub fn derive_wd_lsdata(input: TokenStream) -> TokenStream {
    lsdata::derive_wd_lsdata_impl(input.into()).into()
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

/// Attribute macro that generates event-firing method bodies for interactable elements.
///
/// **Note:** This macro must only be applied to methods on types that implement
/// the `Interactable` trait, as the generated code calls `self.fire_event()`.
/// Applying it to non-interactable elements will result in a compile error.
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

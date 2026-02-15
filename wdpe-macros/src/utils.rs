//! Shared utilities for `wdpe-macros` proc-macro crate.

/// Check if a field has a specific `#[wd_element(flag)]` attribute.
///
/// Used by both [`crate::element`] and [`crate::subelement`] to detect
/// marker flags like `element_ref`, `lsdata_field`, and `lsevents_field`.
pub fn has_wd_element_flag_on_field(attrs: &[syn::Attribute], flag: &str) -> bool {
    for attr in attrs {
        if !attr.path().is_ident("wd_element") {
            continue;
        }
        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(flag) {
                found = true;
            }
            Ok(())
        });
        if found {
            return true;
        }
    }
    false
}

/// Check if any `#[serde(...)]` attribute contains `rename` or `rename_all`
/// using proper structural `Meta` parsing instead of string matching.
pub fn has_serde_rename_attr(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("serde") {
            return false;
        }
        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") || meta.path.is_ident("rename_all") {
                found = true;
            }
            // Skip the rest of the content — could be `= "..."` or `(...)`.
            if meta.input.peek(syn::token::Paren) {
                let _content;
                syn::parenthesized!(_content in meta.input);
            } else if meta.input.peek(syn::Token![=]) {
                let _: syn::Token![=] = meta.input.parse()?;
                let _: syn::Lit = meta.input.parse()?;
            }
            Ok(())
        });
        found
    })
}

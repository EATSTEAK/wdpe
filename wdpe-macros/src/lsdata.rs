use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, Error, Fields, LitStr, Meta, Result, parse2};

/// Implementation for the `#[WdLsData]` attribute macro.
///
/// Transforms a struct so that:
/// - Each field `name: Type` becomes `name: Option<Type>`
/// - `#[wd_lsdata(index = "N")]` becomes `#[serde(rename = "N")]`
///   (or `#[serde(rename(deserialize = "N"))]` if the field has other serde rename attrs)
/// - Outer derives `Clone, serde::Deserialize, Debug, Default` are injected
/// - `#[allow(unused)]` is injected on the struct
/// - Accessor `fn name(&self) -> Option<&Type>` is generated for each field
/// - User-supplied `#[serde(...)]` and `#[doc = "..."]` attributes are preserved
/// - `#[wd_lsdata(...)]` attributes are consumed (removed from output)
pub fn wd_lsdata_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match wd_lsdata_inner(item) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
}

fn wd_lsdata_inner(item: TokenStream) -> Result<TokenStream> {
    let input: DeriveInput = parse2(item)?;
    let name = &input.ident;
    let vis = &input.vis;

    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            Fields::Named(named) => &named.named,
            _ => {
                return Err(Error::new_spanned(
                    &input.ident,
                    "WdLsData can only be applied to structs with named fields",
                ));
            }
        },
        _ => {
            return Err(Error::new_spanned(
                &input.ident,
                "WdLsData can only be applied to structs",
            ));
        }
    };

    // Collect struct-level serde attributes (pass-through)
    let struct_serde_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("serde"))
        .collect();

    // Check if struct has serde rename_all or rename
    let has_struct_serde_rename = struct_serde_attrs.iter().any(|a| {
        let tokens = a.meta.to_token_stream().to_string();
        tokens.contains("rename_all") || tokens.contains("rename")
    });

    // Collect struct-level doc attrs
    let struct_doc_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("doc"))
        .collect();

    // Collect user-written non-serde, non-doc, non-wd_lsdata, non-derive attrs
    let struct_other_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| {
            !a.path().is_ident("serde")
                && !a.path().is_ident("doc")
                && !a.path().is_ident("wd_lsdata")
                && !a.path().is_ident("derive")
                && !a.path().is_ident("WdLsData")
        })
        .collect();

    // Collect any user-written extra derives (beyond WdLsData itself)
    let user_derives = extract_user_derives(&input.attrs);

    let mut field_defs = Vec::new();
    let mut accessors = Vec::new();

    for field in fields {
        let field_name = field
            .ident
            .as_ref()
            .ok_or_else(|| Error::new_spanned(field, "WdLsData fields must be named"))?;
        let field_type = &field.ty;
        let field_vis = &field.vis;

        // Extract #[wd_lsdata(index = "N")]
        let index_value = extract_wd_lsdata_index(&field.attrs)?.ok_or_else(|| {
            Error::new_spanned(
                field,
                "Each WdLsData field must have #[wd_lsdata(index = \"...\")]",
            )
        })?;

        // Collect field-level serde attrs (pass-through)
        let field_serde_attrs: Vec<_> = field
            .attrs
            .iter()
            .filter(|a| a.path().is_ident("serde"))
            .collect();

        // Check if field has serde rename attrs
        let has_field_serde_rename = field_serde_attrs.iter().any(|a| {
            let tokens = a.meta.to_token_stream().to_string();
            tokens.contains("rename")
        });

        // Decide rename strategy
        let rename_attr = if has_field_serde_rename || has_struct_serde_rename {
            // Use deserialize-only rename to avoid conflicts
            quote! { #[serde(rename(deserialize = #index_value))] }
        } else {
            quote! { #[serde(rename = #index_value)] }
        };

        // Collect doc attrs and other non-wd_lsdata, non-serde attrs on the field
        let field_other_attrs: Vec<_> = field
            .attrs
            .iter()
            .filter(|a| !a.path().is_ident("wd_lsdata") && !a.path().is_ident("serde"))
            .collect();

        field_defs.push(quote! {
            #(#field_other_attrs)*
            #(#field_serde_attrs)*
            #rename_attr
            #field_vis #field_name: Option<#field_type>,
        });

        accessors.push(quote! {
            pub fn #field_name(&self) -> Option<&#field_type> {
                self.#field_name.as_ref()
            }
        });
    }

    let user_derive_tokens = if user_derives.is_empty() {
        quote! {}
    } else {
        quote! { #[derive(#(#user_derives),*)] }
    };

    Ok(quote! {
        #(#struct_doc_attrs)*
        #(#struct_other_attrs)*
        #(#struct_serde_attrs)*
        #[derive(Clone, serde::Deserialize, Debug, Default)]
        #user_derive_tokens
        #[allow(unused)]
        #vis struct #name {
            #(#field_defs)*
        }

        #[allow(missing_docs)]
        impl #name {
            #(#accessors)*
        }
    })
}

/// Extract the `index = "N"` value from `#[wd_lsdata(index = "N")]` attributes.
fn extract_wd_lsdata_index(attrs: &[syn::Attribute]) -> Result<Option<String>> {
    for attr in attrs {
        if !attr.path().is_ident("wd_lsdata") {
            continue;
        }
        let mut index_val = None;
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("index") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                index_val = Some(lit.value());
            }
            Ok(())
        })?;
        if let Some(val) = index_val {
            return Ok(Some(val));
        }
    }
    Ok(None)
}

/// Extract user-written derives from `#[derive(...)]` attributes,
/// filtering out `WdLsData` itself.
fn extract_user_derives(attrs: &[syn::Attribute]) -> Vec<syn::Path> {
    let mut derives = Vec::new();
    for attr in attrs {
        if !attr.path().is_ident("derive") {
            continue;
        }
        if let Meta::List(list) = &attr.meta {
            let parser = syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated;
            if let Ok(paths) = list.parse_args_with(parser) {
                for path in paths {
                    // Filter out WdLsData itself
                    if !path.is_ident("WdLsData") {
                        derives.push(path);
                    }
                }
            }
        }
    }
    derives
}

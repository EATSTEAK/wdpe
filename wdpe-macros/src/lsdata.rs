use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Error, Fields, LitStr, Result, parse2};

use crate::utils::has_serde_rename_attr;

/// Implementation for the `#[derive(WdLsData)]` derive macro.
///
/// The user writes a struct with `Option<T>` fields and `#[wd_lsdata(index = "N")]`
/// attributes. The derive macro generates:
///
/// 1. A **private shadow struct** inside `const _: () = { ... }` with `#[serde(rename = "N")]`
///    (or `#[serde(rename(deserialize = "N"))]` when user-written serde rename attrs exist),
///    plus a `serde::Deserialize` impl for the original struct that delegates to the shadow.
/// 2. `impl Clone`, `impl Debug`, `impl Default` for the struct.
/// 3. Accessor methods `fn field_name(&self) -> Option<&Type>` for each field.
///
/// User-supplied `#[serde(...)]` attributes on both the struct and fields are passed through
/// to the shadow struct. The `#[wd_lsdata(...)]` attributes are consumed (not forwarded).
pub fn derive_wd_lsdata_impl(input: TokenStream) -> TokenStream {
    match derive_wd_lsdata_inner(input) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
}

fn derive_wd_lsdata_inner(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveInput = parse2(input)?;
    let name = &input.ident;
    let _vis = &input.vis;

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

    // Collect struct-level serde attributes (pass-through to shadow struct)
    let struct_serde_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("serde"))
        .collect();

    // Check if struct has serde rename_all or rename (using structural parsing)
    let has_struct_serde_rename = has_serde_rename_attr(
        &input
            .attrs
            .iter()
            .filter(|a| a.path().is_ident("serde"))
            .cloned()
            .collect::<Vec<_>>(),
    );

    let helper_name = format_ident!("__{}DeserializeHelper", name);

    let mut helper_field_defs = Vec::new();
    let mut conversion_fields = Vec::new();
    let mut accessors = Vec::new();
    let mut field_names_for_clone = Vec::new();
    let mut field_names_for_debug = Vec::new();
    let mut field_names_for_default = Vec::new();

    for field in fields {
        let field_name = field
            .ident
            .as_ref()
            .ok_or_else(|| Error::new_spanned(field, "WdLsData fields must be named"))?;
        let field_type = &field.ty;
        let _field_vis = &field.vis;

        // Extract #[wd_lsdata(index = "N")]
        let index_value = extract_wd_lsdata_index(&field.attrs)?.ok_or_else(|| {
            Error::new_spanned(
                field,
                "Each WdLsData field must have #[wd_lsdata(index = \"...\")]",
            )
        })?;

        // Collect field-level serde attrs (pass-through to shadow)
        let field_serde_attrs: Vec<_> = field
            .attrs
            .iter()
            .filter(|a| a.path().is_ident("serde"))
            .collect();

        // Check if field has serde rename attrs (using structural parsing)
        let has_field_serde_rename = has_serde_rename_attr(
            &field
                .attrs
                .iter()
                .filter(|a| a.path().is_ident("serde"))
                .cloned()
                .collect::<Vec<_>>(),
        );

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

        // Shadow struct fields (for deserialization)
        helper_field_defs.push(quote! {
            #(#field_serde_attrs)*
            #rename_attr
            #field_name: #field_type,
        });

        // Conversion from helper to user struct
        conversion_fields.push(quote! {
            #field_name: helper.#field_name,
        });

        // Accessor methods
        // Check if the field type is Option<T> — extract T for the accessor return type
        let inner_type = extract_option_inner_type(field_type);
        let accessor = if let Some(inner_ty) = inner_type {
            quote! {
                #(#field_other_attrs)*
                pub fn #field_name(&self) -> Option<&#inner_ty> {
                    self.#field_name.as_ref()
                }
            }
        } else {
            // If not Option<T>, return a reference directly
            quote! {
                #(#field_other_attrs)*
                pub fn #field_name(&self) -> &#field_type {
                    &self.#field_name
                }
            }
        };
        accessors.push(accessor);

        field_names_for_clone.push(quote! { #field_name: self.#field_name.clone() });
        field_names_for_debug.push(quote! {
            .field(stringify!(#field_name), &self.#field_name)
        });
        field_names_for_default.push(quote! { #field_name: Default::default() });
    }

    let name_str = name.to_string();

    Ok(quote! {
        // Generate Deserialize impl via a private shadow struct
        const _: () = {
            #[derive(serde::Deserialize)]
            #(#struct_serde_attrs)*
            struct #helper_name {
                #(#helper_field_defs)*
            }

            impl<'de> serde::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let helper = #helper_name::deserialize(deserializer)?;
                    Ok(#name {
                        #(#conversion_fields)*
                    })
                }
            }
        };

        impl Clone for #name {
            fn clone(&self) -> Self {
                Self {
                    #(#field_names_for_clone,)*
                }
            }
        }

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(#name_str)
                    #(#field_names_for_debug)*
                    .finish()
            }
        }

        impl Default for #name {
            fn default() -> Self {
                Self {
                    #(#field_names_for_default,)*
                }
            }
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

/// Extract the inner type `T` from `Option<T>`. Returns `None` if the type
/// is not a simple `Option<T>` path.
fn extract_option_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(type_path) = ty {
        let segment = type_path.path.segments.last()?;
        if segment.ident == "Option"
            && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
            && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
        {
            return Some(inner);
        }
    }
    None
}

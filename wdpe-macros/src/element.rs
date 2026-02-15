use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Error, Fields, LitStr, Result};

use crate::utils::has_wd_element_flag_on_field;

/// Parsed struct-level `#[wd_element(...)]` attributes.
struct ElementAttrs {
    control_id: String,
    element_name: String,
    interactable: bool,
    def: String,
    def_doc: Option<String>,
    lsdata: String,
    textisable: bool,
    skip_registration: bool,
    wrapper_variant: Option<String>,
}

/// Parsed field info.
struct FieldInfo {
    ident: Ident,
    is_element_ref: bool,
    is_lsdata: bool,
    is_lsevents: bool,
    is_id: bool,
    is_custom: bool,
}

pub fn derive_wd_element_impl(input: TokenStream) -> TokenStream {
    match derive_wd_element_inner(input) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
}

fn derive_wd_element_inner(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveInput = syn::parse2(input)?;
    let struct_name = &input.ident;

    // Parse struct-level attributes
    let attrs = parse_element_attrs(&input)?;

    // Extract lifetime parameter
    let lifetime = input
        .generics
        .lifetimes()
        .next()
        .map(|lt| lt.lifetime.clone());

    // Parse fields
    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            Fields::Named(named) => &named.named,
            _ => {
                return Err(Error::new_spanned(
                    struct_name,
                    "WdElement can only be derived on structs with named fields",
                ));
            }
        },
        _ => {
            return Err(Error::new_spanned(
                struct_name,
                "WdElement can only be derived on structs",
            ));
        }
    };

    let mut field_infos: Vec<FieldInfo> = Vec::new();
    for field in fields {
        let ident = field.ident.as_ref().unwrap().clone();
        let is_element_ref = has_wd_element_flag_on_field(&field.attrs, "element_ref");
        let is_lsdata = has_wd_element_flag_on_field(&field.attrs, "lsdata_field");
        let is_lsevents = has_wd_element_flag_on_field(&field.attrs, "lsevents_field");
        let is_id = ident == "id" && !is_element_ref && !is_lsdata && !is_lsevents;
        let is_custom = !is_element_ref && !is_lsdata && !is_lsevents && !is_id;

        field_infos.push(FieldInfo {
            ident,
            is_element_ref,
            is_lsdata,
            is_lsevents,
            is_id,
            is_custom,
        });
    }

    // Validate
    let has_element_ref = field_infos.iter().any(|f| f.is_element_ref);
    let has_lsdata = field_infos.iter().any(|f| f.is_lsdata);

    if !has_element_ref {
        return Err(Error::new_spanned(
            struct_name,
            "WdElement requires exactly one field with #[wd_element(element_ref)]",
        ));
    }
    if !has_lsdata {
        return Err(Error::new_spanned(
            struct_name,
            "WdElement requires exactly one field with #[wd_element(lsdata_field)]",
        ));
    }

    if attrs.interactable && !field_infos.iter().any(|f| f.is_lsevents) {
        return Err(Error::new_spanned(
            struct_name,
            "#[wd_element(interactable)] requires a field with #[wd_element(lsevents_field)]",
        ));
    }

    let element_ref_field = field_infos
        .iter()
        .find(|f| f.is_element_ref)
        .unwrap()
        .ident
        .clone();
    let lsdata_field = field_infos
        .iter()
        .find(|f| f.is_lsdata)
        .unwrap()
        .ident
        .clone();

    let control_id = &attrs.control_id;
    let element_name_str = &attrs.element_name;
    let def_name = Ident::new(&attrs.def, Span::call_site());
    let lsdata_type = Ident::new(&attrs.lsdata, Span::call_site());

    // Determine the ElementWrapper variant name
    let wrapper_variant_ident = if let Some(ref variant) = attrs.wrapper_variant {
        Ident::new(variant, Span::call_site())
    } else {
        struct_name.clone()
    };

    // Generate definition struct
    let def_doc_attr = if let Some(ref doc) = attrs.def_doc {
        quote! { #[doc = #doc] }
    } else {
        quote! {}
    };

    let def_struct = quote! {
        #def_doc_attr
        #[derive(Clone, Debug)]
        pub struct #def_name {
            id: std::borrow::Cow<'static, str>,
        }

        impl #def_name {
            /// Creates an element definition. Use [`define_elements`](crate::define_elements) macro instead of calling this directly.
            pub const fn new(id: &'static str) -> Self {
                Self {
                    id: std::borrow::Cow::Borrowed(id),
                }
            }
        }

        impl<'body> crate::element::definition::ElementDefinition<'body> for #def_name {
            type Element = #struct_name<'body>;

            fn new_dynamic(id: String) -> Self {
                Self { id: id.into() }
            }

            fn from_ref(
                element_ref: scraper::ElementRef<'_>,
            ) -> Result<Self, crate::error::WebDynproError> {
                let id = element_ref
                    .value()
                    .id()
                    .ok_or(crate::error::BodyError::InvalidElement)?;
                Ok(Self {
                    id: id.to_string().into(),
                })
            }

            fn id(&self) -> &str {
                &self.id
            }

            fn id_cow(&self) -> std::borrow::Cow<'static, str> {
                self.id.clone()
            }
        }
    };

    // Generate Element trait impl
    let element_impl = if let Some(ref lt) = lifetime {
        quote! {
            impl<#lt> crate::element::Element<#lt> for #struct_name<#lt> {
                const CONTROL_ID: &'static str = #control_id;
                const ELEMENT_NAME: &'static str = #element_name_str;
                type ElementLSData = #lsdata_type;
                type Def = #def_name;

                fn lsdata(&self) -> &Self::ElementLSData {
                    self.#lsdata_field.get_or_init(|| {
                        let lsdata_attr = self.#element_ref_field.value().attr("lsdata").unwrap_or("");
                        let Ok(lsdata_obj) = crate::element::utils::parse_lsdata(lsdata_attr)
                            .or_else(|e| {
                                tracing::warn!(?e, "failed to parse lsdata");
                                Err(e)
                            })
                        else {
                            return #lsdata_type::default();
                        };
                        serde_json::from_value::<Self::ElementLSData>(lsdata_obj)
                            .or_else(|e| {
                                tracing::warn!(?e, "failed to convert lsdata to struct");
                                Err(e)
                            })
                            .ok()
                            .unwrap_or(#lsdata_type::default())
                    })
                }

                fn from_ref(
                    element_def: &impl crate::element::definition::ElementDefinition<#lt>,
                    element: scraper::ElementRef<#lt>,
                ) -> Result<Self, crate::error::WebDynproError> {
                    Ok(Self::new(
                        crate::element::definition::ElementDefinition::id_cow(element_def),
                        element,
                    ))
                }

                fn id(&self) -> &str {
                    &self.id
                }

                fn element_ref(&self) -> &scraper::ElementRef<#lt> {
                    &self.#element_ref_field
                }

                fn wrap(self) -> crate::element::ElementWrapper<#lt> {
                    crate::element::ElementWrapper::#wrapper_variant_ident(self)
                }

                fn children(&self) -> Vec<crate::element::ElementWrapper<#lt>> {
                    crate::element::utils::children_element(self.element_ref().clone())
                }
            }
        }
    } else {
        // No lifetime — unlikely but handle gracefully
        return Err(Error::new_spanned(
            struct_name,
            "WdElement structs must have a lifetime parameter",
        ));
    };

    // Generate Interactable impl (conditional)
    let interactable_impl = if attrs.interactable {
        let lsevents_field = field_infos
            .iter()
            .find(|f| f.is_lsevents)
            .unwrap()
            .ident
            .clone();
        let lt = lifetime.as_ref().unwrap();
        quote! {
            impl<#lt> crate::element::Interactable<#lt> for #struct_name<#lt> {
                fn lsevents(&self) -> Option<&crate::element::EventParameterMap> {
                    self.#lsevents_field
                        .get_or_init(|| {
                            let lsevents_attr = self.#element_ref_field.value().attr("lsevents").unwrap_or("");
                            crate::element::utils::parse_lsevents(lsevents_attr)
                                .or_else(|e| {
                                    tracing::warn!(?e, "failed to parse lsevents");
                                    Err(e)
                                })
                                .ok()
                        })
                        .as_ref()
                }
            }
        }
    } else {
        quote! {}
    };

    // Generate constructor
    let lt = lifetime.as_ref().unwrap();
    let mut constructor_field_inits = Vec::new();
    for fi in &field_infos {
        let ident = &fi.ident;
        if fi.is_id {
            constructor_field_inits.push(quote! { #ident, });
        } else if fi.is_element_ref {
            constructor_field_inits.push(quote! { #ident: element_ref, });
        } else if fi.is_lsdata || fi.is_lsevents {
            constructor_field_inits.push(quote! { #ident: std::cell::OnceCell::new(), });
        } else {
            // custom field
            constructor_field_inits.push(quote! { #ident: Default::default(), });
        }
    }

    let constructor = quote! {
        impl<#lt> #struct_name<#lt> {
            /// Creates a new element from an id and element reference.
            pub fn new(
                id: std::borrow::Cow<'static, str>,
                element_ref: scraper::ElementRef<#lt>,
            ) -> Self {
                Self {
                    #(#constructor_field_inits)*
                }
            }
        }
    };

    // Generate inventory::submit! (conditional)
    let inventory_submit = if !attrs.skip_registration {
        quote! {
            inventory::submit! {
                crate::element::registry::ElementRegistration::new(
                    #control_id,
                    #element_name_str,
                    |id, element_ref| {
                        use crate::element::Element as _;
                        use crate::element::definition::ElementDefinition as _;
                        let def = #def_name::new_dynamic(id);
                        Ok(#struct_name::from_ref(&def, element_ref)?.wrap())
                    },
                )
            }
        }
    } else {
        quote! {}
    };

    // Generate textisable registration (conditional)
    let textisable_submit = if attrs.textisable {
        quote! {
            inventory::submit! {
                crate::element::registry::TextisableRegistration {
                    to_string_fn: |wrapper| {
                        match wrapper {
                            crate::element::ElementWrapper::#wrapper_variant_ident(el) => Some(el.to_string()),
                            _ => None,
                        }
                    },
                }
            }
        }
    } else {
        quote! {}
    };

    // Generate Default assertion for custom fields — emit the helper function
    // once, outside the per-field loop, then one call per custom field.
    let custom_field_type_assertions: Vec<_> = field_infos
        .iter()
        .filter(|f| f.is_custom)
        .map(|f| {
            let field_type = fields
                .iter()
                .find(|fld| fld.ident.as_ref().unwrap() == &f.ident)
                .map(|fld| &fld.ty)
                .unwrap();
            quote! {
                _assert_default::<#field_type>();
            }
        })
        .collect();

    let default_assertions = if !custom_field_type_assertions.is_empty() {
        let lt = lifetime.as_ref().unwrap();
        quote! {
            const _: () = {
                fn _assert_default<T: Default>() {}
                fn _check<#lt>() {
                    #(#custom_field_type_assertions)*
                }
            };
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        #def_struct
        #element_impl
        #interactable_impl
        #constructor
        #inventory_submit
        #textisable_submit
        #default_assertions
    })
}

/// Parse all struct-level `#[wd_element(...)]` attributes.
fn parse_element_attrs(input: &DeriveInput) -> Result<ElementAttrs> {
    let mut control_id: Option<String> = None;
    let mut element_name: Option<String> = None;
    let mut interactable = false;
    let mut def: Option<String> = None;
    let mut def_doc: Option<String> = None;
    let mut lsdata: Option<String> = None;
    let mut textisable = false;
    let mut skip_registration = false;
    let mut wrapper_variant: Option<String> = None;

    for attr in &input.attrs {
        if !attr.path().is_ident("wd_element") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("control_id") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                control_id = Some(lit.value());
            } else if meta.path.is_ident("element_name") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                element_name = Some(lit.value());
            } else if meta.path.is_ident("interactable") {
                interactable = true;
            } else if meta.path.is_ident("def") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                def = Some(lit.value());
            } else if meta.path.is_ident("def_doc") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                def_doc = Some(lit.value());
            } else if meta.path.is_ident("lsdata") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                lsdata = Some(lit.value());
            } else if meta.path.is_ident("textisable") {
                textisable = true;
            } else if meta.path.is_ident("skip_registration") {
                skip_registration = true;
            } else if meta.path.is_ident("wrapper_variant") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                wrapper_variant = Some(lit.value());
            }
            Ok(())
        })?;
    }

    let control_id = control_id.ok_or_else(|| {
        Error::new_spanned(
            &input.ident,
            "WdElement requires #[wd_element(control_id = \"...\")]",
        )
    })?;
    let element_name = element_name.ok_or_else(|| {
        Error::new_spanned(
            &input.ident,
            "WdElement requires #[wd_element(element_name = \"...\")]",
        )
    })?;
    let def = def.ok_or_else(|| {
        Error::new_spanned(
            &input.ident,
            "WdElement requires #[wd_element(def = \"...\")]",
        )
    })?;
    let lsdata = lsdata.ok_or_else(|| {
        Error::new_spanned(
            &input.ident,
            "WdElement requires #[wd_element(lsdata = \"...\")]",
        )
    })?;

    Ok(ElementAttrs {
        control_id,
        element_name,
        interactable,
        def,
        def_doc,
        lsdata,
        textisable,
        skip_registration,
        wrapper_variant,
    })
}

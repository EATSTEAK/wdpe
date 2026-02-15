use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Error, Fields, LitStr, Result};

use crate::utils::has_wd_element_flag_on_field;

/// Parsed struct-level `#[wd_element(...)]` attributes for sub-elements.
struct SubElementAttrs {
    parent: String,
    parent_def: String,
    subcontrol_id: String,
    element_name: String,
    def: String,
    def_doc: Option<String>,
    lsdata: String,
}

pub fn derive_wd_subelement_impl(input: TokenStream) -> TokenStream {
    match derive_wd_subelement_inner(input) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
}

fn derive_wd_subelement_inner(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveInput = syn::parse2(input)?;
    let struct_name = &input.ident;

    // Parse struct-level attributes
    let attrs = parse_subelement_attrs(&input)?;

    // Extract lifetime parameter
    let lifetime = input
        .generics
        .lifetimes()
        .next()
        .map(|lt| lt.lifetime.clone())
        .ok_or_else(|| {
            Error::new_spanned(
                struct_name,
                "WdSubElement structs must have a lifetime parameter",
            )
        })?;

    // Parse fields
    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            Fields::Named(named) => &named.named,
            _ => {
                return Err(Error::new_spanned(
                    struct_name,
                    "WdSubElement can only be derived on structs with named fields",
                ));
            }
        },
        _ => {
            return Err(Error::new_spanned(
                struct_name,
                "WdSubElement can only be derived on structs",
            ));
        }
    };

    // Identify field roles
    let mut element_ref_field: Option<Ident> = None;
    let mut lsdata_field_ident: Option<Ident> = None;
    let mut custom_fields: Vec<(Ident, &syn::Type)> = Vec::new();

    for field in fields {
        let ident = field.ident.as_ref().unwrap().clone();
        let is_element_ref = has_wd_element_flag_on_field(&field.attrs, "element_ref");
        let is_lsdata = has_wd_element_flag_on_field(&field.attrs, "lsdata_field");
        let is_id = ident == "id" && !is_element_ref && !is_lsdata;

        if is_element_ref {
            element_ref_field = Some(ident);
        } else if is_lsdata {
            lsdata_field_ident = Some(ident);
        } else if !is_id {
            custom_fields.push((ident, &field.ty));
        }
    }

    let element_ref_field = element_ref_field.ok_or_else(|| {
        Error::new_spanned(
            struct_name,
            "WdSubElement requires exactly one field with #[wd_element(element_ref)]",
        )
    })?;
    let lsdata_field_ident = lsdata_field_ident.ok_or_else(|| {
        Error::new_spanned(
            struct_name,
            "WdSubElement requires exactly one field with #[wd_element(lsdata_field)]",
        )
    })?;

    let parent_ident = Ident::new(&attrs.parent, Span::call_site());
    let parent_def_ident = Ident::new(&attrs.parent_def, Span::call_site());
    let subcontrol_id = &attrs.subcontrol_id;
    let element_name_str = &attrs.element_name;
    let def_name = Ident::new(&attrs.def, Span::call_site());
    let lsdata_type = Ident::new(&attrs.lsdata, Span::call_site());
    let lt = &lifetime;

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
            parent: #parent_def_ident,
        }

        impl #def_name {
            /// Creates a sub-element definition.
            pub const fn new(parent: #parent_def_ident, id: &'static str) -> Self {
                Self {
                    id: std::borrow::Cow::Borrowed(id),
                    parent,
                }
            }
        }

        impl<'body> crate::element::sub::definition::SubElementDefinition<'body> for #def_name {
            type Parent = #parent_ident<'body>;
            type SubElement = #struct_name<'body>;

            fn new_dynamic(
                parent: <Self::Parent as crate::element::Element<'body>>::Def,
                id: String,
            ) -> Self {
                Self {
                    id: id.into(),
                    parent,
                }
            }

            fn from_ref(
                parent: <Self::Parent as crate::element::Element<'body>>::Def,
                element_ref: scraper::ElementRef<'_>,
            ) -> Result<Self, crate::error::WebDynproError> {
                let id = element_ref
                    .value()
                    .id()
                    .ok_or(crate::error::BodyError::InvalidElement)?;
                Ok(Self {
                    id: id.to_string().into(),
                    parent,
                })
            }

            fn id(&self) -> &str {
                &self.id
            }

            fn id_cow(&self) -> std::borrow::Cow<'static, str> {
                self.id.clone()
            }

            fn parent(&self) -> &<Self::Parent as crate::element::Element<'body>>::Def {
                &self.parent
            }
        }
    };

    // Generate SubElement trait impl
    let subelement_impl = quote! {
        impl<#lt> crate::element::sub::SubElement<#lt> for #struct_name<#lt> {
            const SUBCONTROL_ID: &'static str = #subcontrol_id;
            const ELEMENT_NAME: &'static str = #element_name_str;
            type SubElementLSData = #lsdata_type;
            type Def = #def_name;

            fn lsdata(&self) -> &Self::SubElementLSData {
                self.#lsdata_field_ident.get_or_init(|| {
                    let lsdata_attr = self.#element_ref_field.value().attr("lsdata").unwrap_or("");
                    let Ok(lsdata_obj) = crate::element::utils::parse_lsdata(lsdata_attr) else {
                        return #lsdata_type::default();
                    };
                    serde_json::from_value::<Self::SubElementLSData>(lsdata_obj)
                        .ok()
                        .unwrap_or(#lsdata_type::default())
                })
            }

            fn from_ref(
                element_def: &impl crate::element::sub::definition::SubElementDefinition<#lt>,
                element: scraper::ElementRef<#lt>,
            ) -> Result<Self, crate::error::WebDynproError> {
                Ok(Self::new(
                    crate::element::sub::definition::SubElementDefinition::id_cow(element_def),
                    element,
                ))
            }

            fn id(&self) -> &str {
                &self.id
            }

            fn element_ref(&self) -> &scraper::ElementRef<#lt> {
                &self.#element_ref_field
            }
        }
    };

    // Generate constructor
    let mut constructor_field_inits = Vec::new();
    for field in fields {
        let ident = field.ident.as_ref().unwrap();
        let is_element_ref = has_wd_element_flag_on_field(&field.attrs, "element_ref");
        let is_lsdata = has_wd_element_flag_on_field(&field.attrs, "lsdata_field");
        let is_id = *ident == "id" && !is_element_ref && !is_lsdata;

        if is_id {
            constructor_field_inits.push(quote! { #ident, });
        } else if is_element_ref {
            constructor_field_inits.push(quote! { #ident: element_ref, });
        } else if is_lsdata {
            constructor_field_inits.push(quote! { #ident: std::cell::OnceCell::new(), });
        } else {
            // custom field
            constructor_field_inits.push(quote! { #ident: Default::default(), });
        }
    }

    let constructor = quote! {
        impl<#lt> #struct_name<#lt> {
            /// Creates a new sub-element from an id and element reference.
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

    Ok(quote! {
        #def_struct
        #subelement_impl
        #constructor
    })
}

/// Parse all struct-level `#[wd_element(...)]` attributes for sub-elements.
fn parse_subelement_attrs(input: &DeriveInput) -> Result<SubElementAttrs> {
    let mut parent: Option<String> = None;
    let mut parent_def: Option<String> = None;
    let mut subcontrol_id: Option<String> = None;
    let mut element_name: Option<String> = None;
    let mut def: Option<String> = None;
    let mut def_doc: Option<String> = None;
    let mut lsdata: Option<String> = None;

    for attr in &input.attrs {
        if !attr.path().is_ident("wd_element") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("parent") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                parent = Some(lit.value());
            } else if meta.path.is_ident("parent_def") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                parent_def = Some(lit.value());
            } else if meta.path.is_ident("subcontrol_id") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                subcontrol_id = Some(lit.value());
            } else if meta.path.is_ident("element_name") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                element_name = Some(lit.value());
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
            }
            Ok(())
        })?;
    }

    let parent = parent.ok_or_else(|| {
        Error::new_spanned(
            &input.ident,
            "WdSubElement requires #[wd_element(parent = \"...\")]",
        )
    })?;
    let parent_def = parent_def.ok_or_else(|| {
        Error::new_spanned(
            &input.ident,
            "WdSubElement requires #[wd_element(parent_def = \"...\")]",
        )
    })?;
    let subcontrol_id = subcontrol_id.ok_or_else(|| {
        Error::new_spanned(
            &input.ident,
            "WdSubElement requires #[wd_element(subcontrol_id = \"...\")]",
        )
    })?;
    let element_name = element_name.ok_or_else(|| {
        Error::new_spanned(
            &input.ident,
            "WdSubElement requires #[wd_element(element_name = \"...\")]",
        )
    })?;
    let def = def.ok_or_else(|| {
        Error::new_spanned(
            &input.ident,
            "WdSubElement requires #[wd_element(def = \"...\")]",
        )
    })?;
    let lsdata = lsdata.ok_or_else(|| {
        Error::new_spanned(
            &input.ident,
            "WdSubElement requires #[wd_element(lsdata = \"...\")]",
        )
    })?;

    Ok(SubElementAttrs {
        parent,
        parent_def,
        subcontrol_id,
        element_name,
        def,
        def_doc,
        lsdata,
    })
}

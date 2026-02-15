use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Ident, LitStr, Result, Token, Type, parse::Parse, parse::ParseStream};

/// Parsed arguments for `#[wd_event(name = "...", params(...))]`.
struct EventArgs {
    name: String,
    params: Vec<EventParam>,
}

struct EventParam {
    ident: Ident,
    ty: Type,
    key: String,
}

impl Parse for EventArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut name: Option<String> = None;
        let mut params = Vec::new();

        while !input.is_empty() {
            let ident: Ident = input.parse()?;

            if ident == "name" {
                let _eq: Token![=] = input.parse()?;
                let lit: LitStr = input.parse()?;
                name = Some(lit.value());
            } else if ident == "params" {
                let content;
                syn::parenthesized!(content in input);
                while !content.is_empty() {
                    let param_ident: Ident = content.parse()?;
                    let _colon: Token![:] = content.parse()?;
                    let param_ty: Type = content.parse()?;
                    let _arrow: Token![=>] = content.parse()?;
                    let key_lit: LitStr = content.parse()?;
                    params.push(EventParam {
                        ident: param_ident,
                        ty: param_ty,
                        key: key_lit.value(),
                    });
                    if content.peek(Token![,]) {
                        let _comma: Token![,] = content.parse()?;
                    }
                }
            } else {
                return Err(Error::new_spanned(ident, "unexpected attribute"));
            }

            if input.peek(Token![,]) {
                let _comma: Token![,] = input.parse()?;
            }
        }

        let name = name.ok_or_else(|| input.error("wd_event requires `name = \"...\"`"))?;
        Ok(EventArgs { name, params })
    }
}

pub fn wd_event_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    match wd_event_inner(attr, item) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
}

fn wd_event_inner(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let args: EventArgs = syn::parse2(attr)?;
    let func: syn::ImplItemFn = syn::parse2(item)?;

    let event_name = &args.name;
    let fn_name = &func.sig.ident;
    let vis = &func.vis;

    // Preserve doc attributes and other non-wd_event attributes
    let preserved_attrs: Vec<_> = func
        .attrs
        .iter()
        .filter(|a| !a.path().is_ident("wd_event"))
        .collect();

    // Build parameter list for the function signature (skip &self)
    let fn_params: Vec<_> = args
        .params
        .iter()
        .map(|p| {
            let name = &p.ident;
            let ty = &p.ty;
            quote! { #name: #ty }
        })
        .collect();

    // Build HashMap insertions for event parameters
    let param_inserts: Vec<_> = args
        .params
        .iter()
        .map(|p| {
            let key = &p.key;
            let name = &p.ident;
            quote! {
                parameters.insert(#key.to_string(), #name.to_string());
            }
        })
        .collect();

    Ok(quote! {
        #(#preserved_attrs)*
        #vis fn #fn_name(
            &self,
            #(#fn_params,)*
        ) -> Result<crate::event::Event, crate::error::WebDynproError> {
            let mut parameters: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();
            parameters.insert("Id".to_string(), self.id.clone().to_string());
            #(#param_inserts)*
            self.fire_event(#event_name.to_string(), parameters)
        }
    })
}

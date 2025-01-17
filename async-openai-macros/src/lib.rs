use darling::{ast::NestedMeta, FromMeta};
use itertools::{Either, Itertools};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse2, parse_macro_input, Expr, FnArg, ItemFn, Meta, MetaList};

#[proc_macro_attribute]
pub fn extensible(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    extensible_impl(item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn extensible_impl(mut item: ItemFn) -> syn::Result<TokenStream> {
    // Stream variants use a special result type
    let is_stream = item.sig.ident.to_string().ends_with("_stream");

    // Prepare a generic method with a different name
    let mut extension = item.clone();
    extension.sig.ident = format_ident!("{}_ext", extension.sig.ident);

    // Remove our attributes from original method arguments
    for input in &mut item.sig.inputs {
        match input {
            FnArg::Receiver(_) => (),
            FnArg::Typed(arg) => arg.attrs.retain(|attr| match &attr.meta {
                Meta::List(meta) => !attr_is_ours(meta),
                _ => true,
            }),
        }
    }

    // Gather request parameters that must be replaced by generics and their optional bounds
    let mut i = 0;
    let generics = extension
        .sig
        .inputs
        .iter_mut()
        .filter_map(|input| match input {
            FnArg::Receiver(_) => None,
            FnArg::Typed(arg) => {
                let (mine, other): (Vec<_>, Vec<_>) =
                    arg.attrs
                        .clone()
                        .into_iter()
                        .partition_map(|attr| match &attr.meta {
                            Meta::List(meta) if attr_is_ours(meta) => Either::Left(
                                Request::from_list(
                                    &NestedMeta::parse_meta_list(meta.tokens.clone()).unwrap(),
                                )
                                .unwrap(),
                            ),
                            _ => Either::Right(attr),
                        });
                let bounds = mine.into_iter().next();
                arg.attrs = other;
                bounds.map(|b| {
                    let ident = format_ident!("__EXTENSIBLE_REQUEST_{i}");
                    arg.ty = Box::new(parse2(quote! { #ident }).unwrap());
                    i += 1;
                    (ident, b)
                })
            }
        })
        .collect::<Vec<_>>();

    // Add generics and their optional bounds to our method's generics
    for (ident, Request { bounds }) in generics {
        let bounds = bounds.map(|b| quote! { + #b });
        extension
            .sig
            .generics
            .params
            .push(parse2(quote! { #ident : ::serde::Serialize #bounds })?)
    }

    // Make the result type generic too
    let result = if is_stream {
        quote! { std::pin::Pin<Box<dyn futures::Stream<Item = Result<__EXTENSIBLE_RESPONSE, OpenAIError>> + Send>>}
    } else {
        quote! { __EXTENSIBLE_RESPONSE }
    };
    extension.sig.output = parse2(quote! { -> Result<#result, OpenAIError> })?;
    let send_and_static = is_stream.then_some(quote! { + Send + 'static });
    extension.sig.generics.params.push(parse2(
        quote! { __EXTENSIBLE_RESPONSE: serde::de::DeserializeOwned #send_and_static },
    )?);

    Ok(quote! {
        #item

        #extension
    })
}

#[derive(FromMeta)]
struct Request {
    bounds: Option<Expr>,
}

fn attr_is_ours(meta: &MetaList) -> bool {
    meta.path.get_ident().map(|ident| ident.to_string()) == Some("request".to_string())
}

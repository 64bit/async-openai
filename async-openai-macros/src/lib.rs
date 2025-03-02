use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, FnArg, GenericParam, Generics, ItemFn, Pat, PatType, ReturnType, Type,
};

#[proc_macro_attribute]
pub fn byot_passthrough(_args: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn byot(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let mut new_generics = Generics::default();
    let mut param_count = 0;

    // Process function arguments
    let mut new_params = Vec::new();
    let args = input.sig.inputs.iter().map(|arg| {
        match arg {
            FnArg::Receiver(receiver) => receiver.to_token_stream(),
            FnArg::Typed(PatType { pat, ty, .. }) => {
                if let Pat::Ident(pat_ident) = &**pat {
                    let generic_ident = syn::Ident::new(
                        &format!("T{}", param_count),
                        proc_macro2::Span::call_site(),
                    );
                    new_params.push(GenericParam::Type(
                        syn::TypeParam::from(generic_ident.clone())
                    ));
                    param_count += 1;
                    quote! { #pat_ident: #generic_ident }
                } else {
                    arg.to_token_stream()
                }
            }
        }
    }).collect::<Vec<_>>();

    // Add R type parameter for return type
    let generic_r = syn::Ident::new("R", proc_macro2::Span::call_site());
    let return_param = GenericParam::Type(syn::TypeParam::from(generic_r.clone()));
    new_params.push(return_param);

    // Add all generic parameters
    new_generics.params.extend(new_params);

    // Generate the new function with Result<R, OpenAIError> return type
    let fn_name = &input.sig.ident;
    let byot_fn_name = syn::Ident::new(&format!("{}_byot", fn_name), fn_name.span());
    let vis = &input.vis;
    let block = &input.block;
    let attrs = &input.attrs;
    let asyncness = &input.sig.asyncness;

    let expanded = quote! {
        // Original function
        #(#attrs)*
        #input

        // Generated generic function with _byot suffix
        #(#attrs)*
        #vis #asyncness fn #byot_fn_name #new_generics (#(#args),*) -> Result<R, OpenAIError> #block
    };

    expanded.into()
}

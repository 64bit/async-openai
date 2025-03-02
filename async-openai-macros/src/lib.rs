use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, FnArg, GenericParam, Generics, ItemFn, Pat, PatType, ReturnType, Type};

#[proc_macro_attribute]
pub fn byot(args: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input function
    let input = parse_macro_input!(item as ItemFn);
    
    // Create new generic parameters for the function
    let mut new_generics = Generics::default();
    let mut generic_mappings = Vec::new();
    let mut param_count = 0;

    // Process function arguments
    let mut new_params = Vec::new();
    let args = input.sig.inputs.iter().map(|arg| {
        match arg {
            // Preserve self arguments as-is
            FnArg::Receiver(receiver) => {
                receiver.to_token_stream()
            },
            FnArg::Typed(PatType { pat, ty, .. }) => {
                if let Pat::Ident(pat_ident) = &**pat {
                    // Create a new generic parameter for this argument
                    let generic_ident = syn::Ident::new(
                        &format!("T{}", param_count),
                        proc_macro2::Span::call_site()
                    );
                    new_params.push(GenericParam::Type(
                        syn::TypeParam::from(generic_ident.clone())
                    ));
                    generic_mappings.push((ty.clone(), generic_ident.clone()));
                    param_count += 1;

                    // Return the argument with the new generic type
                    quote! { #pat_ident: #generic_ident }
                } else {
                    arg.to_token_stream()
                }
            }
        }
    }).collect::<Vec<_>>();

    // Handle return type
    let (return_generic, return_param) = if let ReturnType::Type(arrow, _) = &input.sig.output {
        let generic_ident = syn::Ident::new("R", proc_macro2::Span::call_site());
        let param = GenericParam::Type(syn::TypeParam::from(generic_ident.clone()));
        (quote! { #arrow #generic_ident }, Some(param))
    } else {
        (input.sig.output.to_token_stream(), None)
    };

    // Add all generic parameters at once
    new_generics.params.extend(new_params);
    if let Some(param) = return_param {
        new_generics.params.push(param);
    }

    // Generate the new function
    let fn_name = &input.sig.ident;
    let vis = &input.vis;
    let block = &input.block;
    let attrs = &input.attrs;
    
    let expanded = quote! {
        #(#attrs)*
        #vis fn #fn_name #new_generics (#(#args),*) #return_generic #block
    };

    expanded.into()
}
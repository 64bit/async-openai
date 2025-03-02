use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    FnArg, GenericParam, Generics, ItemFn, Pat, PatType, ReturnType, Type, TypeParam, TypeParamBound,
};

// Parse attribute arguments like #[byot(T0: Display + Debug, T1: Clone, R: Serialize)]
struct BoundArgs {
    bounds: Vec<(String, syn::TypeParamBound)>,
}

impl Parse for BoundArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut bounds = Vec::new();
        let vars = Punctuated::<syn::MetaNameValue, Comma>::parse_terminated(input)?;
        
        for var in vars {
            let name = var.path.get_ident().unwrap().to_string();
            let bound: TypeParamBound = syn::parse_str(&var.value.into_token_stream().to_string())?;
            bounds.push((name, bound));
        }
        Ok(BoundArgs { bounds })
    }
}

#[proc_macro_attribute]
pub fn byot_passthrough(_args: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn byot(args: TokenStream, item: TokenStream) -> TokenStream {
    let bounds_args = parse_macro_input!(args as BoundArgs);
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
                    let generic_name = format!("T{}", param_count);
                    let generic_ident = syn::Ident::new(&generic_name, proc_macro2::Span::call_site());
                    
                    // Create type parameter with optional bounds
                    let mut type_param = TypeParam::from(generic_ident.clone());
                    if let Some((_, bound)) = bounds_args.bounds.iter().find(|(name, _)| name == &generic_name) {
                        type_param.bounds.extend(vec![bound.clone()]);
                    }
                    
                    new_params.push(GenericParam::Type(type_param));
                    param_count += 1;
                    quote! { #pat_ident: #generic_ident }
                } else {
                    arg.to_token_stream()
                }
            }
        }
    }).collect::<Vec<_>>();

    // Add R type parameter with optional bounds
    let generic_r = syn::Ident::new("R", proc_macro2::Span::call_site());
    let mut return_type_param = TypeParam::from(generic_r.clone());
    if let Some((_, bound)) = bounds_args.bounds.iter().find(|(name, _)| name == "R") {
        return_type_param.bounds.extend(vec![bound.clone()]);
    }
    new_params.push(GenericParam::Type(return_type_param));

    // Add all generic parameters
    new_generics.params.extend(new_params);

    let fn_name = &input.sig.ident;
    let byot_fn_name = syn::Ident::new(&format!("{}_byot", fn_name), fn_name.span());
    let vis = &input.vis;
    let block = &input.block;
    let attrs = &input.attrs;
    let asyncness = &input.sig.asyncness;

    let expanded = quote! {
        #(#attrs)*
        #input

        #(#attrs)*
        #vis #asyncness fn #byot_fn_name #new_generics (#(#args),*) -> Result<R, OpenAIError> #block
    };

    expanded.into()
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam, Generics};
use syn::parse_quote;

#[proc_macro_derive(Vanth)]
pub fn vanth_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident.clone();

    let mut generics = input.generics.clone();

    let type_params: Vec<syn::Ident> = generics.params.iter().filter_map(|param| {
        if let GenericParam::Type(type_param) = param {
            Some(type_param.ident.clone())
        } else {
            None
        }
    }).collect();

    let mut where_clause = generics.where_clause.clone().unwrap_or_else(|| parse_quote!(where ));
    for tp in &type_params {
        where_clause.predicates.push(parse_quote!(#tp : vanth::Vanth));
    }

    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let generics_code = if type_params.is_empty() {
        quote! { String::new() }
    } else {
        quote! {
            format!("<{}>", [#(#type_params::ty().path.join("::")),*].join(","))
        }
    };

    let expanded = quote! {
        impl #impl_generics vanth::Vanth for #name #ty_generics #where_clause {
            fn ty() -> vanth::Ty {
                let module_path = module_path!();
                let mut path: Vec<String> = module_path.split("::").map(|s| s.to_string()).collect();
                let base_name = stringify!(#name);
                let generics_str = #generics_code;
                path.push(format!("{}{}", base_name, generics_str));
                vanth::Ty { path }
            }
        }
    };

    TokenStream::from(expanded)
}

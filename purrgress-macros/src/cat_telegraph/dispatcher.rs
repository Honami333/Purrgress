use quote;
use proc_macro;
use syn;


pub fn derive_purr_key_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let (impl_genetics, ty_generics, where_clauses) = &input.generics.split_for_impl();

    let expanded = quote::quote! {
        impl #impl_genetics purrgress::cat_telegraph::dispatcher_types::PurrKey for #name #ty_generics #where_clauses {}
    };

    proc_macro::TokenStream::from(expanded)
}
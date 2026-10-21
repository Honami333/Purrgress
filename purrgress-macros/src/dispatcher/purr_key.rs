use quote;
use proc_macro;
use syn;

// A macro for automatic implementation of the PurrStep trait for your enum as a derive
pub fn derive_purr_key_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &input.ident;
    let (impl_genetics, ty_generics, where_clause) = &input.generics.split_for_impl();

    let expanded = quote::quote! {
        #[automatically_derived]
        impl #impl_genetics purrgress::cat_telegraph::dispatcher_core::PurrKey for #ident #ty_generics #where_clause {}
    };
    proc_macro::TokenStream::from(expanded)
}
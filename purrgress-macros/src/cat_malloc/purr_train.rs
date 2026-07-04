use quote;
use proc_macro;
use syn;


pub fn derive_purr_rule_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let enum_item = syn::parse_macro_input!(input as syn::ItemEnum);

    let enum_name = &enum_item.ident;

    let enum_variants_iter = enum_item.variants;

    let unpack_rule_trait = enum_variants_iter.iter().map(|variant| {
        let variant_name = &variant.ident;

        let inner_type = match &variant.fields {
            syn::Fields::Unnamed(field) => {
                &field.unnamed.first().unwrap().ty
            },
            _ => panic!("The Derive macro only supports variants with a single field in parentheses, such as Timer(PurrTimer)")
        };

        quote::quote! {
            impl purrgress::cat_malloc::train_types::UnpackRule<#inner_type> for #enum_name {
                fn unpack_ref(&self) -> Option<&#inner_type> {
                    if let Self::#variant_name(field) = self { Some(&field) } else { None }
                }

                fn unpack_mut(&mut self) -> Option<&mut #inner_type> {
                    if let Self::#variant_name(field) = self { Some(field) } else { None }
                }
            }
        }
    });

    let is_finished = enum_variants_iter.iter().map(|variant| {
        let variant_name = &variant.ident; 

        quote::quote! {
            Self::#variant_name(field) => field.is_finished() 
        }
    });
    
    let expanded = quote::quote! {
        #(#unpack_rule_trait)*
        
        impl purrgress::cat_malloc::train_types::PurrRule for #enum_name {
            fn is_finished(&mut self) -> bool {
                match self {
                    #(#is_finished),*
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
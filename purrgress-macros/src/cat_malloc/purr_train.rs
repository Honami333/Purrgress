use quote;
use proc_macro;
use syn;

pub fn derive_purr_rule_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let enum_item = syn::parse_macro_input!(input as syn::ItemEnum);
    let enum_ident = &enum_item.ident;
    let enum_variants_iter = enum_item.variants;
    let (impl_genetics, ty_generics, where_clause) = enum_item.generics.split_for_impl();

    let variant_impls = enum_variants_iter.iter().map(|variant| {
        let variant_name = &variant.ident;

        let inner_type = match &variant.fields {
            syn::Fields::Unnamed(field) => { &field.unnamed.first().unwrap().ty },
            _ => return syn::Error::new_spanned(
                variant,
                "The Derive macro only supports variants with a single field in parentheses, such as Timer(PurrTimer)"
            ).to_compile_error()
        };

        quote::quote! {
            #[automatically_derived]
            impl #impl_genetics purrgress::cat_depot::train_rule::UnpackRule<#inner_type> for #enum_ident #ty_generics #where_clause {
                #[inline] fn unpack(self) -> Option<#inner_type> { if let Self::#variant_name(field) = self { Some(field) } else { None } }
                #[inline] fn unpack_ref(&self) -> Option<&#inner_type> { if let Self::#variant_name(field) = self { Some(field) } else { None } }
                #[inline] fn unpack_mut(&mut self) -> Option<&mut #inner_type> { if let Self::#variant_name(field) = self { Some(field) } else { None } }
            }

            #[automatically_derived]
            impl #impl_genetics From<#inner_type> for #enum_ident #ty_generics #where_clause {
                #[inline] fn from(value: #inner_type) -> Self { Self::#variant_name(value) }
            }
        }
    });

    let is_finished = enum_variants_iter.iter().map(|variant| {
        let inner_type = match &variant.fields {
            syn::Fields::Unnamed(field) => { &field.unnamed.first().unwrap().ty },
            _ => return syn::Error::new_spanned(
                variant,
                "The Derive macro only supports variants with a single field in parentheses, such as Timer(PurrTimer)"
            ).to_compile_error()
        };

        let variant_name = &variant.ident; 
        quote::quote! { Self::#variant_name(field) => <#inner_type as purrgress::cat_depot::train_conditions::PurrCondition>::is_finished(field) }
    });
    
    let expanded = quote::quote! {
        #(#variant_impls)*
        
        #[automatically_derived]
        impl #impl_genetics purrgress::cat_depot::train_rule::PurrRule for #enum_ident #ty_generics #where_clause {
            #[inline]
            fn is_finished(&self) -> bool {
                match self {
                    #(#is_finished),*
                }
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
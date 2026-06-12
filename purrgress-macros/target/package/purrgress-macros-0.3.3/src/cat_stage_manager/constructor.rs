use quote;
use proc_macro;
use syn;


// A constructor macro allowing you to create a manager based on your enum
pub fn attribute_meowphosis_impl(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let enum_item = syn::parse_macro_input!(item as syn::ItemEnum);

    let name = &enum_item.ident;
    let variants_iter = enum_item.variants.iter();

    // Adding stages to the graph
    let graph_pushes = variants_iter.map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            syn::Fields::Unit => quote::quote! {
                cat_manager.add_to_graph(Self::#variant_name);
            },
            syn::Fields::Unnamed(_) => quote::quote! {
                cat_manager.add_to_graph(Self::#variant_name(0));
            },
            _ => quote::quote! {}
        }
    });

    let expanded = quote::quote! {
        #enum_item

        // An impl block that creates the manager state when calling
        // YourEnum::meowphosis_manager()
        impl #name {
            pub fn meowphosis_manager() -> purrgress::cat_stage_manager::manager::StageManager<Self> {
                let mut cat_manager = purrgress::cat_stage_manager::manager::StageManager::<Self>::new();

                #(#graph_pushes)*

                cat_manager
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
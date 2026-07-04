use syn::{Expr, Result, Token, parse::{Parse, ParseStream}};
use syn::punctuated::Punctuated;
use proc_macro;
use quote;

pub(crate) struct BuildingGrimoire {
    pub(crate) _comma1: Token![!],
    pub(crate) _comma2: Token![!],
    pub(crate) _stage_enum: Expr,
    pub(crate) _comma3: Token![:],
    pub(crate) _comma4: Token![<],
    pub(crate) mata_data: Punctuated<syn::Path, Token![,]>,
    pub(crate) _comma5: Token![>],
}

impl Parse for BuildingGrimoire {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(BuildingGrimoire {
            _comma1: input.parse()?,
            _comma2: input.parse()?,
            _stage_enum: input.parse()?,
            _comma3: input.parse()?,
            _comma4: input.parse()?,
            mata_data: Punctuated::parse_separated_nonempty(input)?,
            _comma5: input.parse()?,
        })
    }
}

// A macro for chaining smaller metadata together
// Increasing nesting up to two levels
pub fn abyssal_grimoire(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as BuildingGrimoire);

    // Calling manager creation on the library's built-in enum
    let crate_manager = quote::quote! {
        purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage::meowphosis_manager();
    };

    let mata_data_path = input.mata_data.iter();

    let expanded = quote::quote! {
        {
            let mut cat_manager = #crate_manager;

            let mut animator_meta_data = HashMap::new();

            // Creating a vector from the metadata received from the user
            let elements = [ #(#mata_data_path),* ];

            for (stage, sub_manager, meta_data) in elements {
                // Registering all sub-managers from the received metadata inside a new main manager
                let sub_manager_index = cat_manager.register_sub_manager(sub_manager);

                let purr_chain_index = purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage::PurrChain(sub_manager_index);
                
                cat_manager.add_to_graph(purr_chain_index);
                
                animator_meta_data.insert(stage, (purr_chain_index, meta_data));
            };

            // Creating complete generalized metadata and returning it to the user
            purrgress::cat_motion_blur::memory_demonium::PurrAnimator::new(cat_manager, animator_meta_data)
        }
    };

    proc_macro::TokenStream::from(expanded)
}
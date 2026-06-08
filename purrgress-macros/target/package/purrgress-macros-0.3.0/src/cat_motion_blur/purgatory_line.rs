use syn::{Expr, Result, Token, Ident, parse::{Parse, ParseStream}};
use proc_macro;
use quote;

pub(crate) struct AbyssalCavalcadeInput  {
    pub(crate) _comma_start: (Token![!], Token![!], Token![!]),
    pub(crate) animator_mata_data: Ident,
    pub(crate) _comma1: Token![:],
    pub(crate) _comma2: Token![<],
    pub(crate) purr_action: Expr,
    pub(crate) _comma3: Token![:],
    pub(crate) purr_stage: Expr,
    pub(crate) duplicate_policy: Option<(Token![,], Token![!], Expr, Token![;])>,
    pub(crate) insert_position: Option<(Token![!], Expr, Token![;])>,
    pub(crate) _comma4: Token![>],
}

impl Parse for AbyssalCavalcadeInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let _comma_start = (input.parse()?, input.parse()?, input.parse()?);
        let animator_mata_data = input.parse()?; 

        let _comma1 = input.parse()?; 
        let _comma2 = input.parse()?; 
        let purr_action = input.parse()?; 
        let _comma3 = input.parse()?; 
        let purr_stage = input.parse()?; 

        let duplicate_policy = if input.peek(Token![,]) {
            let comma1: Token![,] = input.parse()?;
            let comma2: Token![!] = input.parse()?;
            
            let cond_expr: Expr = input.parse()?;

            let comma3: Token![;] = input.parse()?;

            Some((comma1, comma2, cond_expr, comma3))
        } else {
            None
        };

        let insert_position = if input.peek(Token![!]) {
            let comma1: Token![!] = input.parse()?;
            
            let cond_expr: Expr = input.parse()?;

            let comma2: Token![;] = input.parse()?;

            Some((comma1, cond_expr, comma2))
        } else {
            None
        };

        let _comma4 = input.parse()?;

        Ok(AbyssalCavalcadeInput {
            _comma_start,
            animator_mata_data,
            _comma1,
            _comma2,
            purr_action,
            _comma3,
            purr_stage,
            duplicate_policy,
            insert_position,
            _comma4,
        })
    }
}

// A macro similar to working with single-nested queues, but for a double-nested animator queue
pub fn abyssal_march_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as AbyssalCavalcadeInput);

    let animator_mata_data = &input.animator_mata_data;

    let purr_stage = &input.purr_stage;

    let purr_action = &input.purr_action;
    
    let purr_action_str = quote::quote! { #purr_action }.to_string();

    let has_dup = input.duplicate_policy.is_some();
    let has_ins = input.insert_position.is_some();

    // Checking the validity of the selected action
    let is_valid = match purr_action_str.as_str() {
        s if s.contains("Insert") => has_dup && has_ins,
        s if s.contains("PushDelete") => !has_dup && !has_ins,
        s if s.contains("Push") && !s.contains("PushDelete") => has_dup && !has_ins,
        _ => false,
    };

    // Emitting a compile-time error if invalid
    if !is_valid {
        return syn::Error::new_spanned(
            purr_action,
            format!(
                "Invalid parameters for action '{}'. Insert needs dup and ins, Push needs dup, PushDelete needs none.", 
                purr_action_str
            )
        )
        .to_compile_error()
        .into();
    }

    // Matching policies and executing the specified action on the queue
    // Specifically adding a first-level sub-manager inside the main one
    let body = match (&input.duplicate_policy, &input.insert_position) {
        (Some((_, _, dup, _)), Some((_, ins, _))) => quote::quote! {
            purrgress_macros::purr_pounce!(
                animator : purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage::PurrChain(index),
                PurrFrameStage,
                #purr_action,
                !#dup,
                !#ins
            );
        },
        (Some((_, _, dup, _)), None) => quote::quote! {
            purrgress_macros::purr_pounce!(
                animator : purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage::PurrChain(index),
                PurrFrameStage,
                #purr_action,
                !#dup
            );
        },
        (None, None) => quote::quote! {
            purrgress_macros::purr_pounce!(
                animator : purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage::PurrChain(index),
                PurrFrameStage,
                #purr_action
            );
        },
        _ => unreachable!(), 
    };

    let expanded = quote::quote! {
        {
            let mut animator_mata_data = #animator_mata_data;

            let mut sub_manager_index = None;

            let mut animated_meta_data = Vec::new();
            
            // Reading data from metadata
            if let Some(animated_stages) = animator_mata_data.get_animated_stages(#purr_stage) {
                if let purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage::PurrChain(index) = animated_stages.0 {
                    sub_manager_index = Some(index);
                };

                for (frame_stage_index, flow_meta_data) in animated_stages.1.get_flow_stages() {
                    animated_meta_data.push((*frame_stage_index, *flow_meta_data));
                };
            };
            
            let animator = animator_mata_data.get_animator_mut();

            // Working with the received data
            if let Some(index) = sub_manager_index {
                if let Some(sub_stage_manager) = animator.get_sub_manager_mut(index) {
                    for (frame_stage_index, flow_meta_data) in animated_meta_data {
                        let last_frame_index = flow_meta_data.get_last_frame_index();
                        let get_flow_stage_chain_index = flow_meta_data.get_flow_stage_chain_index();

                        // Performing an internal frame push inside each second-level sub-manager
                        purrgress_macros::purr_tentacle!(
                            sub_stage_manager : get_flow_stage_chain_index,
                            PurrFrameStage,
                            manager_types::PurrAction::Push : last_frame_index,
                            !manager_types::DuplicatePolicy::KeepAll
                        );

                        // Adding a second-level sub-manager inside a first-level sub-manager
                        purrgress_macros::purr_pounce!(
                            sub_stage_manager : get_flow_stage_chain_index,
                            PurrFrameStage,
                            manager_types::PurrAction::Push,
                            !manager_types::DuplicatePolicy::KeepAll
                        );
                    };

                    #body
                };
            };

            animator_mata_data
        }
    };

    proc_macro::TokenStream::from(expanded)
}
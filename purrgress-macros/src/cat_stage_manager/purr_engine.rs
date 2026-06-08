use quote;
use proc_macro;
use syn::{Expr, Ident, Result, Token, parse::{Parse, ParseStream}, };
use syn::{punctuated::Punctuated, Path};


pub(crate) struct MotorUpdateArgs {
    pub(crate) manager_name: Ident,
    pub(crate) _comma1: Token![:],
    pub(crate) sub_cat_manager_procces: Expr,
    pub(crate) _comma2: Token![,],
    pub(crate) stage_enum: Expr,
    pub(crate) _comma3: Token![,],
    pub(crate) handler_fn: Path, 
    pub(crate) _comma4: Token![:],
    pub(crate) args: Punctuated<Ident, Token![,]>,
    pub(crate) rumble_policy : Option<(Token![!], Token![!], Expr)>,
}

impl Parse for MotorUpdateArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let manager_name = input.parse()?;
        let _comma1 = input.parse()?;
        let sub_cat_manager_procces = input.parse()?;
        let _comma2 = input.parse()?;
        let stage_enum = input.parse()?;
        let _comma3 = input.parse()?;

        let handler_fn = input.parse()?; 
        let _comma4 = input.parse()?;
        let args = Punctuated::parse_separated_nonempty(input)?;

        let mut rumble_policy = None;

        if input.peek(Token![!]) {
            let comma: Token![!] = input.parse()?;
            let bang: Token![!] = input.parse()?;
            
            let cond_expr: Expr = input.call(Expr::parse_without_eager_brace)?;

            rumble_policy = Some((comma, bang, cond_expr));
        };

        Ok(MotorUpdateArgs {
            manager_name,
            _comma1,
            sub_cat_manager_procces,
            _comma2,
            stage_enum,
            _comma3,
            handler_fn,
            _comma4,
            args,
            rumble_policy
        })
    }
}

// A macro designed for updating nested chains
// It features pseudo-concurrency policies
// Allowing actions to be executed out of order
pub fn purr_rumble_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as MotorUpdateArgs);

    let manager_name = &input.manager_name;
    let stage_enum= &input.stage_enum;
    let sub_cat_manager_procces = &input.sub_cat_manager_procces;

    let handle_func = &input.handler_fn;
    let args = input.args.iter();

    // Checking for concurrency or sequencing
    let queue_check_condition  = match &input.rumble_policy {
        Some((_, _, rumble)) => quote::quote! {
            (#rumble != purrgress::cat_stage_manager::manager_types::RumblePolicy::Parallel 
                && !#manager_name.sub_manager_is_first(*sm_idx, sub_manager_index))
        },
        None => quote::quote! {
            !#manager_name.sub_manager_is_first(*sm_idx, sub_manager_index)
        },
    };

    let expanded = quote::quote! {
        'macro_scope: {
            if !#manager_name.current_vec_query().contains(&#sub_cat_manager_procces) { break 'macro_scope None; }

            let #stage_enum::PurrChain(sub_manager_index) = #sub_cat_manager_procces else { break 'macro_scope None; };

            let Some(#stage_enum::PurrChain(sm_idx)) = #manager_name.first_vec_query() else { break 'macro_scope None; };
            
            if #queue_check_condition { break 'macro_scope None; };

            let Some(sub_manager_box) = #manager_name.get_sub_manager_mut(sub_manager_index) else { break 'macro_scope None; };
            
            // Extracting the sub-manager
            let mut sub_manager: 
                &mut purrgress::cat_stage_manager::manager::StageManager<#stage_enum> = &mut **sub_manager_box;

            // Calling the user update function
            #handle_func(sub_manager, #(#args),*); // Passing the sub-manager and necessary arguments into it

            // Processing all queue operation data inside the nested chain
            match sub_manager.update() {
                purrgress::cat_stage_manager::manager_types::PurrEvent::Idle => {
                    if #manager_name.current_vec_query().contains(&#sub_cat_manager_procces) {
                        if let Some(sub_manager_flag) = #manager_name
                            .get_condition_mut::<purrgress::cat_stage_manager::condition::PurrFlag>(#sub_cat_manager_procces) {

                            // Setting the flag to true when it is empty
                            sub_manager_flag.set_flag(true);
                        };
                    };

                    Some(purrgress::cat_stage_manager::manager_types::PurrEvent::Idle)
                },
                purrgress::cat_stage_manager::manager_types::PurrEvent::Running(run_stage) => {
                    Some(purrgress::cat_stage_manager::manager_types::PurrEvent::Running(run_stage))
                },
                purrgress::cat_stage_manager::manager_types::PurrEvent::Transition { from, to } => {
                    Some(purrgress::cat_stage_manager::manager_types::PurrEvent::Transition { from, to })
                },
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
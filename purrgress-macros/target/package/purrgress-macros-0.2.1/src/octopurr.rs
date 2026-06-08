use quote;
use proc_macro;
use syn::{Expr, Ident, Result, Token, parse::{Parse, ParseStream}};


pub(crate) struct TentacleGrip {
    pub(crate) manager_name: Ident,
    pub(crate) _comma1: Token![:],
    pub(crate) sub_cat_manager_procces: Expr,
    pub(crate) _comma2: Token![,],
    pub(crate) stage_enum: Ident,
    pub(crate) _comma3: Token![,],
    pub(crate) purr_action: Expr,
    pub(crate) _comma4: Token![:],
    pub(crate) stage_expr: Expr,
    pub(crate) duplicate_policy: Option<(Token![,], Token![!], Expr)>,
    pub(crate) insert_position: Option<(Token![,], Token![!], Expr)>,
}

impl Parse for TentacleGrip {
    fn parse(input: ParseStream) -> Result<Self> {
        let manager_name = input.parse()?;
        let _comma1 = input.parse()?;
        let sub_cat_manager_procces = input.parse()?;
        let _comma2 = input.parse()?;
        let stage_enum = input.parse()?;
        let _comma3 = input.parse()?;
        let purr_action = input.parse()?;
        let _comma4 = input.parse()?;
        let stage_expr = input.parse()?;
        let mut duplicate_policy = None;
        let mut insert_position = None;

        while input.peek(Token![,]) {
            let comma: Token![,] = input.parse()?;
            let bang: Token![!] = input.parse()?;
            
            let cond_expr: Expr = input.call(Expr::parse_without_eager_brace)?;

            if duplicate_policy.is_none() {
                duplicate_policy = Some((comma, bang, cond_expr));
            } else {
                insert_position = Some((comma, bang, cond_expr));
            };
        };

        Ok(TentacleGrip {
            manager_name,
            _comma1,
            sub_cat_manager_procces,
            _comma2,
            stage_enum,
            _comma3,
            purr_action,
            _comma4,
            stage_expr,
            duplicate_policy,
            insert_position,
        })
    }
}


pub fn purr_tentacle_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as TentacleGrip);

    let purr_action = &input.purr_action;

    let manager_name = &input.manager_name;
    let sub_cat_manager_procces = &input.sub_cat_manager_procces;
    let stage_enum = &input.stage_enum;
    let stage_expr = &input.stage_expr;

    let purr_action_str = quote::quote! { #purr_action }.to_string();

    let has_dup = input.duplicate_policy.is_some();
    let has_ins = input.insert_position.is_some();

    let is_valid = match purr_action_str.as_str() {
        s if s.contains("Insert") => has_dup && has_ins,
        s if s.contains("PushDelete") => !has_dup && !has_ins,
        s if s.contains("Push") && !s.contains("PushDelete") => has_dup && !has_ins,
        _ => false,
    };

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

    let body = match (&input.duplicate_policy, &input.insert_position) {
        (Some((_, _, dup)), Some((_, _, ins))) => quote::quote! {
            sub_manager.insert(#stage_expr, #dup, #ins);
        },
        (Some((_, _, dup)), None) => quote::quote! {
            sub_manager.push(#stage_expr, #dup);
        },
        (None, None) => quote::quote! {
            sub_manager.push_and_delete(#stage_expr);
        },
        _ => unreachable!(), 
    };

    let expanded = quote::quote! {
        if let MyStage::PurrChain(sub_manager_index) = #sub_cat_manager_procces {
            if let Some(sub_manager_box) = #manager_name.get_sub_manager_mut(sub_manager_index) {
                let mut sub_manager: 
                    &mut purrgress::cat_stage_manager::manager::StageManager<#stage_enum> = &mut **sub_manager_box;
                
                #body;
            };
        };
    };

    proc_macro::TokenStream::from(expanded)
}

use quote;
use proc_macro;
use syn::{Expr, Ident, Result, Token, parse::{Parse, ParseStream}};
use syn::punctuated::Punctuated;


pub(crate) struct ChainNode {
    pub(crate) stage_expr: Expr,
    pub(crate) condition_expr: Option<(Token![:], Expr)>,
}

pub(crate) struct PurrChainInput {
    pub(crate) manager_name: Ident,
    pub(crate) _comma1: Token![,],
    pub(crate) stage_enum: Ident,
    pub(crate) _comma2: Token![,],
    pub(crate) nodes: Punctuated<ChainNode, Token![=>]>,
}

impl Parse for ChainNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let stage_expr = input.parse()?;

        let condition_expr = if input.peek(Token![:]) {
            let colon: Token![:] = input.parse()?;
            let cond_expr: Expr = input.parse()?;
            Some((colon, cond_expr))
        } else {
            None
        };

        Ok(ChainNode {
            stage_expr,
            condition_expr,
        })
    }
}

impl Parse for PurrChainInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(PurrChainInput {
            manager_name: input.parse()?,
            _comma1: input.parse()?,
            stage_enum: input.parse()?,
            _comma2: input.parse()?,
            nodes: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}

pub fn new_purr_chain_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as PurrChainInput);

    let main_manager = &input.manager_name;
    let stage_enum = &input.stage_enum;

    let setup_nodes  = input.nodes.iter().map(|node| {
        let stage = &node.stage_expr;

        match &node.condition_expr {
            Some((_colon, cond_expr)) => quote::quote! {
                sub_manager.add_to_graph(#stage);
                sub_manager.set_condition(#stage, #cond_expr);
            },
            None => quote::quote! {
                sub_manager.add_to_graph(#stage);
            },
        }
    });

    let nodes_vec: Vec<&ChainNode> = input.nodes.iter().collect();

    let setup_edges = nodes_vec.windows(2).map(|pair| {
        let from_stage = &pair[0].stage_expr;
        let to_stage = &pair[1].stage_expr;
        
        quote::quote! {
            sub_manager.add_dependency(#from_stage, #to_stage);
        }
    });

    let expanded = quote::quote! {
        {
            let mut sub_manager = purrgress::cat_stage_manager::manager::StageManager::new();

            #(#setup_nodes)*
            #(#setup_edges)*

            let next_index = #main_manager.register_sub_manager(sub_manager);

            let sub_manager_stage = #stage_enum::PurrChain(next_index);

            #main_manager.add_to_graph(sub_manager_stage);

            sub_manager_stage
        }
    };

    proc_macro::TokenStream::from(expanded)
}
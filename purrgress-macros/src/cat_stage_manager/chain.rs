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

// A macro for creating a nested chain with your specified stages and execution conditions
// It sets the necessary dependencies between stages according to your layout
pub fn new_purr_chain_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as PurrChainInput);

    let main_manager = &input.manager_name;
    let stage_enum = &input.stage_enum;

    let setup_nodes  = input.nodes.iter().map(|node| {
        let stage = &node.stage_expr;

        // Adding user-specified stages to the sub-manager graph
        match &node.condition_expr {
            Some((_colon, cond_expr)) => quote::quote! {
                sub_manager.add_to_graph(#stage);

                // Setting the execution condition if specified by the user, otherwise keeping the default instant one
                sub_manager.set_condition(#stage, #cond_expr);
            },
            None => quote::quote! {
                sub_manager.add_to_graph(#stage);
            },
        }
    });

    // Grouping all user-specified stages
    let nodes_vec: Vec<&ChainNode> = input.nodes.iter().collect();

    // Setting a dependency chain within the graph in pairs using the windows(2) method
    let setup_edges = nodes_vec.windows(2).map(|pair| {
        let from_stage = &pair[0].stage_expr;
        let to_stage = &pair[1].stage_expr;
        
        quote::quote! {
            sub_manager.add_dependency(#from_stage, #to_stage);
        }
    });

    let expanded = quote::quote! {
        {
            // Creating a sub-manager
            let mut sub_manager = purrgress::cat_stage_manager::manager::StageManager::new();

            // Calling the method to add conditions and stages to the graph
            #(#setup_nodes)*
            // Calling the dependency setup method
            #(#setup_edges)*

            // Registering the sub-manager inside the main manager
            let next_index = #main_manager.register_sub_manager(sub_manager);

            let sub_manager_stage = #stage_enum::PurrChain(next_index);

            // Adding sub-manager to the graph
            #main_manager.add_to_graph(sub_manager_stage);

            // And then returning its index as PurrChain(index) — the very mandatory stage for working with macros
            sub_manager_stage
        }
    };

    proc_macro::TokenStream::from(expanded)
}
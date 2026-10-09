#![warn(missing_docs)]

use proc_macro;

mod cat_malloc {
    pub(crate) mod purr_train;
    pub(crate) mod purr_step;
}


#[proc_macro_derive(PurrRule)]
pub fn derive_purr_rule(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    cat_malloc::purr_train::derive_purr_rule_impl(input)
}

#[proc_macro_derive(PurrStep)]
pub fn derive_purr_step(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    cat_malloc::purr_step::derive_purr_step_impl(input)
}
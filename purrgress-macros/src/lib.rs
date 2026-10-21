#![warn(missing_docs)]

#[cfg(feature = "train")]
mod cat_malloc {
    pub(crate) mod purr_step;
    pub(crate) mod purr_train;
}

#[cfg(feature = "dispatcher")]
mod dispatcher {
    pub(crate) mod purr_command;
    pub(crate) mod purr_reply;
    pub(crate) mod purr_key;
}

#[cfg(feature = "dispatcher")]
#[proc_macro_derive(PurrCommand)]
pub fn derive_purr_command(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dispatcher::purr_command::derive_purr_command_impl(input)
}

#[cfg(feature = "dispatcher")]
#[proc_macro_derive(PurrReply)]
pub fn derive_purr_reply(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dispatcher::purr_reply::derive_purr_reply_impl(input)
}

#[cfg(feature = "dispatcher")]
#[proc_macro_derive(PurrKey)]
pub fn derive_purr_key(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dispatcher::purr_key::derive_purr_key_impl(input)
}

#[cfg(feature = "train")]
#[proc_macro_derive(PurrStep)]
pub fn derive_purr_step(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    cat_malloc::purr_step::derive_purr_step_impl(input)
}

#[cfg(feature = "train")]
#[proc_macro_derive(PurrRule)]
pub fn derive_purr_rule(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    cat_malloc::purr_train::derive_purr_rule_impl(input)
}
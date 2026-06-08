use proc_macro;

mod chain;
mod constructor;
mod octopurr;
mod pounce_drop;
mod purr_engine;
mod step;


/// # Example of derive_purr_step usage
///
/// ```rust
/// use purrgress_macros::PurrStep;
///
/// // You hang derive on your stage enum
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
/// pub enum MyStage {
///     Idle,
///     Walk,
///     Run,
/// 
///     // Add this stage to work with macros
///     PurrChain(usize)
/// }
///  ```
#[proc_macro_derive(PurrStep)]
pub fn derive_purr_step(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    step::derive_purr_step_impl(input)
}

/// # Example of attribute_meowphosis usage
///
/// ```rust
/// use purrgress_macros::{meowphosis, PurrStep};
///
/// // You hang derive and attribute on your stage enum
/// #[meowphosis]
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
/// pub enum MyStage {
///     Idle,
///     Walk,
///     Run,
/// 
///     // Add this stage to work with macros
///     PurrChain(usize)
/// }
/// 
/// // Create a manager with all selected stages
/// let mut cat_manager = MyStage::meowphosis_manager();
///  ```
#[proc_macro_attribute]
pub fn meowphosis(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    constructor::attribute_meowphosis_impl(item)
}

/// # Macro syntax
/// 
/// ```ignore
/// purr_pounce!(
///     main_manager,
///     YourEnumStageType,
///     SpecificStageofYourEnum : Option condition => 
///     SpecificStageofYourEnum : Option condition =>
///     ...
/// );
/// ```
/// 
/// # Example of new_purr_chain() usage
///
/// ```rust
/// use purrgress::cat_stage_manager::*;
/// use purrgress_macros::{meowphosis, PurrStep};
///
/// // You hang derive and attribute on your stage enum
/// #[meowphosis]
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
/// pub enum MyStage {
///     Idle,
///     Walk,
///     Run,
/// 
///     // Add this stage to work with macros
///     PurrChain(usize)
/// }
/// 
/// // Create a manager with all selected stages
/// let mut cat_manager = MyStage::meowphosis_manager();
/// 
/// // create all the necessary conditions
/// let idle_condition = condition::PurrTimer::new(2.0);
/// let walk_condition = condition::PurrTimer::new(2.0);
/// 
/// // Create your own chain of dependencies and their conditions
/// let sub_cat_manager_procces_1 = purrgress_macros::new_purr_chain!(
///     cat_manager,
///     MyStage,
///     MyStage::Idle : idle_condition =>
///     MyStage::Walk : walk_condition =>
///     MyStage::Run
/// );
///  ```
#[proc_macro]
pub fn new_purr_chain(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    chain::new_purr_chain_impl(input)
}


/// # Macro syntax
/// 
/// ```ignore
/// purrgress_macros::purr_tentacle!(
///     main_manager : sub_manager_variable,
///     StageEnumTypeName,
///     PurrActionType : SpecificStageofYourEnum,
/// 
///     !DuplicatePolicyType,   // Not required when using PurrAction::PushDelete
///     !InsertPositionType     // Required only if PurrAction::Insert is used
/// );
/// ```
/// 
/// # Example of purr_tentacle() usage
///
/// ```rust
/// use purrgress::cat_stage_manager::*;
/// use purrgress_macros::{meowphosis, PurrStep};
///
/// // You hang derive and attribute on your stage enum
/// #[meowphosis]
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
/// pub enum MyStage {
///     Idle,
///     Walk,
///     Run,
/// 
///     // Add this stage to work with macros
///     PurrChain(usize)
/// }
/// 
/// // Create a manager with all selected stages
/// let mut cat_manager = MyStage::meowphosis_manager();
/// 
/// // create all the necessary conditions
/// # let idle_condition = condition::PurrTimer::new(2.0);
/// # let walk_condition = condition::PurrTimer::new(2.0);
/// 
/// // Create your own chain of dependencies and their conditions
/// let sub_cat_manager_procces_1 = purrgress_macros::new_purr_chain!(
/// #    cat_manager,
/// #    MyStage,
/// #    MyStage::Idle : idle_condition =>
/// #    MyStage::Walk : walk_condition =>
/// #    MyStage::Run
/// );
/// 
/// // Select the stage and its method of implementation of your sub-manager in the queue
/// purrgress_macros::purr_tentacle!(
///     cat_manager : sub_cat_manager_procces_1,
///     MyStage,
///     manager::PurrAction::Inset : MyStage::Run,
///     !manager_types::DuplicatePolicy::RemoveMatch,
///     !manager_types::InsertPosition::Forward
/// );
///  ```
#[proc_macro]
pub fn purr_tentacle(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    octopurr::purr_tentacle_impl(input)
}


/// # Macro syntax
/// 
/// ```ignore
/// purrgress_macros::purr_pounce!(
///     main_manager : sub_manager_variable,
///     StageEnumTypeName,
///     PurrActionType,
/// 
///     !DuplicatePolicyType,   // Not required when using PurrAction::PushDelete
///     !InsertPositionType     // Required only if PurrAction::Insert is used
/// );
/// ```
/// 
/// # Example of purr_pounce() usage
///
/// ```rust
/// use purrgress::cat_stage_manager::*;
/// use purrgress_macros::{meowphosis, PurrStep};
///
/// // You hang derive and attribute on your stage enum
/// #[meowphosis]
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
/// pub enum MyStage {
///     Idle,
///     Walk,
///     Run,
/// 
///     // Add this stage to work with macros
///     PurrChain(usize)
/// }
/// 
/// // Create a manager with all selected stages
/// let mut cat_manager = MyStage::meowphosis_manager();
/// 
/// // create all the necessary conditions
/// # let idle_condition = condition::PurrTimer::new(2.0);
/// # let walk_condition = condition::PurrTimer::new(2.0);
/// 
/// // Create your own chain of dependencies and their conditions
/// let sub_cat_manager_procces_1 = purrgress_macros::new_purr_chain!(
/// #    cat_manager,
/// #    MyStage,
/// #    MyStage::Idle : idle_condition =>
/// #    MyStage::Walk : walk_condition =>
/// #    MyStage::Run
/// );
/// 
/// // Select the stage and its method of implementation of your sub-manager in the queue
/// purrgress_macros::purr_tentacle!(
/// #    cat_manager : sub_cat_manager_procces_1,
/// #    MyStage,
/// #    manager::PurrAction::Inset : MyStage::Run,
/// #    !manager_types::DuplicatePolicy::RemoveMatch,
/// #    !manager_types::InsertPosition::Forward
/// );
/// 
/// // Add a sub-manager to the main manager
/// purrgress_macros::purr_pounce!(
///     cat_manager : sub_cat_manager_procces_1,
///     MyStage,
///     manager::PurrAction::Push,
///     !manager_types::DuplicatePolicy::RemoveMatch
/// );
///  ```
#[proc_macro]
pub fn purr_pounce(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    pounce_drop::purr_pounce_impl(input)
}


/// # Macro syntax
/// 
/// ```ignore
/// purrgress_macros::purr_rumble!(
///     main_manager : sub_manager_variable,
///     StageEnumTypeName,
/// 
///     your_function_name : arguments,
/// 
///     // Enables for this queue element
///     // The feature is executed out of order - in parallel
///     !!manager_types::RumblePolicy::Parallel 
/// );
/// ```
/// 
/// # Example of purr_rumble() usage
///
/// ```rust
/// use purrgress::cat_stage_manager::*;
/// use purrgress_macros::{meowphosis, PurrStep};
///
/// // You hang derive and attribute on your stage enum
/// #[meowphosis]
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
/// pub enum MyStage {
///     Idle,
///     Walk,
///     Run,
/// 
///     // Add this stage to work with macros
///     PurrChain(usize)
/// }
/// 
/// // Create a manager with all selected stages
/// let mut cat_manager = MyStage::meowphosis_manager();
/// 
/// // create all the necessary conditions
/// # let idle_condition = condition::PurrTimer::new(2.0);
/// # let walk_condition = condition::PurrTimer::new(2.0);
/// 
/// // Create your own chain of dependencies and their conditions
/// let sub_cat_manager_procces_1 = purrgress_macros::new_purr_chain!(
/// #    cat_manager,
/// #    MyStage,
/// #    MyStage::Idle : idle_condition =>
/// #    MyStage::Walk : walk_condition =>
/// #    MyStage::Run
/// );
/// 
/// // Select the stage and its method of implementation of your sub-manager in the queue
/// purrgress_macros::purr_tentacle!(
/// #    cat_manager : sub_cat_manager_procces_1,
/// #    MyStage,
/// #    manager::PurrAction::Inset : MyStage::Run,
/// #    !manager_types::DuplicatePolicy::RemoveMatch,
/// #    !manager_types::InsertPosition::Forward
/// );
/// 
/// // Add a sub-manager to the main manager
/// purrgress_macros::purr_pounce!(
/// #    cat_manager : sub_cat_manager_procces_1,
/// #    MyStage,
/// #    manager::PurrAction::Push,
/// #    !manager_types::DuplicatePolicy::RemoveMatch
/// );
/// 
/// // Run and install the update for this sub-manager.
/// let sub_cat_manager_stage_1 = purrgress_macros::purr_rumble!(
///     cat_manager : sub_cat_manager_procces_1,
///     MyStage,
///     my_sm_func : delta
/// );
/// 
/// // Treat this sub-manager as you wish.
/// if let Some(stage) = sub_cat_manager_stage_1 {
///     match stage {
///         // The queue is empty
///         manager_types::PurrEvent::Idle => println!("The queue sub manager 1 is empty"),
/// 
///         // Active stage
///         manager_types::PurrEvent::Running(stage) => println!("The stage run {:?}", stage),,
/// 
///         // The moment of completion of the active stage
///         manager_types::PurrEvent::Transition { from, to } => {
///             println!("The action {:?} is over", from);
///
///             if let Some(to) = to {
///                 println!("Go to {:?}", to);
///             };
///         },
///     };
/// };
/// 
/// // IMPORTANT!
/// // To properly sabotage the queue, you must also call the update on the main sub-manager.
/// //See the state manager documentation!
/// 
/// // Create your own function to update conditions!
/// fn my_sm_func(sub_manager_1: &mut manager::StageManager<MyStage>, delta: f32) { ... }
///  ```
#[proc_macro]
pub fn purr_rumble(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    purr_engine::purr_rumble_impl(input)
}
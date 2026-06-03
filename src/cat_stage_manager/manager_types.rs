use std::hash::Hash;

/// # Example of PurrStep trait usage
/// use purrgress::cat_stage_manager::*;
/// 
/// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// 
/// enum MyStage { Idle, Walk }
/// 
/// impl manager_types::PurrStep for MyStage {}
pub trait PurrStep: Copy + Clone + PartialEq + Eq + Hash {}

/// # Example of DuplicatePolicy usage
///
/// ```rust
/// use purrgress::cat_stage_manager::*;
/// # #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// enum MyStage { Idle, Walk }
/// # impl manager_types::PurrStep for MyStage {}
/// 
/// # let mut cat_manager = manager::StageManager::new();
/// cat_manager.add_to_graph(MyStage::Idle);
/// cat_manager.add_to_graph(MyStage::Walk);
/// 
/// cat_manager.add_dependency(MyStage::Idle , MyStage::Walk);
/// 
/// cat_manager.push(MyStage::Idle, manager_types::DuplicatePolicy::KeepAll);
/// // print [Idle]
/// 
/// cat_manager.push(MyStage::Walk, manager_types::DuplicatePolicy::KeepAll);
/// // print [Idle, Idle, Walk]
/// 
/// cat_manager.push(MyStage::Walk, manager_types::DuplicatePolicy::RemoveMatch);
/// // print [Idle, Walk]
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicatePolicy  {
    KeepAll,
    RemoveMatch,
}

/// # Example of InsertPosition usage
///
/// ```rust
/// use purrgress::cat_stage_manager::*;
/// # #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// enum MyStage { Idle, Walk }
/// # impl manager_types::PurrStep for MyStage {}
/// 
/// # let mut cat_manager = manager::StageManager::new();
/// cat_manager.add_to_graph(MyStage::Idle);
/// cat_manager.add_to_graph(MyStage::Walk);
/// 
/// cat_manager.add_dependency(MyStage::Idle , MyStage::Walk);
/// 
/// cat_manager.push(MyStage::Walk, manager_types::DuplicatePolicy::KeepAll);
/// // print [Idle, Walk]
/// 
/// cat_manager.insert(MyStage::Walk, manager_types::DuplicatePolicy::KeepAll, manager_types::InsertPosition::Forward);
/// // print [Idle, Walk, Idle, Walk]
/// 
/// cat_manager.insert(MyStage::Walk, manager_types::DuplicatePolicy::KeepAll, manager_types::InsertPosition::Index(1));
/// // print [Idle, Idle, Walk, Walk]
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertPosition {
    Forward,
    Index(usize)
}

/// # Example of PurrEvent usage
///
/// ```rust
/// use purrgress::cat_stage_manager::*;
/// 
/// # #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// # enum MyStage { Idle }
/// # impl manager_types::PurrStep for MyStage {}
/// # 
/// # let mut cat_manager = manager::StageManager::new();
/// # cat_manager.add_to_graph(MyStage::Idle);
/// # 
/// # let idle_condition = condition::PurrTimer::new(1.0);
/// # cat_manager.set_condition(MyStage::Idle, idle_condition);
/// # 
/// # cat_manager.push(MyStage::Idle, manager_types::DuplicatePolicy::KeepAll);
///
/// loop {
///     # if let Some(timer) = cat_manager.get_condition_mut::<condition::PurrTimer>(MyStage::Idle) {
///         # timer.tick(0.1);
///     # };
/// 
///     match cat_manager.update() {
///         manager_types::PurrEvent::Idle => break,
///         manager_types::PurrEvent::Running(stage) => println!("In progress: {:?}", stage),
///         manager_types::PurrEvent::Transition { from, to } => {
///             println!("Stage {:?} Completed!", from);
///             if let Some(next) = to {
///                 println!("Swap to: {:?}", next);
///             };
///         },
///     };
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PurrEvent<T> {
    Idle,
    Running(T),
    Transition { from: T, to: Option<T> }
}

/// # Example of PurrAction usage
///
/// ```rust
/// use purrgress::cat_stage_manager::*;
/// use purrgress_macros::{meowphosis, PurrStep};
/// 
/// #[meowphosis]
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
/// pub enum MyStage {
///     Idle,
///     Walk,
///     PurrChain(usize)
/// }
///
/// let mut cat_manager = MyStage::meowphosis_manager();
/// 
/// let idle_condition = condition::PurrTimer::new(2.0);
/// let walk_condition = condition::PurrTimer::new(2.0);
/// 
/// let sub_cat_manager_procces_1 = purrgress_macros::new_purr_chain!(
///     cat_manager,
///     MyStage,
///     MyStage::Idle : idle_condition =>
///     MyStage::Walk : walk_condition
/// );
/// 
/// purrgress_macros::purr_tentacle!(
///     cat_manager : sub_cat_manager_procces_1,
///     MyStage,
///     manager::PurrAction::Push : MyStage::Walk,
///     !manager_types::DuplicatePolicy::RemoveMatch
/// );
///
/// purrgress_macros::purr_pounce!(
///     cat_manager : sub_cat_manager_procces_1,
///     MyStage,
///     manager::PurrAction::Push,
///     !manager_types::DuplicatePolicy::RemoveMatch
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PurrAction {
    Push,
    PushDelete,
    Insert,
}

/// # Example of RumblePolicy usage
///
/// ```rust
/// use purrgress::cat_stage_manager::*;
/// use purrgress_macros::{meowphosis, PurrStep};
/// 
/// #[meowphosis]
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
/// pub enum MyStage {
///     Idle,
///     Walk,
///     PurrChain(usize)
/// }
/// 
/// let delta = 0.01; // Your time delta
///
/// let mut cat_manager = MyStage::meowphosis_manager();
/// 
/// let idle_condition = condition::PurrTimer::new(2.0);
/// let walk_condition = condition::PurrTimer::new(2.0);
/// 
/// let sub_cat_manager_procces_1 = purrgress_macros::new_purr_chain!(
///     cat_manager,
///     MyStage,
///     MyStage::Idle : idle_condition =>
///     MyStage::Walk : walk_condition
/// );
/// 
/// purrgress_macros::purr_tentacle!(
///     cat_manager : sub_cat_manager_procces_1,
///     MyStage,
///     manager::PurrAction::Push : MyStage::Walk,
///     !manager_types::DuplicatePolicy::RemoveMatch
/// );
///
/// purrgress_macros::purr_pounce!(
///     cat_manager : sub_cat_manager_procces_1,
///     MyStage,
///     manager::PurrAction::Push,
///     !manager_types::DuplicatePolicy::RemoveMatch
/// );
/// 
/// let sub_cat_manager_stage_1 = purrgress_macros::purr_rumble!(
///     cat_manager : sub_cat_manager_procces_1,
///     MyStage,
///     sub_manager_procces_1_func : delta
/// 
///     // Enables for this queue element
///     // The feature is executed out of order - in parallel
///     !!manager_types::RumblePolicy::Parallel 
/// );
/// 
/// fn sub_manager_procces_1_func(sub_manager_1: &mut manager::StageManager<MyStage>, delta: f32) {
/// 
///     if let Some(idle_time) = sub_manager_1.get_condition_mut::<condition::PurrTimer>(MyStage::Idle) {
///         idle_time.tick(delta);
///     };
/// 
///     if let Some(walk_time) = sub_manager_1.get_condition_mut::<condition::PurrTimer>(MyStage::Walk) {
///         walk_time.tick(delta);
///     };
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RumblePolicy {
    StrictOrder,
    Parallel,
}
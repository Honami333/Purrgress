use std::hash::Hash;
use std::fmt::Debug;


// A structural file containing policies as enums
//  Custom structures similar to Vec2
//  And the main trait for stages

/// # Example of PurrStep trait usage
/// use purrgress::cat_stage_manager::*;
/// 
/// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// 
/// enum MyStage { Idle, Walk }
/// 
/// impl manager_types::PurrStep for MyStage {}
pub trait PurrStep: Debug + Copy + Clone + PartialEq + Eq + Hash {}

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
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
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
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
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
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
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
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
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
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RumblePolicy {
    StrictOrder,
    Parallel,
}

/// #PurrVec
/// // Vector2 with values ​​in the form of x and y coordinates
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PurrVec {
    pub x: f32,
    pub y: f32,
}

impl Default for PurrVec {
    /// # Example of PurrVec create
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// 
    /// // Creates a vector2 with zero values
    /// let my_vec2 = manager_types::PurrVec::default();
    /// ```
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl PurrVec {
    /// # Example of PurrVec create
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// 
    /// // Creates a vector2 with any values
    /// let my_vec2 = manager_types::PurrVec::new(1.0, 1.0);
    /// ```
    pub const fn new(x: f32, y: f32) -> Self {
        PurrVec { x, y }
    }
}

impl PurrVec {
    /// # Example of PurrVec - length_squared() usage
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// 
    /// // Creates a vector2 with any values
    /// let my_vec2 = manager_types::PurrVec::new(1.0, 1.0);
    /// 
    /// // Calculates the squared length of a vector for easy comparisons
    /// let length_squared = my_vec2.length_squared();
    /// ```
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// # Example of PurrVec - length() usage
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// 
    /// // Creates a vector2 with any values
    /// let my_vec2 = manager_types::PurrVec::new(1.0, 1.0);
    /// 
    /// // Calculates the length of a vector for convenient comparisons but is a more expensive operation
    /// let length = my_vec2.length();
    /// ```
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// # Example of PurrVec - distance() usage
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// 
    /// // Creates a vector2 with any values
    /// let my_vec1 = manager_types::PurrVec::new(1.0, 1.0);
    /// let my_vec2 = manager_types::PurrVec::new(2.0, 10.0);
    /// 
    /// // Calculates the distance between two vectors
    /// let distance = my_vec1.distance(my_vec2);
    /// ```
    pub fn distance(&self, other: Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn distance_not_sqrt(&self, other: Self) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    /// # Example of PurrVec - normalize() usage
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// 
    /// // Creates a vector2 with any values
    /// let my_vec2 = manager_types::PurrVec::new(1.0, 1.0);
    /// 
    /// // Normalizes vector coordinates for convenient work with directions, etc.
    /// let my_vec2_normalize = my_vec2.normalize();
    /// ```
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self { x: self.x / len, y: self.y / len }
        } else {
            Self::default()
        }
    }
}

impl PurrVec {
    /// # Example of PurrVec - zero() usage
    ///
    /// ```rust
    /// // Creates a vector with values: 0.0 0.0
    /// let zero_vec = manager_types::PurrVec::zero();
    /// ```
    pub const fn zero() -> Self { Self::new(0.0, 0.0) }
    /// # Example of PurrVec - one() usage
    ///
    /// ```rust
    /// // Creates a vector with values: 1.0 1.0
    /// let one_vec = manager_types::PurrVec::one();
    /// ```
    pub const fn one() -> Self { Self::new(1.0, 1.0) }
    /// # Example of PurrVec - left() usage
    ///
    /// ```rust
    /// // Creates a vector with values: -1.0 0.0
    /// let left_vec = manager_types::PurrVec::left();
    /// ```
    pub const fn left() -> Self { Self::new(-1.0, 0.0) }
    /// # Example of PurrVec - right() usage
    ///
    /// ```rust
    /// // Creates a vector with values: 1.0 0.0
    /// let right_vec = manager_types::PurrVec::right();
    /// ```
    pub const fn right() -> Self { Self::new(1.0, 0.0) }
    /// # Example of PurrVec - up() usage
    ///
    /// ```rust
    /// // Creates a vector with values: 0.0 1.0
    /// let up_vec = manager_types::PurrVec::up();
    /// ```
    pub const fn up() -> Self { Self::new(0.0, 1.0) }
    /// # Example of PurrVec - down() usage
    ///
    /// ```rust
    /// // Creates a vector with values: 0.0 -1.0
    /// let down_vec = manager_types::PurrVec::down();
    /// ```
    pub const fn down() -> Self { Self::new(0.0, -1.0) }
}
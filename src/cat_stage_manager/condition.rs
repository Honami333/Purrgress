use std::fmt::Debug;

use super::manager_types::*;


// A file for built-in conditions
// You can learn to create your own custom conditions by using the built-in ones as an example

// Too lazy to comment self-explanatory functions that you have likely encountered in other crates!!!

/// # Example of PurrCondition trait usage
/// use purrgress::cat_stage_manager::*;
/// 
/// #[derive(Debug)]
/// pub struct InstantCondition;
/// 
/// impl condition::PurrCondition for InstantCondition {
/// 
///     fn is_finished(&mut self) -> bool { true }
///     fn reset(&mut self) {}
///     fn as_any(&self) -> &dyn std::any::Any { self }
///     fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
/// }
pub trait PurrCondition: Debug {
    fn is_finished(&mut self) -> bool;
    fn reset(&mut self);

    fn as_any(&self) -> &(dyn std::any::Any + 'static);
    fn as_any_mut(&mut self) -> &mut (dyn std::any::Any + 'static);
}

/// # Example of InstantCondition usage
///
/// ```rust
/// use purrgress::cat_stage_manager::*;
/// 
/// # #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// # enum MyStage { Idle }
/// # impl manager_types::PurrStep for MyStage {}
///
/// let mut cat_manager = manager::StageManager::new();
/// 
/// cat_manager.add_to_graph(MyStage::Idle);
/// 
/// // It's a bit useless because this value is already set by default.
/// let idle_condition = condition::InstantCondition;
/// cat_manager.set_condition(MyStage::Idle, idle_condition);
/// ```
#[derive(Debug)]
pub struct InstantCondition;

impl PurrCondition for InstantCondition {
    fn is_finished(&mut self) -> bool { true }

    fn reset(&mut self) {}

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

/// # Example of PurrTimer usage
///
/// ```rust
/// use purrgress::cat_stage_manager::*;
/// 
/// # #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// # enum MyStage { Idle }
/// # impl manager_types::PurrStep for MyStage {}
///
/// let mut cat_manager = manager::StageManager::new();
/// 
/// cat_manager.add_to_graph(MyStage::Idle);
/// 
/// let idle_condition = condition::PurrTimer::new(1.0);;
/// cat_manager.set_condition(MyStage::Idle, idle_condition);
/// ```
#[derive(Debug)]
pub struct PurrTimer {
    duration: f32,
    time_left: f32,
}

impl Default for PurrTimer {
    fn default() -> Self {
        PurrTimer::new(1.0)
    }
}

impl PurrTimer {
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            time_left: 0.0
        }
    }

    pub fn tick(&mut self, delta: f32) {
        self.time_left += delta;
    }
}

impl PurrTimer {
    pub fn get_duration(&self) -> &f32 {
        &self.duration
    }

    pub fn get_duration_mut(&mut self) -> &mut f32 {
        &mut self.duration
    }

    pub fn get_time_left(&self) -> &f32 {
        &self.time_left
    }

    pub fn get_time_left_mut(&mut self) -> &mut f32 {
        &mut self.time_left
    }
}

impl PurrCondition for PurrTimer {
    fn is_finished(&mut self) -> bool {
        if self.time_left < self.duration { return  false; }
        
        self.reset();
        true
    }

    fn reset(&mut self) {
        self.time_left = 0.0;
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

/// # Example of PurrFlag usage
///
/// ```rust
/// use purrgress::cat_stage_manager::*;
/// 
/// # #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// # enum MyStage { Idle }
/// # impl manager_types::PurrStep for MyStage {}
///
/// let mut cat_manager = manager::StageManager::new();
/// 
/// cat_manager.add_to_graph(MyStage::Idle);
/// 
/// let idle_condition = condition::PurrFlag::new();
/// cat_manager.set_condition(MyStage::Idle, idle_condition);
/// ```
#[derive(Debug)]
pub struct PurrFlag {
    select_flag: bool,
}

impl Default for PurrFlag {
    fn default() -> Self {
        PurrFlag::new()
    }
}

impl PurrFlag {
    pub fn new() -> Self {
        Self {
            select_flag: false
        }
    }

    pub fn get_flag(&self) -> bool {
        self.select_flag
    }

    pub fn set_flag(&mut self, flag: bool){
        self.select_flag = flag;
    }

    pub fn reverse_flag(&mut self){
        self.select_flag = !self.select_flag;
    }
}

impl PurrCondition for PurrFlag {
    fn is_finished(&mut self) -> bool {
        self.select_flag
    }

    fn reset(&mut self) {
        self.select_flag = false;
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

/// # Example of PurrProximity usage
///
/// ```rust
/// use purrgress::cat_stage_manager::*;
/// 
/// # #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// # enum MyStage { Idle }
/// # impl manager_types::PurrStep for MyStage {}
///
/// let mut cat_manager = manager::StageManager::new();
/// 
/// cat_manager.add_to_graph(MyStage::Idle);
/// 
/// // Initializing the coordinate conditions, where:
/// let idle_condition = condition::PurrProximity::new(
///     // The first vector is the current value
///     manager_type::PurrVec::new(1.0),
/// 
///     // The second is the starting point to reset to
///     manager_type::PurrVec::new(1.0),
/// 
///     // The third value indicates the end of the path
///     manager_type::PurrVec::new(1.0)
/// );
/// 
/// cat_manager.set_condition(MyStage::Idle, idle_condition);
/// ```
#[derive(Debug)]
pub struct PurrProximity {
    pos: PurrVec,
    start_pos: PurrVec,
    target_pos: PurrVec,
}

// Creating a PurrProximity condition with zero values
impl Default for PurrProximity {
    fn default() -> Self {
        Self::new(PurrVec::zero(), PurrVec::zero(), PurrVec::zero())
    }
}

// Create PurrProximity condition
impl PurrProximity {
    pub fn new(pos: PurrVec, start_pos:PurrVec, target_pos: PurrVec) -> Self {
        PurrProximity {
            pos,
            start_pos,
            target_pos
        }
    }
}

// Retrieving mutable and immutable data about the current condition
impl PurrProximity {
    pub fn get_start_pos(&self) -> PurrVec {
        self.start_pos
    }

    pub fn get_start_pos_mut(&mut self) -> &mut PurrVec {
        &mut self.start_pos
    }

    pub fn get_target_pos(&self) -> PurrVec {
        self.target_pos
    }

    pub fn get_target_pos_mut(&mut self) -> &mut PurrVec {
        &mut self.target_pos
    }

    pub fn get_pos(&self) -> PurrVec {
        self.pos
    }

    pub fn get_pos_mut(&mut self) -> &mut PurrVec {
        &mut self.pos
    }
}

impl PurrProximity {
    // Adding another vector to the current position vector
    pub fn pos_add(&mut self, add: PurrVec) {
        self.pos.x += add.x;
        self.pos.y += add.y;
    } 

    // Subtracting another vector from the current position vector
    pub fn pos_sub(&mut self, sub: PurrVec) {
        self.pos.x -= sub.x;
        self.pos.y -= sub.y;
    }

    // Multiplying the current position vector by another vector
    pub fn pos_mul(&mut self, mul: PurrVec) {
        self.pos.x *= mul.x;
        self.pos.y *= mul.y;
    }

    // Dividing the current position vector by another vector
    pub fn pos_div(&mut self, div: PurrVec) {
        if div.x != 0.0 { self.pos.x /= div.x };
        if div.y != 0.0 { self.pos.y /= div.y };
    }

    // Distance from the current position to the target position
    pub fn get_distance(&self) -> f32 {
        self.pos.distance(self.target_pos)
    }
}

impl PurrCondition for PurrProximity {
    fn is_finished(&mut self) -> bool {
        self.pos == self.target_pos
    }

    fn reset(&mut self) {
        self.pos = self.start_pos;
    }

    fn as_any(&self) -> &dyn std::any::Any { self }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
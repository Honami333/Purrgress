use std::fmt::Debug;

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
/// let idle_condition = condition::PurrFlag::new();;
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
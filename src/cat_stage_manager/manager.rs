use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

use crate::condition::*;
use crate::types::*;

pub use purrgress_macros::{purr_pounce, purr_tentacle, purr_rumble, new_purr_chain, meowphosis, PurrStep};


// The crate's root file containing the manager structure
//  Graph operations
//  The queue update system
//  And functions for retrieving data from the queue

#[doc = "## Example of StageManager usage"]
#[doc = ""]
#[doc = include_str!("../../examples/basic_test.rs")]
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug)]
pub struct StageManager<T: PurrStep> {
    pub(crate) vector_stage: Vec<T>,
    pub(crate) graph: DiGraph<T, ()>,
    pub(crate) nodes: HashMap<T, NodeIndex>,
    pub(crate) conditions: HashMap<T, Box<dyn PurrConditionAny>>,
    pub(crate) sub_managers: HashMap<usize, Box<StageManager<T>>>,
}

impl<T> Default for StageManager<T> 
where 
    T: PurrStep
{
    /// # Example of StageManager create
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    /// enum MyStage { Idle }
    /// impl manager_types::PurrStep for MyStage {}
    /// 
    /// let mut cat_manager: manager::StageManager<MyStage> = manager::StageManager::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}


impl<T> StageManager<T> 
where 
    T: PurrStep
{
    /// # Example of StageManager create
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    /// enum MyStage { Idle }
    /// impl manager_types::PurrStep for MyStage {}
    /// 
    /// let mut cat_manager: manager::StageManager<MyStage> = manager::StageManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            vector_stage: Vec::new(),
            graph: DiGraph::new(),
            nodes: HashMap::new(),
            conditions: HashMap::new(),
            sub_managers: HashMap::new(),
        }
    }

    /// # Example of StageManager - add_to_graph() usage
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    /// enum MyStage { Idle }
    /// impl manager_types::PurrStep for MyStage {}
    /// 
    /// let mut cat_manager = manager::StageManager::new();
    /// cat_manager.add_to_graph(MyStage::Idle);
    /// ```
    pub fn add_to_graph(&mut self, stage: T) {
        if self.nodes.contains_key(&stage) { return; };

        let node_index = self.graph.add_node(stage);

        self.nodes.insert(stage, node_index);

        self.set_condition(stage, InstantCondition)
    }
}


impl<T> StageManager<T>  
where 
    T: PurrStep
{
    /// # Example of StageManager - update() usage
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// 
    /// # #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    /// # enum MyStage { Idle }
    /// # impl manager_types::PurrStep for MyStage {}
    /// # 
    /// let mut cat_manager = manager::StageManager::new();
    /// # cat_manager.add_to_graph(MyStage::Idle);
    /// # 
    /// # let idle_condition = condition::PurrTimer::new(1.0);
    /// # cat_manager.set_condition(MyStage::Idle, idle_condition);
    /// # 
    /// # cat_manager.push(MyStage::Idle, manager_types::DuplicatePolicy::KeepAll);
    ///
    /// loop {
    ///     if let Some(timer) = cat_manager.get_condition_mut::<condition::PurrTimer>(MyStage::Idle) {
    ///         timer.tick(0.1);
    ///     };
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
    pub fn update(&mut self) -> PurrEvent<T> {
        let mut event = PurrEvent::Idle;

        if self.vector_stage.is_empty() { return event; };

        let Some(conditions) = self.conditions.get_mut(&self.vector_stage[0]) else { 
            return event; 
        };
        
        if conditions.is_finished() {
            let running_stage = self.vector_stage[0];
            let next_stage = self.vector_stage.get(1).copied();
            self.vector_stage.remove(0);

            if let Some(next) = next_stage 
                && let Some(next_condition) = self.conditions.get_mut(&next) {
                    next_condition.reset();
            };

            event = PurrEvent::Transition { from: running_stage, to: next_stage }
        } else {
            let running_stage = self.vector_stage[0];

            event = PurrEvent::Running(running_stage)
        };

        event
    }
}


impl<T> StageManager<T> 
where 
    T: PurrStep
{
    /// # Example of StageManager - current_vec_query() usage
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// 
    /// # #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    /// # enum MyStage { Idle }
    /// # impl manager_types::PurrStep for MyStage {}
    ///
    /// let mut cat_manager = manager::StageManager::new();
    /// # cat_manager.add_to_graph(MyStage::Idle);
    /// 
    /// cat_manager.push(MyStage::Idle, manager_types::DuplicatePolicy::KeepAll);
    /// 
    /// let current_vec_query = cat_manager.current_vec_query();
    /// 
    /// println!("{:?}", current_vec_query);
    /// ```
    pub fn current_vec_query(&self) -> &[T] {
        &self.vector_stage
    }

    /// # Example of StageManager - len_vec_query() usage
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
    /// # cat_manager.add_to_graph(MyStage::Idle);
    /// 
    /// cat_manager.push(MyStage::Idle, manager_types::DuplicatePolicy::KeepAll);
    /// 
    /// let len_vec_query = cat_manager.len_vec_query();
    /// 
    /// println!("{:?}", len_vec_query);
    /// ```
    pub fn len_vec_query(&self) -> usize {
        self.current_vec_query().len()
    }

    /// # Example of StageManager - query_is_empty() usage
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
    /// # cat_manager.add_to_graph(MyStage::Idle);
    /// 
    /// cat_manager.push(MyStage::Idle, manager_types::DuplicatePolicy::KeepAll);
    /// 
    /// if cat_manager.query_is_empty() {
    ///     println!("{:?}", MyStage::Idle);
    /// };
    /// ```
    pub fn query_is_empty(&self) -> bool {
        self.current_vec_query().is_empty()
    }

    /// # Example of StageManager - first_vec_query() usage
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
    /// # cat_manager.add_to_graph(MyStage::Idle);
    /// 
    /// cat_manager.push(MyStage::Idle, manager_types::DuplicatePolicy::KeepAll);
    /// 
    /// let first_vec_query = cat_manager.first_vec_query();
    /// 
    /// println!("{:?}", first_vec_query);
    /// ```
    pub fn first_vec_query(&self) -> Option<&T> {
        self.current_vec_query().first()
    }

    /// # Example of StageManager - last_vec_query() usage
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
    /// # cat_manager.add_to_graph(MyStage::Idle);
    /// 
    /// cat_manager.push(MyStage::Idle, manager_types::DuplicatePolicy::KeepAll);
    /// 
    /// let last_vec_query = cat_manager.last_vec_query();
    /// 
    /// println!("{:?}", last_vec_query);
    /// ```
    pub fn last_vec_query(&self) -> Option<&T> {
        self.current_vec_query().last()
    }

    /// # Example of StageManager - next_vec_query() usage
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
    /// # cat_manager.add_to_graph(MyStage::Idle);
    /// 
    /// cat_manager.push(MyStage::Idle, manager_types::DuplicatePolicy::KeepAll);
    /// 
    /// let next_vec_query = cat_manager.next_vec_query();
    /// 
    /// println!("{:?}", next_vec_query);
    /// ```
    pub fn next_vec_query(&self) -> Option<&T> {
        self.current_vec_query().get(1)
    }

    /// # Example of StageManager - contains_stage() usage
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
    /// # cat_manager.add_to_graph(MyStage::Idle);
    /// 
    /// cat_manager.push(MyStage::Idle, manager_types::DuplicatePolicy::KeepAll);
    /// 
    /// if cat_manager.contains_stage(&MyStage::Idle){
    ///     println!("{:?}", MyStage::Idle);
    /// };
    /// ```
    pub fn contains_stage(&self, stage: &T) -> bool {
        self.current_vec_query().contains(stage)
    }

    /// # Example of StageManager - clear_query() usage
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
    /// # cat_manager.add_to_graph(MyStage::Idle);
    /// 
    /// cat_manager.push(MyStage::Idle, manager_types::DuplicatePolicy::KeepAll);
    /// 
    /// cat_manager.clear_query();
    /// 
    /// let current_vec_query = cat_manager.current_vec_query();
    /// 
    /// println!("{:?}", current_vec_query);
    /// ```
    pub fn clear_query(&mut self) {
        self.vector_stage.clear();
    }
}


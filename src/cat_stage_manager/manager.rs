use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{Reversed, Dfs};
use std::collections::HashMap;
use std::hash::Hash;

use super::condition::*;


pub trait PurrStep: Copy + Clone + PartialEq + Eq + Hash {}

pub enum DuplicatePolicy  {
    KeepAll,
    RemoveMatch,
}

pub enum InsertPosition {
    Forward,
    Index(usize)
}

pub enum PurrEvent<T> {
    Idle,
    Running(T),
    Transition { from: T, to: Option<T> }
}

pub struct StageManager<T: PurrStep> {
    vector_stage: Vec<T>,
    graph: DiGraph<T, ()>,
    nodes: HashMap<T, NodeIndex>,
    conditions: HashMap<T, Box<dyn PurrCondition>>,
}


impl<T> StageManager<T> 
where 
    T: PurrStep
{
     pub fn new() -> Self {
        Self {
            vector_stage: Vec::new(),
            graph: DiGraph::new(),
            nodes: HashMap::new(),
            conditions: HashMap::new(),
        }
    }

    pub fn add_to_graph(&mut self, stage: T){
        if !self.nodes.contains_key(&stage) {
            let node_index = self.graph.add_node(stage);

            self.nodes.insert(stage, node_index);

            self.set_condition(stage, Box::new(InstantCondition))
        };
    }
}


impl<T> StageManager<T>  
where 
    T: PurrStep
{
    pub fn update(&mut self) -> PurrEvent<T> {
        let mut ivent = PurrEvent::Idle;

        if !self.vector_stage.is_empty() {
            if let Some(conditions) = self.conditions.get_mut(&self.vector_stage[0]) {
                if conditions.is_finished() {
                    let running_stage = self.vector_stage[0];
                    let next_stage = self.vector_stage.get(1).copied();

                    self.vector_stage.remove(0);

                    if let Some(next) = next_stage {
                        if let Some(next_condition) = self.conditions.get_mut(&next) {
                            next_condition.reset();
                        };
                    };

                    ivent = PurrEvent::Transition { from: running_stage, to: next_stage }
                } else {
                    let running_stage = self.vector_stage[0];

                    ivent = PurrEvent::Running(running_stage)
                };
            };
        };

        ivent
    }
}


impl<T> StageManager<T> 
where 
    T: PurrStep
{
    pub fn set_condition(&mut self, stage: T, condition: Box<dyn PurrCondition>) {
        self.conditions.insert(stage, condition);
    }

    pub fn get_condition<U>(&self, stage: T) -> Option<&U>
    where 
        U: 'static
    {
        let boxed_condition = self.conditions.get(&stage)?;

        let any = boxed_condition.as_any();

        any.downcast_ref::<U>()
    }

    pub fn get_condition_mut<U>(&mut self, stage: T) -> Option<&mut U>
    where 
        U: 'static
    {
        let boxed_condition  = self.conditions.get_mut(&stage)?;

        let any_mut = boxed_condition.as_any_mut();

        any_mut.downcast_mut::<U>()
    }
}


impl<T> StageManager<T> 
where 
    T: PurrStep
{
    pub fn add_dependency(&mut self, from: T, to: T) {
        if let (Some(&from_idx), Some(&to_idx)) = (self.nodes.get(&from), self.nodes.get(&to)) {
            if !self.graph.contains_edge(from_idx, to_idx) {
                self.graph.add_edge(from_idx, to_idx, ());
            }
        };
    }

    pub fn push(&mut self, target: T, duplicate_policy: DuplicatePolicy) {
        let mut new_stages = self.calculate_dependencies(target);

        self.check_duplicate(&mut new_stages, duplicate_policy);
        
        self.vector_stage.extend(new_stages);
    }

    pub fn push_and_delete(&mut self, target: T) {
        let new_stages = self.calculate_dependencies(target);

        self.vector_stage = new_stages;
    }

    pub fn insert(
        &mut self,
        target: T,
        duplicate_policy: DuplicatePolicy,
        index: InsertPosition,
    ) {
        let mut new_stages = self.calculate_dependencies(target);

        self.check_duplicate(&mut new_stages, duplicate_policy);

        let target_index = match index {
            InsertPosition::Forward => 0,
            InsertPosition::Index(idx) => {
                if idx > self.len_vec_query() {
                    self.len_vec_query()
                } else {
                    idx
                }
            },
        };

        self.vector_stage.splice(target_index..target_index, new_stages);
    }

    fn check_duplicate(&self, new_stages: &mut Vec<T>, duplicate_policy: DuplicatePolicy) {
        match duplicate_policy {
            DuplicatePolicy::KeepAll => (),
            DuplicatePolicy::RemoveMatch => {
                if !new_stages.is_empty() && !self.vector_stage.is_empty() {
                    if new_stages.first() == self.vector_stage.last() {
                        new_stages.remove(0);
                    };
                };
            },
        };
    }

    fn calculate_dependencies(&self, target: T) -> Vec<T> {
        let Some(&target_idx) = self.nodes.get(&target) else { return Vec::new();};

        let mut path = Vec::new();

        let reversed_graph = Reversed(&self.graph);

        let mut dfs = Dfs::new(reversed_graph, target_idx);

        while let Some(node_index) = dfs.next(reversed_graph) {
            let stage = self.graph[node_index];

            path.push(stage);
        }

        path.reverse();
        path
    }
}


impl<T> StageManager<T> 
where 
    T: PurrStep
{
    pub fn len_vec_query(&self) -> usize {
        self.vector_stage.len()
    }

    pub fn current_vec_query(&self) -> Vec<T> {
        self.vector_stage.clone()
    }
}
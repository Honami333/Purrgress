use petgraph::visit::{Reversed, Dfs};

use super::manager_types::*;
use super::manager::*;


// A file for working with the queue
// Here are all the functions for inserting stages into the queue

impl<T> StageManager<T> 
where 
    T: PurrStep
{
    /// # Example of StageManager - add_dependency() usage
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// # #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    /// enum MyStage { Idle, Walk }
    /// # impl manager_types::PurrStep for MyStage {}
    /// 
    /// let mut cat_manager = manager::StageManager::new();
    /// cat_manager.add_to_graph(MyStage::Idle);
    /// cat_manager.add_to_graph(MyStage::Walk);
    /// 
    /// cat_manager.add_dependency(MyStage::Idle , MyStage::Walk);
    pub fn add_dependency(&mut self, from: T, to: T) {
        let (Some(&from_idx), Some(&to_idx)) = (self.nodes.get(&from), self.nodes.get(&to)) else { return; };

        if self.graph.contains_edge(from_idx, to_idx) { return; }

        self.graph.add_edge(from_idx, to_idx, ());
    }

    /// # Example of StageManager - push() usage
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// # #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    /// enum MyStage { Idle, Walk }
    /// # impl manager_types::PurrStep for MyStage {}
    /// 
    /// let mut cat_manager = manager::StageManager::new();
    /// # cat_manager.add_to_graph(MyStage::Idle);
    /// # cat_manager.add_to_graph(MyStage::Walk);
    /// 
    /// # cat_manager.add_dependency(MyStage::Idle , MyStage::Walk);
    /// 
    /// cat_manager.push(MyStage::Walk, manager_types::DuplicatePolicy::KeepAll);
    /// // print [Idle, Walk]
    pub fn push(&mut self, target: T, duplicate_policy: DuplicatePolicy) {
        let mut new_stages = self.calculate_dependencies(target);

        self.check_duplicate(&mut new_stages, duplicate_policy);
        
        self.vector_stage.extend(new_stages);
    }

    /// # Example of StageManager - push_and_delete() usage
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// # #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    /// enum MyStage { Idle, Walk }
    /// # impl manager_types::PurrStep for MyStage {}
    /// 
    /// let mut cat_manager = manager::StageManager::new();
    /// # cat_manager.add_to_graph(MyStage::Idle);
    /// # cat_manager.add_to_graph(MyStage::Walk);
    /// 
    /// # cat_manager.add_dependency(MyStage::Idle , MyStage::Walk);
    /// 
    /// cat_manager.push(MyStage::Walk, manager_types::DuplicatePolicy::KeepAll);
    /// // print [Idle, Walk]
    /// 
    /// cat_manager.push_and_delete(MyStage::Idle);
    /// // print [Idle]
    pub fn push_and_delete(&mut self, target: T) {
        let new_stages = self.calculate_dependencies(target);

        self.vector_stage = new_stages;
    }

    /// # Example of StageManager - insert() usage
    ///
    /// ```rust
    /// use purrgress::cat_stage_manager::*;
    /// # #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    /// enum MyStage { Idle, Walk }
    /// # impl manager_types::PurrStep for MyStage {}
    /// 
    /// let mut cat_manager = manager::StageManager::new();
    /// # cat_manager.add_to_graph(MyStage::Idle);
    /// # cat_manager.add_to_graph(MyStage::Walk);
    /// 
    /// # cat_manager.add_dependency(MyStage::Idle , MyStage::Walk);
    /// 
    /// cat_manager.push(MyStage::Walk, manager_types::DuplicatePolicy::KeepAll);
    /// // print [Idle, Walk]
    /// 
    /// cat_manager.insert(MyStage::Walk, manager_types::DuplicatePolicy::KeepAll, manager_types::InsertPosition::Index(1));
    /// // print [Idle, Idle, Walk, Walk]
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

    /// # Example function used in queue handling functions
    fn check_duplicate(&self, new_stages: &mut Vec<T>, duplicate_policy: DuplicatePolicy) {
        match duplicate_policy {
            DuplicatePolicy::KeepAll => (),
            DuplicatePolicy::RemoveMatch => {
                if new_stages.is_empty() || self.vector_stage.is_empty() { return; };

                if new_stages.first() == self.vector_stage.last() {
                        new_stages.remove(0);
                };
            },
        };
    }

    /// # Example function used in queue handling functions
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


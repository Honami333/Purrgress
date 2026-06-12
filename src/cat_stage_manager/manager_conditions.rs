use super::condition::*;
use super::manager_types::*;
use super::manager::*;


// A file for working with stage execution conditions in the manager

impl<T> StageManager<T> 
where 
    T: PurrStep
{
    /// # Example of StageManager - set_condition() usage
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
    /// let idle_condition = condition::PurrTimer::new(1.0);
    /// cat_manager.set_condition(MyStage::Idle, idle_condition);
    /// ```
    pub fn set_condition<C>(&mut self, stage: T, condition: C) 
    where 
        C: PurrCondition + 'static + Send + Sync
    {
        let box_condition = Box::new(condition);
        self.conditions.insert(stage, box_condition);
    }

    /// # Example of StageManager - get_condition() usage
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
    /// # let idle_condition = condition::PurrTimer::new(1.0);
    /// # cat_manager.set_condition(MyStage::Idle, idle_condition);
    /// 
    /// if let Some(idle_time) = cat_manager.get_condition::<condition::PurrTimer>(MyStage::Idle) {
    ///     println!("Длительность таймера: {}", idle_time.get_duration());
    /// };
    /// ```
    pub fn get_condition<U>(&self, stage: T) -> Option<&U>
    where 
        U: 'static
    {
        let boxed_condition = self.conditions.get(&stage)?;

        let any = boxed_condition.as_any();

        any.downcast_ref::<U>()
    }

    /// # Example of StageManager - get_condition_mut() usage
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
    /// # let idle_condition = condition::PurrTimer::new(1.0);
    /// # cat_manager.set_condition(MyStage::Idle, idle_condition);
    ///
    /// if let Some(idle_time) = cat_manager.get_condition_mut::<condition::PurrTimer>(MyStage::Idle) {
    ///     idle_time.tick(0.01);
    /// };
    /// ```
    pub fn get_condition_mut<U>(&mut self, stage: T) -> Option<&mut U>
    where 
        U: 'static
    {
        let boxed_condition  = self.conditions.get_mut(&stage)?;

        let any_mut = boxed_condition.as_any_mut();

        any_mut.downcast_mut::<U>()
    }
}
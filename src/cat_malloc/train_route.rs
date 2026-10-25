use std::collections::HashMap;

use wibr::{New, Make};

use crate::cat_depot::train_core::*;
use crate::cat_depot::train_route::RouteScheduler;
use crate::cat_depot::train_rule::PurrRule;
use crate::cat_depot::train_error::*;

use super::train_siding::PurrSiding;
use super::train_design::PurrDesign;

/// **PurrRoute** is an active list of routes that stores expanded chains of Vec
/// 
/// It is intended for efficient work with routes (More details in PurrSiding methods)
/// Each stage from PurrDesign is bound to its corresponding Vec at the end of which there is always RouteBox - of the current carriage + rule from DesignBox
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Make)]
pub struct PurrRoute<T: PurrStep, U: PurrRule> {
    /// schedule is a storage for all baked routes in the form of Vec
    /// 
    /// Direct interaction with the field for writing to it is not recommended as:
    /// You lose convenient recursive baking (Which automatically reads PurrDesign and bakes with checks for carriage presence and cycling) and duplicate policy
    /// Pub for raw interaction with schedule and writing custom logic for PurrRoute
    #[Some(HashMap::with_capacity(DEFAULT_CAPACITY))] pub schedule: HashMap<T, Vec<RouteBox<T, U>>>,
    /// visited_buffer is a buffer that is filled in the bake_carriage method via recursive traversal
    /// 
    /// Needed for reuse, stores inside itself carriages that have already been in the chain
    /// Pub for raw interaction with schedule and writing custom logic for PurrRoute
    /// 
    /// # Example
    /// Vec(TestCarriage::Component, TestCarriage::Component) by carriage TestCarriage::Pipeline - valid
    /// 
    /// Vec(TestCarriage::Component, TestCarriage::Pipeline) by carriage TestCarriage::Pipeline - not valid Err(TrainError::RecursiveCarriage { TestCarriage::Pipeline })
    #[Some(Vec::with_capacity(DEFAULT_CAPACITY))] pub visited_buffer: Vec<T>,
}

impl<T, U> PurrRoute<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    /// construct_schedule is a method for baking a template from PurrDesign
    /// Inside it performs a recursive traversal via bake_carriage, unfolding even the most unusual chains
    /// 
    /// # Parameters
    /// * `SaveMode` - whether to keep already baked routes or completely clear schedule
    /// * `DuplicatePolicy` - how to handle duplicates:
    ///   * `Overwrite` - overwrite existing entries
    ///   * `Ignore` - skip duplicate carriages
    ///   * `RaiseError` - throw an error on duplicate detection
    /// 
    /// # Errors
    /// * `TrainError::DuplicateCarriage` - if DuplicatePolicy::RaiseError is chosen and a carriage value already exists in schedule
    /// * `TrainError::RecursiveCarriage` - if during chain unfolding, a carriage appears that causes infinite recursion
    /// * `TrainError::UnscheduledParam` - if a carriage from the chain was not found inside PurrDesign
    #[inline(never)]
    pub fn construct_schedule(&mut self, purr_design: &PurrDesign<T, U>, save_mode: SaveMode, duplicate_policy: DuplicatePolicy) -> TrainResult<(), T> {
        if matches!(save_mode, SaveMode::Clear) { self.schedule.clear(); };
        for &carriage in purr_design.blueprints.keys() { if self.contains_carriage(carriage) { duplicate_policy.should_overwrite(carriage)?; }; };
        for &carriage in purr_design.blueprints.keys() {
            if self.contains_carriage(carriage) && !duplicate_policy.should_overwrite(carriage)? { continue; };
            let mut bake_buffer = Vec::with_capacity(DEFAULT_CAPACITY);
            self.visited_buffer.clear();
            self.bake_carriage(carriage, purr_design, &mut bake_buffer)?;
            self.schedule.insert(carriage, bake_buffer);
        };
        Ok(())
    }

    /// bake_carriage is a method used by construct_schedule
    /// It is public due to the public API policy, not intended for regular use
    /// 
    /// # Errors
    /// * `TrainError::RecursiveCarriage` - if during chain unfolding, a carriage appears that causes infinite recursion
    /// * `TrainError::UnscheduledParam` - if a carriage from either the argument or within the chain could not be found
    #[inline(never)]
    pub fn bake_carriage(&mut self, carriage: T, purr_design: &PurrDesign<T, U>, bake_buffer: &mut Vec<RouteBox<T, U>>) -> TrainResult<(), T> {
        if self.visited_buffer.contains(&carriage) { return Err(TrainError::RecursiveCarriage { carriage }); };

        let design_box = purr_design.blueprints.get(&carriage).ok_or(TrainError::UnscheduledParam { carriage })?;
        self.visited_buffer.push(carriage);

        if let Some(route_carriage_vec) = &design_box.coupling {
            for &route_carriage in route_carriage_vec { self.bake_carriage(route_carriage, purr_design, bake_buffer)?; };
        };

        self.visited_buffer.pop();
        let route_box = RouteBox::new(carriage, design_box.rule);
        bake_buffer.push(route_box);

        Ok(())
    }

    /// Checks for the presence of the carriage in schedule
    #[inline] pub fn contains_carriage(&self, carriage: T) -> bool { self.schedule.contains_key(&carriage) }
}

impl<T, U> RouteScheduler for PurrRoute<T, U>
where
    T: PurrStep,
    U: PurrRule
{
    type Step = T;
    type Rule = U;

    /// route_as_slice is a method for fast slicing.
    /// It was primarily conceived as a way to bypass PurrSiding, allowing for direct interaction with PurrTrain via slice methods.
    /// 
    /// # Errors
    /// * `TrainError::UnscheduledParam` - if a carriage from either the argument or within the chain could not be found
    #[inline]
    fn route_as_slice(&self, carriage: Self::Step) -> TrainResult<&[RouteBox<Self::Step, Self::Rule>], Self::Step> {
        if let Some(branch ) = self.schedule.get(&carriage) { return Ok(branch); };
        Err( TrainError::UnscheduledParam { carriage } )
    }

    #[inline]
    fn load(&self, purr_siding: &mut PurrSiding<Self::Step, Self::Rule>, carriage: Self::Step) -> TrainResult<(), Self::Step> {
        if let Some(branch ) = self.schedule.get(&carriage) {
            purr_siding.line.extend_from_slice(branch);
            return Ok(());
        };
        Err( TrainError::UnscheduledParam { carriage } )
    }
}

/// **RouteBox** is a convenient container for a carriage and its rule
/// 
/// This structure is used throughout the entire Route - Siding - Train cycle
/// It runs inside Train and all interaction happens with it during and after execution
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, New)]
pub struct RouteBox<T: PurrStep, U: PurrRule> {
    /// carriage is a specific stage connected to a concrete rule via the rule field
    /// Pub field for direct interaction with the carriage and writing custom logic for RouteBox
    pub carriage: T,
    /// rule is a specific execution rule for a carriage within the line structure of PurrTrain
    /// Pub field for direct interaction with the rule and writing custom logic for RouteBox
    pub rule: U,
}

#[cfg(test)]
mod purr_route_test {
    use crate::cat_malloc::train_design::PurrDesign;
    use crate::cat_depot::train_rule::PurrRuleLite;
    use crate::cat_depot::train_error::TrainError;
    use crate::cat_depot::train_core::{SaveMode, DuplicatePolicy};
    use crate::PurrStep;

    use super::*;

    // Test carriage enum for PurrRoute
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
    enum TestCarriage {
        Component,
        Assembly,
        Pipeline
    }

    #[test]
    fn construct_schedule_fails_on_unregistered_coupled_carriage() {
        let mut purr_route = PurrRoute::new();
        let mut purr_design = PurrDesign::new();
        let coupling = vec![TestCarriage::Component];

        purr_design.chain(TestCarriage::Pipeline, PurrRuleLite::instant(), coupling.clone());
        let result = purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default());
        assert_eq!(result, Err(TrainError::UnscheduledParam { carriage: TestCarriage::Component }));
    }

    #[test]
    fn construct_schedule_detects_circular_dependency() {
        let mut purr_route = PurrRoute::new();
        let mut purr_design = PurrDesign::new();
        let coupling = vec![TestCarriage::Component, TestCarriage::Pipeline];

        purr_design.single(TestCarriage::Component, PurrRuleLite::instant());
        purr_design.chain(TestCarriage::Pipeline, PurrRuleLite::instant(), coupling.clone());
        let result = purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default());
        assert_eq!(result, Err(TrainError::RecursiveCarriage { carriage: TestCarriage::Pipeline }));
    }

    #[test]
    fn construct_schedule_with_save_mode_keep_preserves_old_carriages() {
        let mut purr_route = PurrRoute::new();
        let mut purr_design = construct_purr_design();

        assert!(purr_route.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::default()).is_ok());

        assert!(purr_route.contains_carriage(TestCarriage::Component));
        assert!(purr_route.contains_carriage(TestCarriage::Assembly));

        purr_design.clear();
        purr_design.single(TestCarriage::Pipeline, PurrRuleLite::instant());
        assert!(purr_route.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::default()).is_ok());

        assert!(purr_route.contains_carriage(TestCarriage::Component));
        assert!(purr_route.contains_carriage(TestCarriage::Assembly));
        assert!(purr_route.contains_carriage(TestCarriage::Pipeline));
    }

    #[test]
    fn construct_schedule_with_save_mode_clear_wipes_old_carriages() {
        let mut purr_route = PurrRoute::new();
        let mut purr_design = construct_purr_design();

        assert!(purr_route.construct_schedule(&purr_design, SaveMode::Clear, DuplicatePolicy::default()).is_ok());

        assert!(purr_route.contains_carriage(TestCarriage::Component));
        assert!(purr_route.contains_carriage(TestCarriage::Assembly));

        purr_design.clear();
        purr_design.single(TestCarriage::Pipeline, PurrRuleLite::instant());
        assert!(purr_route.construct_schedule(&purr_design, SaveMode::Clear, DuplicatePolicy::default()).is_ok());

        assert!(!purr_route.contains_carriage(TestCarriage::Component));
        assert!(!purr_route.contains_carriage(TestCarriage::Assembly));
        assert!(purr_route.contains_carriage(TestCarriage::Pipeline));
    }

    #[test]
    fn construct_schedule_with_raise_error_policy_fails_on_duplicate_carriage() {
        let mut purr_route = PurrRoute::new();
        let mut purr_design = construct_purr_design();

        assert!(purr_route.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::RaiseError).is_ok());

        purr_design.clear();
        purr_design.single(TestCarriage::Component, PurrRuleLite::instant());
        let result = purr_route.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::RaiseError);
        assert_eq!(result, Err(TrainError::DuplicateCarriage { carriage: TestCarriage::Component }));
    }

    #[test]
    fn construct_schedule_with_ignore_policy_skips_duplicate_carriage() {
        let mut purr_route = PurrRoute::new();
        let mut purr_design = construct_purr_design();

        assert!(purr_route.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::Ignore).is_ok());

        assert_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant());

        purr_design.clear();
        purr_design.single(TestCarriage::Component, PurrRuleLite::flag());
        assert!(purr_route.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::Ignore).is_ok());

        assert_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant());
    }

    #[test]
    fn construct_schedule_with_overwrite_policy_replaces_duplicate_carriage() {
        let mut purr_route = PurrRoute::new();
        let mut purr_design = construct_purr_design();

        assert!(purr_route.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::Overwrite).is_ok());

        assert_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant());

        purr_design.clear();
        purr_design.single(TestCarriage::Component, PurrRuleLite::flag());
        assert!(purr_route.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::Overwrite).is_ok());

        assert_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::flag());
    }

    #[test]
    fn purr_route_contains_carriage_scenarios_validation() {
        let mut purr_route = PurrRoute::new();
        assert!(!purr_route.contains_carriage(TestCarriage::Component));
        assert!(!purr_route.contains_carriage(TestCarriage::Assembly));
        assert!(!purr_route.contains_carriage(TestCarriage::Pipeline));

        let purr_design = construct_purr_design();
        assert!(purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default()).is_ok());

        assert!(purr_route.contains_carriage(TestCarriage::Component));
        assert!(purr_route.contains_carriage(TestCarriage::Assembly));
        assert!(!purr_route.contains_carriage(TestCarriage::Pipeline));
    }

    #[test]
    fn route_as_slice_returns_correct_chain_for_coupled_carriage() {
        let mut purr_route = PurrRoute::new();
        let mut purr_design = construct_purr_design();

        let coupling = vec![TestCarriage::Component, TestCarriage::Assembly];
        purr_design.chain(TestCarriage::Pipeline, PurrRuleLite::instant(), coupling);
        assert!(purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default()).is_ok());

        let target_route = [
            RouteBox::new(TestCarriage::Component, PurrRuleLite::instant()),
            RouteBox::new(TestCarriage::Assembly, PurrRuleLite::instant()),
            RouteBox::new(TestCarriage::Pipeline, PurrRuleLite::instant())
        ];
        assert_eq!(purr_route.route_as_slice(TestCarriage::Pipeline), Ok(target_route.as_slice()));
    }

    #[test]
    fn route_as_slice_returns_err_for_unscheduled_carriage() {
        let mut purr_route = PurrRoute::new();
        let purr_design = construct_purr_design();
        assert!(purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default()).is_ok());
        assert_eq!(purr_route.route_as_slice(TestCarriage::Pipeline), Err(TrainError::UnscheduledParam { carriage: TestCarriage::Pipeline }));
    }
    
    fn construct_purr_design() -> PurrDesign<TestCarriage, PurrRuleLite> {
        let mut purr_design = PurrDesign::new();
        purr_design.single(TestCarriage::Component, PurrRuleLite::instant());
        purr_design.single(TestCarriage::Assembly, PurrRuleLite::instant());
        purr_design
    }

    fn assert_carriage_scheduled(purr_route: &PurrRoute<TestCarriage, PurrRuleLite>, carriage: TestCarriage, rule: PurrRuleLite) {
        let route_box_vec = purr_route.schedule.get(&carriage);
        assert!(route_box_vec.is_some());
        let route_box = route_box_vec.unwrap()[0];
        assert_eq!(route_box.carriage, carriage);
        assert_eq!(route_box.rule, rule);
    }
}
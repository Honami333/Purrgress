use std::collections::HashMap;
use std::collections::hash_map::Entry;

use wibr::{New, Make};

use crate::cat_depot::train_core::*;
use crate::cat_depot::train_route::RouteScheduler;
use crate::cat_depot::train_rule::PurrRule;
use crate::cat_depot::train_error::*;

use super::train_route::RouteBox;
use super::train_siding::PurrSiding;
use super::train_design::PurrDesign;

#[macro_export]
macro_rules! single_array {
    ($size:expr) => { purrgress::cat_malloc::train_route_single::SingleArray::new(purrgress::single_array_storage!($size)) };
}

#[macro_export]
macro_rules! single_array_storage {
    ($size:expr) => { [None; $size] };
}

pub trait SingleSchedule {
    type Step: PurrStep;
    type Rule: PurrRule;

    fn new_route(&mut self, route_box: RouteBox<Self::Step, Self::Rule>) -> TrainResult<(), Self::Step>;
    fn overwrite(&mut self, route_box: RouteBox<Self::Step, Self::Rule>) -> TrainResult<(), Self::Step>;
    fn get(&self, carriage: Self::Step) -> Option<RouteBox<Self::Step, Self::Rule>>;
    fn contains(&self, carriage: Self::Step) -> bool;
    fn clear(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, New)]
pub struct PurrRouteSingle<Single: SingleSchedule> {
    pub schedule: Single
}

impl<T, U> PurrRouteSingle<SingleMap<T, U>> 
where
    T: PurrStep,
    U: PurrRule
{
    pub fn with_map() -> Self { Self::new(SingleMap::new()) }
}

impl<T, U> PurrRouteSingle<SingleVec<T, U>> 
where
    T: PurrStep,
    U: PurrRule
{
    pub fn with_vec() -> Self { Self::new(SingleVec::new()) }
}

impl<Single> PurrRouteSingle<Single> where Single: SingleSchedule {
    pub fn construct_schedule(
        &mut self,
        purr_design: &PurrDesign<Single::Step, Single::Rule>,
        save_mode: SaveMode,
        duplicate_policy: DuplicatePolicy
    ) -> TrainResult<(), Single::Step> {
        if matches!(save_mode, SaveMode::Clear) { self.schedule.clear(); };
        for &carriage in purr_design.blueprints.keys() { if self.contains_carriage(carriage) { duplicate_policy.should_overwrite(carriage)?; }; };
        for (&carriage, design_box) in purr_design.blueprints.iter() {
            if self.contains_carriage(carriage) {
                if duplicate_policy.should_overwrite(carriage)? { self.schedule.overwrite(RouteBox::new(carriage, design_box.rule))?; }
            } else { self.schedule.new_route(RouteBox::new(carriage, design_box.rule))?; };
        };
        Ok(())
    }

    /// Checks for the presence of the carriage in schedule
    pub fn contains_carriage(&self, carriage: Single::Step) -> bool { self.schedule.contains(carriage) }
}

impl<Single> RouteScheduler for PurrRouteSingle<Single> where Single: SingleSchedule {
    type Step = Single::Step;
    type Rule = Single::Rule;

    fn load(&self, purr_siding: &mut PurrSiding<Self::Step, Self::Rule>, carriage: Self::Step) -> TrainResult<(), Self::Step> {
        if let Some(branch ) = self.schedule.get(carriage) {
            purr_siding.line.push(branch);
            return Ok(());
        };
        Err( TrainError::UnscheduledParam { carriage } )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Make)]
pub struct SingleMap<T: PurrStep, U: PurrRule> {
    #[Some(HashMap::with_capacity(DEFAULT_CAPACITY))] pub storage: HashMap<T, RouteBox<T, U>>
}

impl<T, U> SingleSchedule for SingleMap<T, U>
where
    T: PurrStep,
    U: PurrRule
{
    type Step = T;
    type Rule = U;

    fn new_route(&mut self, route_box: RouteBox<Self::Step, Self::Rule>) -> TrainResult<(), Self::Step> {
        match self.storage.entry(route_box.carriage) {
            Entry::Occupied(_) => return Err(TrainError::DuplicateCarriage { carriage: route_box.carriage }),
            Entry::Vacant(v) => v.insert(route_box)
        };
        Ok(())
    }

    fn overwrite(&mut self, route_box: RouteBox<Self::Step, Self::Rule>) -> TrainResult<(), Self::Step> {
        self.storage.insert(route_box.carriage, route_box);
        Ok(())
    }

    fn get(&self, carriage: Self::Step) -> Option<RouteBox<Self::Step, Self::Rule>> { self.storage.get(&carriage).copied() }
    fn contains(&self, carriage: Self::Step) -> bool { self.storage.contains_key(&carriage) }
    fn clear(&mut self) { self.storage.clear(); }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Make)]
pub struct SingleVec<T: PurrStep, U: PurrRule> {
    #[Some(Vec::with_capacity(DEFAULT_CAPACITY))] pub storage: Vec<RouteBox<T, U>>
}

impl<T, U> SingleSchedule for SingleVec<T, U>
where
    T: PurrStep,
    U: PurrRule
{
    type Step = T;
    type Rule = U;

    fn new_route(&mut self, route_box: RouteBox<Self::Step, Self::Rule>) -> TrainResult<(), Self::Step> {
        self.storage.push(route_box);
        Ok(())
    }

    fn overwrite(&mut self, route_box: RouteBox<Self::Step, Self::Rule>) -> TrainResult<(), Self::Step> {
        if let Some(index) = self.storage.iter().position(|r| r.carriage == route_box.carriage) {
            self.storage[index] = route_box;
            return Ok(());
        };
        self.new_route(route_box)
    }

    fn get(&self, carriage: Self::Step) -> Option<RouteBox<Self::Step, Self::Rule>> { self.storage.iter().find(|r| r.carriage == carriage).copied() }
    fn contains(&self, carriage: Self::Step) -> bool { self.storage.iter().any(|r| r.carriage == carriage) }
    fn clear(&mut self) { self.storage.clear(); }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, New)]
pub struct SingleArray<T: PurrStep, U: PurrRule, const N: usize> {
    pub storage: [Option<RouteBox<T, U>>; N]
}

impl<T, U, const N: usize> SingleSchedule for SingleArray<T, U, N>
where
    T: PurrStep,
    U: PurrRule
{
    type Step = T;
    type Rule = U;

    fn new_route(&mut self, route_box: RouteBox<Self::Step, Self::Rule>) -> TrainResult<(), Self::Step> {
        if let Some(index) = self.storage.iter().position(|r| r.is_none()) {
            self.storage[index] = Some(route_box);
            return Ok(());
        };
        Err(TrainError::StorageOverflow { capacity: self.storage.len() })
    }

    fn overwrite(&mut self, route_box: RouteBox<Self::Step, Self::Rule>) -> TrainResult<(), Self::Step> {
        if let Some(index) = self.storage.iter().position(|r| r.is_some_and(|r| r.carriage == route_box.carriage)) {
            self.storage[index] = Some(route_box);
            return Ok(());
        };
        self.new_route(route_box)
    }

    fn get(&self, carriage: Self::Step) -> Option<RouteBox<Self::Step, Self::Rule>> {
        self.storage.iter().flatten().find(|r| r.carriage == carriage).copied()
    }

    fn contains(&self, carriage: Self::Step) -> bool { self.storage.iter().flatten().any(|r| r.carriage == carriage) }
    fn clear(&mut self) { self.storage = single_array_storage!(N) }
}

#[cfg(test)]
mod purr_route_single_test {
    use purrgress_macros::PurrStep;

    use crate::cat_depot::train_rule::PurrRuleLite;
    use crate::cat_depot::train_core::{SaveMode, DuplicatePolicy};
    use crate::cat_malloc::train_route::PurrRoute;

    use super::*;

    // Test carriage enum for PurrRouteSingle
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
    enum TestCarriage {
        Component,
        Assembly,
        Pipeline
    }

    #[test]
    fn chain_adds_only_carriages_without_couplings() {
        let mut purr_route = PurrRouteSingle::with_vec();
        let mut purr_design = construct_purr_design();
        purr_design.chain(TestCarriage::Pipeline, PurrRuleLite::instant(), vec![TestCarriage::Component, TestCarriage::Assembly]);

        assert!(purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default()).is_ok());
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Assembly, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Pipeline, PurrRuleLite::instant()));
    }

    #[test]
    fn clear_mode_drops_old_carriages() {
        let mut purr_route = PurrRouteSingle::with_vec();
        let purr_design = construct_purr_design();

        assert!(purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default()).is_ok());
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Assembly, PurrRuleLite::instant()));
        assert!(!is_carriage_scheduled(&purr_route, TestCarriage::Pipeline, PurrRuleLite::instant()));

        let mut purr_design = PurrDesign::new();
        purr_design.single(TestCarriage::Pipeline, PurrRuleLite::instant());
        assert!(purr_route.construct_schedule(&purr_design, SaveMode::Clear, DuplicatePolicy::default()).is_ok());
        assert!(!is_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant()));
        assert!(!is_carriage_scheduled(&purr_route, TestCarriage::Assembly, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Pipeline, PurrRuleLite::instant()));
    }

    #[test]
    fn keep_mode_preserves_old_carriages() {
        let mut purr_route = PurrRouteSingle::with_vec();
        let purr_design = construct_purr_design();

        assert!(purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default()).is_ok());
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Assembly, PurrRuleLite::instant()));
        assert!(!is_carriage_scheduled(&purr_route, TestCarriage::Pipeline, PurrRuleLite::instant()));

        let mut purr_design = PurrDesign::new();
        purr_design.single(TestCarriage::Pipeline, PurrRuleLite::instant());
        assert!(purr_route.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::default()).is_ok());
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Assembly, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Pipeline, PurrRuleLite::instant()));
    }

    #[test]
    fn overwrite_policy_replaces_all_duplicates() {
        let mut purr_route = PurrRouteSingle::with_vec();
        let mut purr_design = construct_purr_design();

        assert!(purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default()).is_ok());
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Assembly, PurrRuleLite::instant()));

        purr_design.single(TestCarriage::Component, PurrRuleLite::flag());
        assert!(purr_route.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::Overwrite).is_ok());
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::flag()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Assembly, PurrRuleLite::instant()));
    }

    #[test]
    fn ignore_skips_every_duplicate() {
        let mut purr_route = PurrRouteSingle::with_vec();
        let mut purr_design = construct_purr_design();

        assert!(purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default()).is_ok());
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Assembly, PurrRuleLite::instant()));

        purr_design.single(TestCarriage::Component, PurrRuleLite::flag());
        assert!(purr_route.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::Ignore).is_ok());
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Assembly, PurrRuleLite::instant()));
    }

    #[test]
    fn raise_error_policy_fails_on_duplicate() {
        let mut purr_route = PurrRouteSingle::with_vec();
        let purr_design = construct_purr_design();

        assert!(purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default()).is_ok());
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Assembly, PurrRuleLite::instant()));

        let mut purr_design = PurrDesign::new();
        purr_design.single(TestCarriage::Component, PurrRuleLite::flag());
        assert_eq!(purr_route.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::RaiseError), Err(TrainError::DuplicateCarriage { carriage: TestCarriage::Component }));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Assembly, PurrRuleLite::instant()));
    }

    #[test]
    fn launch_results_equal_across_route_kinds() {
        let mut purr_route = PurrRouteSingle::with_vec();
        let purr_design = construct_purr_design();
        let mut purr_siding_single = PurrSiding::new();

        assert!(purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default()).is_ok());
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Assembly, PurrRuleLite::instant()));

        assert!(purr_siding_single.launch(TestCarriage::Component, SaveMode::Keep, &purr_route).is_ok());
        assert!(purr_siding_single.launch(TestCarriage::Assembly, SaveMode::Keep, &purr_route).is_ok());

        let mut purr_route = PurrRoute::new();
        assert!(purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default()).is_ok());
        let mut purr_siding = PurrSiding::new();

        assert!(purr_siding.launch(TestCarriage::Component, SaveMode::Keep, &purr_route).is_ok());
        assert!(purr_siding.launch(TestCarriage::Assembly, SaveMode::Keep, &purr_route).is_ok());
        assert_eq!(purr_siding, purr_siding_single);
    }

    #[test]
    fn launch_unscheduled_carriage_fails() {
        let mut purr_route = PurrRouteSingle::with_vec();
        let purr_design = construct_purr_design();
        let mut purr_siding = PurrSiding::new();

        assert!(purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default()).is_ok());
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Component, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route, TestCarriage::Assembly, PurrRuleLite::instant()));
        assert!(!is_carriage_scheduled(&purr_route, TestCarriage::Pipeline, PurrRuleLite::instant()));

        assert_eq!(purr_siding.launch(TestCarriage::Pipeline, SaveMode::default(), &purr_route), Err(TrainError::UnscheduledParam { carriage: TestCarriage::Pipeline }));
    }

    #[test]
    fn storage_kind_does_not_affect_route_behavior() {
        let mut purr_route_map = PurrRouteSingle::with_map();
        let mut purr_route_vec = PurrRouteSingle::with_vec();

        let mut purr_design = construct_purr_design();

        assert!(purr_route_map.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default()).is_ok());
        assert!(purr_route_vec.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default()).is_ok());

        assert!(is_carriage_scheduled(&purr_route_map, TestCarriage::Component, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route_map, TestCarriage::Assembly, PurrRuleLite::instant()));

        assert!(is_carriage_scheduled(&purr_route_vec, TestCarriage::Component, PurrRuleLite::instant()));
        assert!(is_carriage_scheduled(&purr_route_vec, TestCarriage::Assembly, PurrRuleLite::instant()));

        purr_design.single(TestCarriage::Component, PurrRuleLite::flag());

        assert!(purr_route_map.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::Overwrite).is_ok());
        assert!(purr_route_vec.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::Overwrite).is_ok());

        assert!(is_carriage_scheduled(&purr_route_map, TestCarriage::Component, PurrRuleLite::flag()));
        assert!(is_carriage_scheduled(&purr_route_map, TestCarriage::Assembly, PurrRuleLite::instant()));

        assert!(is_carriage_scheduled(&purr_route_vec, TestCarriage::Component, PurrRuleLite::flag()));
        assert!(is_carriage_scheduled(&purr_route_vec, TestCarriage::Assembly, PurrRuleLite::instant()));

        let mut purr_design = PurrDesign::new();
        purr_design.single(TestCarriage::Component, PurrRuleLite::flag());

        assert_eq!(purr_route_map.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::RaiseError), Err(TrainError::DuplicateCarriage { carriage: TestCarriage::Component }));
        assert_eq!(purr_route_vec.construct_schedule(&purr_design, SaveMode::Keep, DuplicatePolicy::RaiseError), Err(TrainError::DuplicateCarriage { carriage: TestCarriage::Component }));

        assert!(is_carriage_scheduled(&purr_route_map, TestCarriage::Component, PurrRuleLite::flag()));
        assert!(is_carriage_scheduled(&purr_route_map, TestCarriage::Assembly, PurrRuleLite::instant()));

        assert!(is_carriage_scheduled(&purr_route_vec, TestCarriage::Component, PurrRuleLite::flag()));
        assert!(is_carriage_scheduled(&purr_route_vec, TestCarriage::Assembly, PurrRuleLite::instant()));
    }

    fn construct_purr_design() -> PurrDesign<TestCarriage, PurrRuleLite> {
        let mut purr_design = PurrDesign::new();
        purr_design.single(TestCarriage::Component, PurrRuleLite::instant());
        purr_design.single(TestCarriage::Assembly, PurrRuleLite::instant());
        purr_design
    }

    fn is_carriage_scheduled<Single>(purr_route: &PurrRouteSingle<Single>, carriage: TestCarriage, rule: PurrRuleLite) -> bool
    where
        Single: SingleSchedule<Step = TestCarriage, Rule = PurrRuleLite>
    {
        purr_route.schedule.get(carriage) == Some(RouteBox::new(carriage, rule))
    }
}

#[cfg(test)]
mod single_map_test {
    use purrgress_macros::PurrStep;

    use crate::cat_depot::train_rule::PurrRuleLite;

    use super::*;

    // Test carriage enum for SingleMap
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
    enum TestCarriage {
        Component,
        Assembly
    }

    #[test]
    fn overwrite_inserts_or_replaces_without_duplicates() {
        let mut map = SingleMap::new();
        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(map.overwrite(route_box).is_ok());
        assert_eq!(map.get(TestCarriage::Component), Some(route_box));
        assert_eq!(map.storage.len(), 1);

        let new_route_box1 = RouteBox::new(TestCarriage::Component, PurrRuleLite::flag());
        assert!(map.overwrite(new_route_box1).is_ok());
        assert_eq!(map.get(TestCarriage::Component), Some(new_route_box1));
        assert_eq!(map.storage.len(), 1);

        let new_route_box2 = RouteBox::new(TestCarriage::Assembly, PurrRuleLite::instant());
        assert!(map.overwrite(new_route_box2).is_ok());
        assert_eq!(map.get(TestCarriage::Component), Some(new_route_box1));
        assert_eq!(map.get(TestCarriage::Assembly), Some(new_route_box2));
        assert_eq!(map.storage.len(), 2);
    }

    #[test]
    fn empty_map_has_no_carriages() {
        let mut map = SingleMap::new();
        assert!(map.get(TestCarriage::Component).is_none());
        assert!(!map.contains(TestCarriage::Component));

        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(map.new_route(route_box).is_ok());
        assert_eq!(map.get(TestCarriage::Component), Some(route_box));
        assert!(map.contains(TestCarriage::Component));
    }

    #[test]
    fn new_route_accepts_different_carriages() {
        let mut map = SingleMap::new();
        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(map.new_route(route_box).is_ok());
        let new_route_box = RouteBox::new(TestCarriage::Assembly, PurrRuleLite::instant());
        assert!(map.new_route(new_route_box).is_ok());

        assert_eq!(map.get(TestCarriage::Component), Some(route_box));
        assert_eq!(map.get(TestCarriage::Assembly), Some(new_route_box));
    }

    #[test]
    fn duplicate_new_route_errors_and_keeps_old() {
        let mut map = SingleMap::new();
        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(map.new_route(route_box).is_ok());
        assert_eq!(map.get(TestCarriage::Component), Some(route_box));

        let new_route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::flag());
        assert_eq!(map.new_route(new_route_box), Err(TrainError::DuplicateCarriage { carriage: TestCarriage::Component }));
        assert_eq!(map.get(TestCarriage::Component), Some(route_box));

        assert!(map.overwrite(new_route_box).is_ok());
        assert_eq!(map.get(TestCarriage::Component), Some(new_route_box));
    }

    #[test]
    fn clear_removes_entries_without_side_effects() {
        let mut map = SingleMap::new();
        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(map.new_route(route_box).is_ok());

        let mut clear_map = map.clone();
        clear_map.clear();

        assert_eq!(map.get(TestCarriage::Component), Some(route_box));
        assert!(map.contains(TestCarriage::Component));
        assert!(clear_map.get(TestCarriage::Component).is_none());
        assert!(!clear_map.contains(TestCarriage::Component));

        assert!(clear_map.new_route(route_box).is_ok());
        assert_eq!(map, clear_map);
    }
}

#[cfg(test)]
mod single_vector_test {
    use purrgress_macros::PurrStep;

    use crate::cat_depot::train_rule::PurrRuleLite;

    use super::*;

    // Test carriage enum for SingleVec
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
    enum TestCarriage {
        Component,
        Assembly
    }

    #[test]
    fn overwrite_inserts_or_replaces_without_duplicates() {
        let mut vec = SingleVec::new();
        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(vec.overwrite(route_box).is_ok());
        assert_eq!(vec.storage[0], route_box);
        assert_eq!(vec.storage.get(1), None);

        let new_route_box1 = RouteBox::new(TestCarriage::Component, PurrRuleLite::flag());
        assert!(vec.overwrite(new_route_box1).is_ok());
        assert_eq!(vec.storage[0], new_route_box1);
        assert_eq!(vec.storage.get(1), None);

        let new_route_box2 = RouteBox::new(TestCarriage::Assembly, PurrRuleLite::instant());
        assert!(vec.overwrite(new_route_box2).is_ok());
        assert_eq!(vec.storage[0], new_route_box1);
        assert_eq!(vec.storage[1], new_route_box2);
    }

    #[test]
    fn empty_vec_has_no_carriages() {
        let mut vec = SingleVec::new();
        assert!(vec.get(TestCarriage::Component).is_none());
        assert!(!vec.contains(TestCarriage::Component));

        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(vec.new_route(route_box).is_ok());
        assert_eq!(vec.storage[0], route_box);

        assert_eq!(vec.get(TestCarriage::Component), Some(route_box));
        assert!(vec.contains(TestCarriage::Component));
    }

    #[test]
    fn new_route_accepts_different_carriages() {
        let mut vec = SingleVec::new();
        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(vec.new_route(route_box).is_ok());
        let new_route_box = RouteBox::new(TestCarriage::Assembly, PurrRuleLite::instant());
        assert!(vec.new_route(new_route_box).is_ok());

        assert_eq!(vec.get(TestCarriage::Component), Some(route_box));
        assert_eq!(vec.get(TestCarriage::Assembly), Some(new_route_box));
    }

    #[test]
    fn new_route_appends_duplicates_order_decides_lookup() {
        let mut vec = SingleVec::new();
        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(vec.new_route(route_box).is_ok());
        let new_route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::flag());
        assert!(vec.new_route(new_route_box).is_ok());

        assert_eq!(vec.get(TestCarriage::Component), Some(route_box));
        assert_eq!(vec.storage[0], route_box);
        assert_eq!(vec.storage[1], new_route_box);
    }

    #[test]
    fn clear_removes_entries_without_side_effects() {
        let mut vec = SingleVec::new();
        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(vec.new_route(route_box).is_ok());

        let mut clear_vec = vec.clone();
        clear_vec.clear();

        assert_eq!(vec.get(TestCarriage::Component), Some(route_box));
        assert!(vec.contains(TestCarriage::Component));
        assert!(clear_vec.get(TestCarriage::Component).is_none());
        assert!(!clear_vec.contains(TestCarriage::Component));

        assert!(clear_vec.new_route(route_box).is_ok());
        assert_eq!(vec, clear_vec);
    }
}

#[cfg(test)]
mod single_array_test {
    use purrgress_macros::PurrStep;

    use crate::cat_depot::train_rule::PurrRuleLite;

    use super::*;

    // Test carriage enum for SingleArray
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
    enum TestCarriage {
        Component,
        Assembly
    }

    #[test]
    fn new_route_on_full_array_overflows_keeps_old() {
        let mut array = single_array!(1);
        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(array.new_route(route_box).is_ok());
        assert_eq!(array.storage[0], Some(route_box));

        let new_route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert_eq!(array.new_route(new_route_box), Err(TrainError::StorageOverflow { capacity: 1 }));
        assert_eq!(array.storage[0], Some(route_box));
    }

    #[test]
    fn overwrite_replaces_existing_overflows_on_new_when_full() {
        let mut array = single_array!(1);
        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(array.overwrite(route_box).is_ok());
        assert_eq!(array.storage[0], Some(route_box));

        let new_route_box1 = RouteBox::new(TestCarriage::Component, PurrRuleLite::flag());
        assert!(array.overwrite(new_route_box1).is_ok());
        assert_eq!(array.storage[0], Some(new_route_box1));

        let new_route_box2 = RouteBox::new(TestCarriage::Assembly, PurrRuleLite::instant());
        assert_eq!(array.overwrite(new_route_box2), Err(TrainError::StorageOverflow { capacity: 1 }));
        assert_eq!(array.storage[0], Some(new_route_box1));
    }

    #[test]
    fn empty_array_has_no_carriages() {
        let mut array = single_array!(1);
        assert!(array.get(TestCarriage::Component).is_none());
        assert!(!array.contains(TestCarriage::Component));

        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(array.new_route(route_box).is_ok());
        assert_eq!(array.storage[0], Some(route_box));

        assert_eq!(array.get(TestCarriage::Component), Some(route_box));
        assert!(array.contains(TestCarriage::Component));
    }

    #[test]
    fn new_route_accepts_different_carriages() {
        let mut array = single_array!(2);
        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(array.new_route(route_box).is_ok());
        let new_route_box = RouteBox::new(TestCarriage::Assembly, PurrRuleLite::instant());
        assert!(array.new_route(new_route_box).is_ok());

        assert_eq!(array.get(TestCarriage::Component), Some(route_box));
        assert_eq!(array.get(TestCarriage::Assembly), Some(new_route_box));
    }

    #[test]
    fn new_route_appends_duplicates_order_decides_lookup() {
        let mut array = single_array!(2);
        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(array.new_route(route_box).is_ok());
        let new_route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::flag());
        assert!(array.new_route(new_route_box).is_ok());

        assert_eq!(array.get(TestCarriage::Component), Some(route_box));
        assert_eq!(array.storage[0], Some(route_box));
        assert_eq!(array.storage[1], Some(new_route_box));
    }

    #[test]
    fn clear_removes_entries_without_side_effects() {
        let mut array = single_array!(1);
        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert!(array.new_route(route_box).is_ok());

        let mut clear_array = array;
        clear_array.clear();

        assert_eq!(array.get(TestCarriage::Component), Some(route_box));
        assert!(array.contains(TestCarriage::Component));
        assert!(clear_array.get(TestCarriage::Component).is_none());
        assert!(!clear_array.contains(TestCarriage::Component));

        assert!(clear_array.new_route(route_box).is_ok());
        assert_eq!(array, clear_array);
    }

    #[test]
    fn zero_capacity_array_overflows_on_any_insert() {
        let mut array = single_array!(0);
        let route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        assert_eq!(array.new_route(route_box), Err(TrainError::StorageOverflow { capacity: 0 }));
        assert_eq!(array.overwrite(route_box), Err(TrainError::StorageOverflow { capacity: 0 }));
    }
}
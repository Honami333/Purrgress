use wibr::Make;

use crate::cat_depot::train_core::*;
use crate::cat_depot::train_route::RouteScheduler;
use crate::cat_depot::train_rule::PurrRule;
use crate::cat_depot::train_error::*;

use super::train_route::RouteBox;

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Make)]
pub struct PurrSiding<T: PurrStep, U: PurrRule> {
    #[Some(Vec::with_capacity(DEFAULT_CAPACITY))] pub line: Vec<RouteBox<T, U>>,
}

impl<T, U> PurrSiding<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    pub fn launch(&mut self, carriage: T, save_mode: SaveMode, route_scheduler: &impl RouteScheduler<Step = T, Rule = U>) -> TrainResult<(), T> {
        if matches!(save_mode, SaveMode::Clear) { self.clear_line(); };
        route_scheduler.load(self, carriage)
    }

    pub fn change_rule(&mut self, carriage: T, rule: U, mut change_rule_count: ChangeRuleCount) {
        for route_box in self.line.iter_mut() {
            if matches!(change_rule_count, ChangeRuleCount::Count(0)) { return; };
            if carriage == route_box.carriage {
                route_box.rule = rule;
                if change_rule_count.is_final_rule_count() { return; };
            };
        };
    }

    pub fn change_rule_few(&mut self, route_boxes: &[RouteBox<T, U>], change_rule_count: ChangeRuleCount) {
        for &route_box in route_boxes.iter() {
            let mut route_rule_count = change_rule_count;
            for main_route_box in self.line.iter_mut() {
                if matches!(route_rule_count, ChangeRuleCount::Count(0)) { break; };
                if main_route_box.carriage == route_box.carriage {
                    main_route_box.rule = route_box.rule;
                    if route_rule_count.is_final_rule_count() { break; };
                };
            };
        };
    }
    
    pub fn line_is_empty(&self) -> bool { self.line.is_empty() }
    pub fn line_len(&self) -> usize { self.line.len() }
    pub fn clear_line(&mut self) { self.line.clear(); }
}

#[cfg(test)]
mod purr_siding_test {
    use crate::cat_malloc::train_design::PurrDesign;
    use crate::cat_malloc::train_route::PurrRoute;
    use crate::cat_depot::train_rule::PurrRuleLite;
    use crate::cat_depot::train_error::TrainError;
    use crate::cat_depot::train_core::{SaveMode, DuplicatePolicy};
    use crate::PurrStep;

    use super::*;

    // Test carriage enum for PurrSiding
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
    enum TestCarriage {
        Component,
        Assembly,
        Pipeline
    }
    
    #[test]
    fn successful_launch() {
        let mut purr_siding = PurrSiding::new();
        let purr_route = construct_purr_route(true);
        attach(&mut purr_siding, &purr_route, TestCarriage::Pipeline, SaveMode::default());
    }

    #[test]
    fn launch_unscheduled_carriage() {
        let mut purr_siding = PurrSiding::new();
        let purr_route = construct_purr_route(false);
        let result = purr_siding.launch(TestCarriage::Pipeline, SaveMode::default(), &purr_route);
        assert_eq!(result, Err(TrainError::UnscheduledParam { carriage: TestCarriage::Pipeline }));
    }

    #[test]
    fn change_rule_first_replaces_first_match() {
        let mut purr_siding = PurrSiding::new();
        let purr_route = construct_purr_route(true);
        attach(&mut purr_siding, &purr_route, TestCarriage::Component, SaveMode::default());

        let route_box = purr_siding.line[0];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Component, PurrRuleLite::instant()));

        purr_siding.change_rule(TestCarriage::Component, PurrRuleLite::flag(), ChangeRuleCount::First);
        let route_box = purr_siding.line[0];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Component, PurrRuleLite::flag()));
    }

    #[test]
    fn change_rule_count_and_all() {
        let mut purr_siding = PurrSiding::new();
        let purr_route = construct_purr_route(true);
        attach(&mut purr_siding, &purr_route, TestCarriage::Component, SaveMode::Keep);
        attach(&mut purr_siding, &purr_route, TestCarriage::Component, SaveMode::Keep);
        attach(&mut purr_siding, &purr_route, TestCarriage::Component, SaveMode::Keep);
        
        let route_box = purr_siding.line[0];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Component, PurrRuleLite::instant()));

        purr_siding.change_rule(TestCarriage::Component, PurrRuleLite::flag(), ChangeRuleCount::Count(1));
        let route_box = purr_siding.line[0];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Component, PurrRuleLite::flag()));
        let route_box = purr_siding.line[1];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Component, PurrRuleLite::instant()));
        let route_box = purr_siding.line[2];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Component, PurrRuleLite::instant()));

        purr_siding.change_rule(TestCarriage::Component, PurrRuleLite::flag(), ChangeRuleCount::All);
        let route_box = purr_siding.line[0];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Component, PurrRuleLite::flag()));
        let route_box = purr_siding.line[1];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Component, PurrRuleLite::flag()));
        let route_box = purr_siding.line[2];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Component, PurrRuleLite::flag()));
    }

    #[test]
    fn change_few_rules_first() {
        let mut purr_siding = PurrSiding::new();
        let purr_route = construct_purr_route(true);
        attach(&mut purr_siding, &purr_route, TestCarriage::Pipeline, SaveMode::Keep);
        attach(&mut purr_siding, &purr_route, TestCarriage::Component, SaveMode::Keep);

        let route_boxes = [
            RouteBox::new(TestCarriage::Component, PurrRuleLite::flag()),
            RouteBox::new(TestCarriage::Assembly, PurrRuleLite::flag()),
            RouteBox::new(TestCarriage::Pipeline, PurrRuleLite::timer(2.0))
        ];
        purr_siding.change_rule_few(&route_boxes, ChangeRuleCount::First);

        let route_box = purr_siding.line[0];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Component, PurrRuleLite::flag()));
        let route_box = purr_siding.line[1];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Assembly, PurrRuleLite::flag()));
        let route_box = purr_siding.line[2];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Pipeline, PurrRuleLite::timer(2.0)));

        let route_box = purr_siding.line[3];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Component, PurrRuleLite::instant()));
    }

    #[test]
    fn change_few_rules_all() {
        let mut purr_siding = PurrSiding::new();
        let purr_route = construct_purr_route(true);
        attach(&mut purr_siding, &purr_route, TestCarriage::Pipeline, SaveMode::Keep);
        attach(&mut purr_siding, &purr_route, TestCarriage::Component, SaveMode::Keep);

        let route_boxes = [
            RouteBox::new(TestCarriage::Component, PurrRuleLite::flag()),
            RouteBox::new(TestCarriage::Assembly, PurrRuleLite::flag()),
            RouteBox::new(TestCarriage::Pipeline, PurrRuleLite::timer(2.0))
        ];
        purr_siding.change_rule_few(&route_boxes, ChangeRuleCount::All);

        let route_box = purr_siding.line[0];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Component, PurrRuleLite::flag()));
        let route_box = purr_siding.line[1];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Assembly, PurrRuleLite::flag()));
        let route_box = purr_siding.line[2];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Pipeline, PurrRuleLite::timer(2.0)));

        let route_box = purr_siding.line[3];
        assert_eq!(route_box, RouteBox::new(TestCarriage::Component, PurrRuleLite::flag()));
    }

    #[test]
    fn verify_line_is_empty() {
        let mut purr_siding = PurrSiding::new();
        assert!(purr_siding.line_is_empty());
        let purr_route = construct_purr_route(true);
        attach(&mut purr_siding, &purr_route, TestCarriage::Pipeline, SaveMode::default());
        assert!(!purr_siding.line_is_empty());
    }

    #[test]
    fn verify_clear_line() {
        let mut purr_siding = PurrSiding::new();
        let purr_route = construct_purr_route(true);
        attach(&mut purr_siding, &purr_route, TestCarriage::Pipeline, SaveMode::default());
        assert!(!purr_siding.line_is_empty());
        purr_siding.clear_line();
        assert!(purr_siding.line_is_empty());
    }

    fn attach(purr_siding: &mut PurrSiding<TestCarriage, PurrRuleLite>, purr_route: &PurrRoute<TestCarriage, PurrRuleLite>, carriage: TestCarriage, save_mode: SaveMode) {
        let result = purr_siding.launch(carriage, save_mode, purr_route);
        assert!(result.is_ok());
    }

    fn construct_purr_route(construct: bool) -> PurrRoute<TestCarriage, PurrRuleLite> {
        let mut purr_route = PurrRoute::new();
        if construct {
            let mut purr_design = PurrDesign::new();
            purr_design.single(TestCarriage::Component, PurrRuleLite::instant());
            purr_design.single(TestCarriage::Assembly, PurrRuleLite::instant());
            purr_design.chain(TestCarriage::Pipeline, PurrRuleLite::instant(), vec![TestCarriage::Component, TestCarriage::Assembly]);
            let result = purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default());
            assert!(result.is_ok());
        };
        purr_route
    }
}
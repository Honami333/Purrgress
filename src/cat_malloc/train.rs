use wibr::New;

use crate::cat_depot::train_error::TrainResult;
use crate::cat_depot::train_track::PurrTrack;
use crate::cat_depot::train_rule::PurrRule;
use crate::cat_depot::train_core::*;

#[cfg(feature = "cursor")]
use crate::cat_depot::train_cursor::PurrCursor;

use super::train_route::RouteBox;
use super::train_siding::PurrSiding;

#[cfg(feature = "cursor")]
pub type CursorTrain<T, U> = PurrTrain<PurrCursor<RouteBox<T, U>>>;

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, New)]
pub struct PurrTrain<S> {
    pub line: S
}

#[cfg(feature = "cursor")]
impl<T, U> CursorTrain<T, U> where T: PurrStep, U: PurrRule {
    #[inline] pub fn with_cursor() -> Self { Self::new(PurrCursor::tr_new()) }
}

impl<S> PurrTrain<S> where S: PurrTrack {
    #[inline] pub fn get_current(&self) -> Option<RouteBox<S::Step, S::Rule>> { self.line.tr_get_current() }

    #[inline] pub fn get_current_ref(&self) -> Option<&RouteBox<S::Step, S::Rule>> { self.line.tr_get_current_ref() }

    #[inline] pub fn get_current_mut(&mut self) -> Option<&mut RouteBox<S::Step, S::Rule>> { self.line.tr_get_current_mut() }

    #[inline] pub fn get_cursor(&self) -> usize { self.line.tr_get_cursor() }
    
    #[inline] pub fn line_as_slice(&self) -> &[RouteBox<S::Step, S::Rule>] { self.line.tr_as_slice() }

    #[inline] pub fn line_as_mut_slice(&mut self) -> &mut [RouteBox<S::Step, S::Rule>] { self.line.tr_as_mut_slice() }
    
    #[inline] pub fn line_is_empty(&self) -> bool { self.line.tr_is_empty() }

    #[inline] pub fn line_len(&self) -> usize { self.line.tr_len() }

    #[inline] pub fn clear_line(&mut self) { self.line.tr_clear(); }

    #[inline] pub fn attach(&mut self, purr_siding: &mut PurrSiding<S::Step, S::Rule>) -> TrainResult<(), S::Step> { self.line.tr_extend(purr_siding.line.drain(..)) }

    #[inline] pub fn attach_slice(&mut self, slice: &[RouteBox<S::Step, S::Rule>]) -> TrainResult<(), S::Step> { self.line.tr_extend(slice.iter().copied()) }

    #[inline]
    pub fn replace(&mut self, purr_siding: &mut PurrSiding<S::Step, S::Rule>) -> TrainResult<(), S::Step> {
        self.clear_line();
        self.attach(purr_siding)
    }

    #[inline]
    pub fn replace_slice(&mut self, slice: &[RouteBox<S::Step, S::Rule>]) -> TrainResult<(), S::Step> {
        self.clear_line();
        self.attach_slice(slice)
    }

    #[inline]
    pub fn reroute_at(&mut self, purr_siding: &mut PurrSiding<S::Step, S::Rule>, insert_position: InsertPosition) -> TrainResult<(), S::Step> {
        let cursor = self.line.tr_get_cursor();
        let index = insert_position.get(cursor, self.line_len());
        drop(self.line.tr_splice(index..index, purr_siding.line.drain(..))?);
        Ok(())
    }

    #[inline]
    pub fn reroute_at_slice(&mut self, slice: &[RouteBox<S::Step, S::Rule>], insert_position: InsertPosition) -> TrainResult<(), S::Step> {
        let cursor = self.line.tr_get_cursor();
        let index = insert_position.get(cursor, self.line_len());
        drop(self.line.tr_splice(index..index, slice.iter().copied())?);
        Ok(())
    }

    #[inline]
    pub fn shrink_line(&mut self, line_length: usize) {
        let cursor = self.line.tr_get_cursor();
        if cursor > line_length { drop(self.line.tr_drain(0..cursor)); };
    }

    #[inline]
    pub fn advance_train(&mut self) -> PurrTrainEvent<S::Step, S::Rule> {
        if let Some(route_box) = self.get_current() {
            if route_box.rule.is_finished() {
                if self.line.tr_is_last() {
                    self.clear_line();
                    return PurrTrainEvent::Transition { from: route_box, to: None };
                };
                let stepped = self.line.tr_step_cursor();
                debug_assert!(stepped.is_ok());
                let to_route_box = self.get_current();
                return PurrTrainEvent::Transition { from: route_box, to: to_route_box };
            };
            return PurrTrainEvent::Running(route_box)
        };
        PurrTrainEvent::Idle
    }
}

#[cfg(test)]
mod purr_train_test {
    use crate::cat_depot::train_conditions::PurrFlag;
    use crate::cat_malloc::train_design::PurrDesign;
    use crate::cat_malloc::train_route::PurrRoute;
    use crate::cat_malloc::train_siding::PurrSiding;
    use crate::cat_depot::train_rule::PurrRuleLite;
    use crate::cat_depot::train_core::{SaveMode, DuplicatePolicy};
    use crate::PurrStep;

    use super::*;

    // Test carriage enum for PurrTrain
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
    enum TestCarriage {
        Component,
        Assembly,
        Pipeline
    }

    #[test]
    fn attach_to_empty_train() {
        let mut purr_train = PurrTrain::with_cursor();
        assert!(purr_train.line_is_empty());

        let mut purr_siding = construct_purr_siding();
        assert!(purr_train.attach(&mut purr_siding).is_ok());
        len_assert(&purr_train, 3);
        line_assert(&purr_train);
    }

    #[test]
    fn replace_overwrites_existing() {
        let mut purr_train = PurrTrain::with_cursor();
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train.attach(&mut purr_siding).is_ok());
        len_assert(&purr_train, 3);
        line_assert(&purr_train);
        purr_siding = construct_purr_siding();
        assert!(purr_train.attach(&mut purr_siding).is_ok());
        len_assert(&purr_train, 6);
        line_assert(&purr_train);
        purr_siding = construct_purr_siding();
        assert!(purr_train.replace(&mut purr_siding).is_ok());
        len_assert(&purr_train, 3);
        line_assert(&purr_train);
    }

    #[test]
    fn reroute_forward_inserts_at_cursor() {
        let mut purr_train = PurrTrain::with_cursor();
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train.attach(&mut purr_siding).is_ok());
        len_assert(&purr_train, 3);
        line_assert(&purr_train);
        purr_siding = construct_purr_siding();
        assert!(purr_train.attach(&mut purr_siding).is_ok());
        len_assert(&purr_train, 6);
        line_assert(&purr_train);
        purr_siding = construct_purr_siding();
        assert!(purr_train.reroute_at(&mut purr_siding, InsertPosition::Forward).is_ok());
        len_assert(&purr_train, 9);
        line_assert(&purr_train);
    }

    #[test]
    fn index_max_matches_double_attach() {
        let mut purr_train = PurrTrain::with_cursor();
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train.attach(&mut purr_siding).is_ok());
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train.attach(&mut purr_siding).is_ok());

        let mut purr_train_insert = PurrTrain::with_cursor();
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train_insert.attach(&mut purr_siding).is_ok());
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train_insert.reroute_at(&mut purr_siding, InsertPosition::Index(usize::MAX)).is_ok());
        
        len_assert(&purr_train, 6);
        len_assert(&purr_train_insert, 6);
        line_assert(&purr_train);
        line_assert(&purr_train_insert);
        assert_eq!(purr_train.line_as_slice(), purr_train_insert.line_as_slice());
    }

    #[test]
    fn forward_equals_index_zero() {
        let mut purr_train_forward = PurrTrain::with_cursor();
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train_forward.attach(&mut purr_siding).is_ok());
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train_forward.reroute_at(&mut purr_siding, InsertPosition::Forward).is_ok());

        let mut purr_train_index = PurrTrain::with_cursor();
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train_index.attach(&mut purr_siding).is_ok());
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train_index.reroute_at(&mut purr_siding, InsertPosition::Index(0)).is_ok());
        
        len_assert(&purr_train_forward, 6);
        len_assert(&purr_train_index, 6);
        line_assert(&purr_train_forward);
        line_assert(&purr_train_index);
        assert_eq!(purr_train_forward.line_as_slice(), purr_train_index.line_as_slice());
    }

    #[test]
    fn reroute_splits_line_at_index() {
        let mut purr_train = PurrTrain::with_cursor();
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train.attach(&mut purr_siding).is_ok());
        len_assert(&purr_train, 3);
        line_assert(&purr_train);
        purr_siding = construct_purr_siding();
        assert!(purr_train.attach(&mut purr_siding).is_ok());
        len_assert(&purr_train, 6);
        line_assert(&purr_train);
        purr_siding = construct_purr_siding();
        assert!(purr_train.reroute_at(&mut purr_siding, InsertPosition::Index(1)).is_ok());
        len_assert(&purr_train, 9);

        assert_eq!(purr_train.line_as_slice()[0].carriage, TestCarriage::Component);
        assert_eq!(purr_train.line_as_slice()[1].carriage, TestCarriage::Component);
        assert_eq!(purr_train.line_as_slice()[2].carriage, TestCarriage::Assembly);
        assert_eq!(purr_train.line_as_slice()[3].carriage, TestCarriage::Pipeline);
        assert_eq!(purr_train.line_as_slice()[4].carriage, TestCarriage::Assembly);
        assert_eq!(purr_train.line_as_slice()[5].carriage, TestCarriage::Pipeline);
    }

    #[test]
    fn attach_replace_reroute_drain_siding() {
        let mut purr_train = PurrTrain::with_cursor();
        let mut purr_siding = construct_purr_siding();
        assert!(!purr_siding.line_is_empty());
        assert!(purr_train.attach(&mut purr_siding).is_ok());
        assert!(purr_siding.line_is_empty());

        purr_siding = construct_purr_siding();
        assert!(!purr_siding.line_is_empty());
        assert!(purr_train.replace(&mut purr_siding).is_ok());
        assert!(purr_siding.line_is_empty());

        purr_siding = construct_purr_siding();
        assert!(!purr_siding.line_is_empty());
        assert!(purr_train.reroute_at(&mut purr_siding, InsertPosition::default()).is_ok());
        assert!(purr_siding.line_is_empty());
    }

    #[test]
    fn shrink_drops_passed_when_cursor_over_mark() {
        let mut purr_train = PurrTrain::with_cursor();
        let mut purr_siding = construct_purr_siding();

        assert!(purr_train.attach(&mut purr_siding).is_ok());
        purr_train.shrink_line(1);
        len_assert(&purr_train, 3);
        line_assert(&purr_train);

        purr_train.advance_train();
        purr_train.shrink_line(1);
        len_assert(&purr_train, 3);
        line_assert(&purr_train);

        purr_train.advance_train();
        purr_train.shrink_line(1);
        len_assert(&purr_train, 1);
        line_assert(&purr_train);
    }
    
    #[test]
    fn every_stage_boundary_is_caught() {
        let mut from_route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        let mut to_route_box = RouteBox::new(TestCarriage::Assembly, PurrRuleLite::instant());

        let mut purr_train = PurrTrain::with_cursor();
        let mut purr_siding = construct_purr_siding();

        assert!(purr_train.attach(&mut purr_siding).is_ok());
        let event = purr_train.advance_train();
        assert_eq!(event, PurrTrainEvent::Transition { from: from_route_box, to: Some(to_route_box) });
        len_assert(&purr_train, 3);
        line_assert(&purr_train);

        from_route_box = to_route_box;
        to_route_box.carriage = TestCarriage::Pipeline;

        let event = purr_train.advance_train();
        assert_eq!(event, PurrTrainEvent::Transition { from: from_route_box, to: Some(to_route_box) });
        len_assert(&purr_train, 3);
        line_assert(&purr_train);

        from_route_box = to_route_box;

        let event = purr_train.advance_train();
        assert_eq!(event, PurrTrainEvent::Transition { from: from_route_box, to: None });
        len_assert(&purr_train, 0);
        line_assert(&purr_train);
    }

    #[test]
    fn advance_on_empty_is_idle() {
        let mut purr_train: PurrTrain<PurrCursor<RouteBox<TestCarriage, PurrRuleLite>>> = PurrTrain::with_cursor();
        len_assert(&purr_train, 0);
        line_assert(&purr_train);
        let event = purr_train.advance_train();
        assert_eq!(event, PurrTrainEvent::Idle);
    }

    #[test]
    fn flag_gate_returns_running_then_completes() {
        let mut from_route_box = RouteBox::new(TestCarriage::Component, PurrRuleLite::instant());
        let mut to_route_box = RouteBox::new(TestCarriage::Assembly, PurrRuleLite::flag());

        let mut purr_train = PurrTrain::with_cursor();
        let mut purr_siding = construct_purr_siding();

        purr_siding.change_rule(TestCarriage::Assembly, PurrRuleLite::flag(), ChangeRuleCount::default());
        assert!(purr_train.attach(&mut purr_siding).is_ok());

        let event = purr_train.advance_train();
        assert_eq!(event, PurrTrainEvent::Transition { from: from_route_box, to: Some(to_route_box) });
        len_assert(&purr_train, 3);
        line_assert(&purr_train);

        from_route_box = to_route_box;
        to_route_box = RouteBox::new(TestCarriage::Pipeline, PurrRuleLite::instant());

        loop {
            let event = purr_train.advance_train();
            if matches!(event, PurrTrainEvent::Transition { .. }) {
                assert_eq!(event, PurrTrainEvent::Transition { from: RouteBox::new(TestCarriage::Assembly, PurrRuleLite::Flag(PurrFlag::from_flag(true))), to: Some(to_route_box) });
                len_assert(&purr_train, 3);
                line_assert(&purr_train);
                from_route_box = to_route_box;
                break;
            };
            assert_eq!(event, PurrTrainEvent::Running(from_route_box));
            if let Some(route_box) = purr_train.get_current_mut() 
                && let Some(flag) = route_box.rule.as_mut_rule::<PurrFlag>() { flag.set(true); };
        };

        let event = purr_train.advance_train();
        assert_eq!(event, PurrTrainEvent::Transition { from: from_route_box, to: None });

        len_assert(&purr_train, 0);
        line_assert(&purr_train);
    }

    #[test]
    fn clear_line_resets_cursor() {
        let mut purr_train = PurrTrain::with_cursor();
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train.attach(&mut purr_siding).is_ok());
        len_assert(&purr_train, 3);
        line_assert(&purr_train);
        purr_train.clear_line();
        len_assert(&purr_train, 0);
        line_assert(&purr_train);
        assert!(purr_train.line_is_empty());
        assert_eq!(purr_train.get_cursor(), 0);
    }

    #[test]
    fn attach_slice_matches_attach() {
        let mut purr_train = PurrTrain::with_cursor();

        let mut purr_siding = construct_purr_siding();
        assert!(purr_train.attach(&mut purr_siding).is_ok());


        let mut purr_train_slice = PurrTrain::with_cursor();
        let purr_siding = construct_purr_siding();
        let slice = purr_siding.line.as_slice();
        assert!(purr_train_slice.attach_slice(slice).is_ok());

        assert_trains_eq(purr_train, purr_train_slice);
    }

    #[test]
    fn replace_slice_matches_replace() {
        let mut purr_train = PurrTrain::with_cursor();

        let mut purr_siding = construct_purr_siding();
        assert!(purr_train.attach(&mut purr_siding).is_ok());
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train.replace(&mut purr_siding).is_ok());

        let mut purr_train_slice = PurrTrain::with_cursor();
        let purr_siding = construct_purr_siding();
        let slice = purr_siding.line.as_slice();
        assert!(purr_train_slice.attach_slice(slice).is_ok());
        assert!(purr_train_slice.replace_slice(slice).is_ok());

        assert_trains_eq(purr_train, purr_train_slice);
    }

    #[test]
    fn reroute_slice_forward_matches_reroute() {
        let mut purr_train = PurrTrain::with_cursor();

        let mut purr_siding = construct_purr_siding();
        assert!(purr_train.attach(&mut purr_siding).is_ok());
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train.reroute_at(&mut purr_siding, InsertPosition::Forward).is_ok());

        let mut purr_train_slice = PurrTrain::with_cursor();
        let purr_siding = construct_purr_siding();
        let slice = purr_siding.line.as_slice();
        assert!(purr_train_slice.attach_slice(slice).is_ok());
        assert!(purr_train_slice.reroute_at_slice(slice, InsertPosition::Forward).is_ok());

        assert_trains_eq(purr_train, purr_train_slice);
    }

    #[test]
    fn reroute_slice_index_matches_reroute() {
        let mut purr_train = PurrTrain::with_cursor();

        let mut purr_siding = construct_purr_siding();
        assert!(purr_train.attach(&mut purr_siding).is_ok());
        let mut purr_siding = construct_purr_siding();
        assert!(purr_train.reroute_at(&mut purr_siding, InsertPosition::Index(1)).is_ok());

        let mut purr_train_slice = PurrTrain::with_cursor();
        let purr_siding = construct_purr_siding();
        let slice = purr_siding.line.as_slice();
        assert!(purr_train_slice.attach_slice(slice).is_ok());
        assert!(purr_train_slice.reroute_at_slice(slice, InsertPosition::Index(1)).is_ok());

        assert_trains_eq(purr_train, purr_train_slice);
    }

    fn len_assert<S: PurrTrack<Step = TestCarriage>>(purr_train: &PurrTrain<S>, len: usize) { assert_eq!(purr_train.line_len(), len) }

    fn line_assert<S: PurrTrack<Step = TestCarriage>>(purr_train: &PurrTrain<S>) {
        for i in 0..purr_train.line_len() / 3 {
            assert_eq!(purr_train.line_as_slice()[3 * i].carriage, TestCarriage::Component);
            assert_eq!(purr_train.line_as_slice()[3 * i + 1].carriage, TestCarriage::Assembly);
            assert_eq!(purr_train.line_as_slice()[3 * i + 2].carriage, TestCarriage::Pipeline);
        };
    }

    fn assert_trains_eq<S: PurrTrack<Step = TestCarriage, Rule = PurrRuleLite>>(mut purr_train: PurrTrain<S>, mut other_purr_train: PurrTrain<S>) {
        assert!(!purr_train.line_is_empty());
        assert!(!other_purr_train.line_is_empty());
        assert_eq!(purr_train.line_len(), other_purr_train.line_len());
        for (m, o) in purr_train.line.tr_drain(..).unwrap().zip(other_purr_train.line.tr_drain(..).unwrap()) {
            assert_eq!(m, o);
        };
    }

    fn construct_purr_siding() -> PurrSiding<TestCarriage, PurrRuleLite> {
        let mut purr_design = PurrDesign::new();
        purr_design.single(TestCarriage::Component, PurrRuleLite::instant());
        purr_design.single(TestCarriage::Assembly, PurrRuleLite::instant());
        purr_design.chain(TestCarriage::Pipeline, PurrRuleLite::instant(), vec![TestCarriage::Component, TestCarriage::Assembly]);

        let mut purr_route = PurrRoute::new();
        let result = purr_route.construct_schedule(&purr_design, SaveMode::default(), DuplicatePolicy::default());
        assert!(result.is_ok());

        let mut purr_siding = PurrSiding::new();
        let result = purr_siding.launch(TestCarriage::Pipeline, SaveMode::default(), &purr_route);
        assert!(result.is_ok());
        purr_siding
    }
}
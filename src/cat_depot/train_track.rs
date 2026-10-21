use std::ops::RangeBounds;

use crate::cat_malloc::train_route::RouteBox;

use super::train_core::PurrStep;
use super::train_error:: TrainResult;
use super::train_rule::PurrRule;

pub trait PurrTrack {
    type Step: PurrStep;
    type Rule: PurrRule;

    fn tr_new() -> Self;
    fn tr_with_capacity(capacity: usize) -> Self;

    fn tr_get_cursor(&self) -> usize;
    fn tr_set_cursor(&mut self, index: usize) -> TrainResult<(), Self::Step>;
    fn tr_step_cursor(&mut self) -> TrainResult<(), Self::Step>;

    fn tr_get(&self, index: usize) -> Option<RouteBox<Self::Step, Self::Rule>>;
    fn tr_get_ref(&self, index: usize) -> Option<&RouteBox<Self::Step, Self::Rule>>;
    fn tr_get_mut(&mut self, index: usize) -> Option<&mut RouteBox<Self::Step, Self::Rule>>;

    fn tr_get_current(&self) -> Option<RouteBox<Self::Step, Self::Rule>>;
    fn tr_get_current_ref(&self) -> Option<&RouteBox<Self::Step, Self::Rule>>;
    fn tr_get_current_mut(&mut self) -> Option<&mut RouteBox<Self::Step, Self::Rule>>;

    fn tr_as_slice(&self) -> &[RouteBox<Self::Step, Self::Rule>];
    fn tr_as_mut_slice(&mut self) -> &mut [RouteBox<Self::Step, Self::Rule>];

    fn tr_extend<I: IntoIterator<Item = RouteBox<Self::Step, Self::Rule>>>(&mut self, iter: I) -> TrainResult<(), Self::Step>;

    fn tr_splice<'a, I: IntoIterator<Item = RouteBox<Self::Step, Self::Rule>> + 'a>(
        &'a mut self, range: impl RangeBounds<usize>, replace_with: I
    ) -> TrainResult<impl Iterator<Item = RouteBox<Self::Step, Self::Rule>>, Self::Step>;

    fn tr_drain(&mut self, range: impl RangeBounds<usize>) -> TrainResult<impl Iterator<Item = RouteBox<Self::Step, Self::Rule>>, Self::Step> ;

    fn tr_is_last(&self) -> bool;
    fn tr_len(&self) -> usize;
    fn tr_is_empty(&self) -> bool;
    fn tr_clear(&mut self);
}
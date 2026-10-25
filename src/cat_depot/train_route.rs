use crate::cat_malloc::train_route::RouteBox;
use crate::cat_malloc::train_siding::PurrSiding;

use super::train_core::PurrStep;
use super::train_rule::PurrRule;
use super::train_error::TrainResult;

pub trait RouteScheduler {
    type Step: PurrStep;
    type Rule: PurrRule;

    fn route_as_slice(&self, carriage: Self::Step) -> TrainResult<&[RouteBox<Self::Step, Self::Rule>], Self::Step>;
    fn load(&self, purr_siding: &mut PurrSiding<Self::Step, Self::Rule>, carriage: Self::Step) -> TrainResult<(), Self::Step>;
}
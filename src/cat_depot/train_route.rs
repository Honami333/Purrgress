use crate::cat_malloc::train_siding::PurrSiding;

use super::train_core::PurrStep;
use super::train_rule::PurrRule;
use super::train_error::TrainResult;

pub trait RouteScheduler {
    type Step: PurrStep;
    type Rule: PurrRule;

    fn load(&self, purr_siding: &mut PurrSiding<Self::Step, Self::Rule>, carriage: Self::Step) -> TrainResult<(), Self::Step>;
}

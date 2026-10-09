use crate::cat_depot::train_core::PurrStep;
use crate::cat_depot::train_rule::PurrRule;
use crate::cat_depot::train_error::TrainResult;
use crate::cat_malloc::train_siding::PurrSiding;


pub trait RouteScheduler {
    type Step: PurrStep;
    type Rule: PurrRule;

    fn load(&self, purr_siding: &mut PurrSiding<Self::Step, Self::Rule>, carriage: Self::Step) -> TrainResult<(), Self::Step>;
}

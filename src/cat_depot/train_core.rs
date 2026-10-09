use std::fmt::Debug;
use std::hash::Hash;

use crate::cat_depot::train_error::{TrainError, TrainResult};
use crate::cat_malloc::train_route::RouteBox;

use super::train_rule::PurrRule;

pub const DEFAULT_CAPACITY: usize = 16;


pub trait PurrStep: Debug + Copy + PartialEq + Eq + Hash {}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum InsertPosition {
    #[default] Forward,
    Index(usize)
}

impl InsertPosition {
    pub fn get(self, index: usize, max_index: usize) -> usize {
        let index = match self {
            InsertPosition::Forward => index,
            InsertPosition::Index(i) => i.saturating_add(index)
        };
        index.min(max_index)
    }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SaveMode {
    #[default] Clear,
    Keep
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DuplicatePolicy {
    #[default] Overwrite,
    Ignore,
    RaiseError
}

impl DuplicatePolicy {
    pub fn should_overwrite<T: PurrStep>(self, carriage: T) -> TrainResult<bool, T> {
        match self {
            DuplicatePolicy::RaiseError => Err(TrainError::DuplicateCarriage { carriage }),
            DuplicatePolicy::Ignore => Ok(false),
            DuplicatePolicy::Overwrite => Ok(true)
        }
    }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ChangeRuleCount {
    #[default] All,
    Count(usize),
    First
}

impl ChangeRuleCount {
    pub fn is_final_rule_count(&mut self) -> bool {
        match self {
            ChangeRuleCount::First => true,
            ChangeRuleCount::Count(count) => {
                *count = count.saturating_sub(1);
                *count == 0
            },
            ChangeRuleCount::All => false
        }
    }
}

#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PurrTrainEvent<T: PurrStep, U: PurrRule> {
    Idle,
    Running( RouteBox<T, U> ),
    Transition { from: RouteBox<T, U>, to: Option<RouteBox<T, U>> }
}

#[cfg(test)]
mod insert_position_test {
    use super::InsertPosition;

    // Проверка того, применяет ли InsertPosition::get смещение, ограничивает ли результат по max и насыщается ли при переполнении
    #[test]
    fn get_returns_correct_index_with_offset_and_clamping() {
        // max = 0
        assert_eq!(InsertPosition::Forward.get(1, 0), 0);

        // offset < max
        assert_eq!(InsertPosition::Forward.get(0, usize::MAX), 0);
        assert_eq!(InsertPosition::Forward.get(12, usize::MAX), 12);
        assert_eq!(InsertPosition::Forward.get(12, 20), 12);

        // offset == max
        assert_eq!(InsertPosition::Forward.get(12, 12), 12);
        assert_eq!(InsertPosition::Forward.get(usize::MAX, usize::MAX), usize::MAX);

        // offset > max
        assert_eq!(InsertPosition::Forward.get(12, 8), 8);

        // max = 0
        assert_eq!(InsertPosition::Index(1).get(2, 0), 0);
        assert_eq!(InsertPosition::Index(1).get(0, 0), 0);

        // sum < max
        assert_eq!(InsertPosition::Index(0).get(0, usize::MAX), 0);
        assert_eq!(InsertPosition::Index(0).get(12, usize::MAX), 12);
        assert_eq!(InsertPosition::Index(12).get(0, usize::MAX), 12);
        assert_eq!(InsertPosition::Index(12).get(12, usize::MAX), 24);
        assert_eq!(InsertPosition::Index(12).get(0, 20), 12);

        // sum == max
        assert_eq!(InsertPosition::Index(12).get(12, 24), 24);
        assert_eq!(InsertPosition::Index(12).get(0, 12), 12);

        // sum is max - 1 / max + 1
        assert_eq!(InsertPosition::Index(12).get(12, 23), 23);
        assert_eq!(InsertPosition::Index(12).get(12, 25), 24);

        // offset > max
        assert_eq!(InsertPosition::Index(0).get(12, 8), 8);
        assert_eq!(InsertPosition::Index(2).get(12, 8), 8);

        // sum > max
        assert_eq!(InsertPosition::Index(12).get(12, 20), 20);

        // addition overflow
        assert_eq!(InsertPosition::Index(usize::MAX).get(usize::MAX, usize::MAX), usize::MAX);
        assert_eq!(InsertPosition::Index(usize::MAX).get(1, usize::MAX), usize::MAX);
        assert_eq!(InsertPosition::Index(usize::MAX).get(5, 20), 20);
    }
}

#[cfg(test)]
mod duplicate_policy_test {
    use purrgress_macros::PurrStep;

    use crate::cat_depot::train_error::TrainError;

    use super::DuplicatePolicy;

    // Test carriage for DuplicatePolicy
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
    struct ComponentCarriage;

    // Verification of whether DuplicatePolicy::should_overwrite returns the expected result for each policy
    #[test]
    fn should_overwrite_returns_expected_result_for_each_policy() {
        assert_eq!(DuplicatePolicy::Overwrite.should_overwrite(ComponentCarriage), Ok(true));
        assert_eq!(DuplicatePolicy::Ignore.should_overwrite(ComponentCarriage), Ok(false));
        assert_eq!(DuplicatePolicy::RaiseError.should_overwrite(ComponentCarriage), Err(TrainError::DuplicateCarriage { carriage: ComponentCarriage }));
    }
}

#[cfg(test)]
mod change_rule_count_test {
    use super::ChangeRuleCount;

    // Verification of whether is_final_rule_count reports the final rule correctly for First, Count and All
    #[test]
    fn is_final() {
        // First stays final on repeated calls
        let mut first = ChangeRuleCount::First;
        assert!(first.is_final_rule_count());
        assert!(first.is_final_rule_count());

        // Count(2) becomes final only on the second call
        let mut count = ChangeRuleCount::Count(2);
        assert!(!count.is_final_rule_count());
        assert!(count.is_final_rule_count());

        // Count(1) is final on the first call
        let mut count = ChangeRuleCount::Count(1);
        assert!(count.is_final_rule_count());

        // Count(0) is final immediately
        let mut count = ChangeRuleCount::Count(0);
        assert!(count.is_final_rule_count());

        // All stays final on repeated calls
        let mut all = ChangeRuleCount::All;
        assert!(!all.is_final_rule_count());
        assert!(!all.is_final_rule_count());
    }
}
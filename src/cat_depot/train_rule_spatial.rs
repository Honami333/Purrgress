use std::fmt::Debug;

use crate::PurrRule;

use super::train_conditions;
use super::train_conditions_spatial::{self, PurrVec};

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PurrRule)]
pub enum PurrRuleSpatial {
    Instant(train_conditions::PurrInstant),
    Flag(train_conditions::PurrFlag),
    Timer(train_conditions::PurrTimer),
    Proximity(train_conditions_spatial::PurrProximity)
}

impl PurrRuleSpatial {
    pub fn instant() -> Self { Self::Instant(train_conditions::PurrInstant) }
    pub fn timer(duration: f32) -> Self { Self::Timer(train_conditions::PurrTimer::new(duration)) }
    pub fn flag() -> Self { Self::Flag(train_conditions::PurrFlag::new()) }
    pub fn proximity(current: PurrVec, target: PurrVec) -> Self { Self::Proximity(train_conditions_spatial::PurrProximity::new(current, target)) }
}
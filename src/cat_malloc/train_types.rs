use crate::condition::{self, PurrCondition};
use crate::types::PurrVec;

use purrgress_macros::PurrRule;


#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, Clone, PurrRule)]
pub enum StandardRules {
    Instant(condition::InstantCondition),
    Flag(condition::PurrFlag),
    Timer(condition::PurrTimer),
    Proximity(condition::PurrProximity)
}

impl StandardRules {
    pub fn instant() -> Self {
        Self::Instant(condition::InstantCondition)
    }

    pub fn timer(duration: f32) -> Self {
        Self::Timer(condition::PurrTimer::new(duration))
    }

    pub fn flag() -> Self {
        Self::Flag(condition::PurrFlag::new())
    }

    pub fn proximity(pos: PurrVec, start_pos: PurrVec, target_pos: PurrVec) -> Self {
        Self::Proximity(condition::PurrProximity::new(pos, start_pos, target_pos))
    }
}

pub trait PurrRule: Clone {
    fn is_finished(&mut self) -> bool;

    fn as_ref_rule<C>(&self) -> Option<&C> where Self: UnpackRule<C> { self.unpack_ref() }

    fn as_mut_rule<C>(&mut self) -> Option<&mut C> where Self: UnpackRule<C> { self.unpack_mut() }
}

pub trait UnpackRule<C> {
    fn unpack_ref(&self) -> Option<&C>;
    fn unpack_mut(&mut self) -> Option<&mut C>;
}
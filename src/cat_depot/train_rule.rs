use std::fmt::Debug;

use crate::PurrRule;

use super::train_conditions;

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PurrRule)]
pub enum PurrRuleLite {
    Instant(train_conditions::PurrInstant),
    Flag(train_conditions::PurrFlag),
    Timer(train_conditions::PurrTimer)
}

impl PurrRuleLite {
    #[inline] pub fn instant() -> Self { Self::Instant(train_conditions::PurrInstant) }

    #[inline] pub fn timer(duration: f32) -> Self { Self::Timer(train_conditions::PurrTimer::new(duration)) }

    #[inline] pub fn flag() -> Self { Self::Flag(train_conditions::PurrFlag::new()) }
}

pub trait PurrRule: PartialEq + Copy {
    fn is_finished(&self) -> bool;

    #[inline] fn as_rule<C: Copy>(&self) -> Option<C> where Self: UnpackRule<C> { self.unpack() }

    #[inline] fn as_ref_rule<C: Copy>(&self) -> Option<&C> where Self: UnpackRule<C> { self.unpack_ref() }

    #[inline] fn as_mut_rule<C: Copy>(&mut self) -> Option<&mut C> where Self: UnpackRule<C> { self.unpack_mut() }
}

pub trait UnpackRule<C: Copy> {
    fn unpack(self) -> Option<C>;

    fn unpack_ref(&self) -> Option<&C>;
    
    fn unpack_mut(&mut self) -> Option<&mut C>;
}
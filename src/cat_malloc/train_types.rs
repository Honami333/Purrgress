use std::fmt::Debug;
use std::ops::Range;

use cursorvec::CursorVec;

use crate::cat_malloc::train_route::RouteBox;
use crate::condition::{self, PurrCondition};
use crate::types::{PurrStep, PurrVec};

use purrgress_macros::PurrRule;

pub const VECTOR_SIZE: usize = 16;


#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufferMode {
    Clear,
    Keep
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PurrRule)]
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

pub trait PurrRule: Debug + Clone + Copy + PartialEq {
    fn is_finished(&self) -> bool;
    fn as_ref_rule<C>(&self) -> Option<&C> where Self: UnpackRule<C> { self.unpack_ref() }
    fn as_mut_rule<C>(&mut self) -> Option<&mut C> where Self: UnpackRule<C> { self.unpack_mut() }
}

pub trait UnpackRule<C> {
    fn unpack_ref(&self) -> Option<&C>;
    fn unpack_mut(&mut self) -> Option<&mut C>;
}

#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PurrTrainEvent<T: PurrStep, U: PurrRule> {
    Idle,
    Running( RouteBox<T, U> ),
    Transition { from: RouteBox<T, U>, to: Option<RouteBox<T, U>> }
}

#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TrackCapacity {
    Cap2,
    Cap4,
    Cap8,
    Cap16,
    #[default]
    Cap32,
    Cap64,
    Cap128,
    Cap256,
    Cap512,
    Cap1024,
    Cap2048
}

pub trait PurrTrack<V: Clone + Copy> {
    fn tr_new() -> Self;
    fn tr_with_capacity(size: TrackCapacity) -> Self;

    fn tr_get_cursor(&self) -> usize;
    fn tr_set_cursor(&mut self, index: usize);
    fn tr_step_cursor(&mut self);

    fn tr_get(&self, index: usize) -> Option<V>;
    fn tr_get_ref(&self, index: usize) -> Option<&V>;
    fn tr_get_mut(&mut self, index: usize) -> Option<&mut V>;

    fn tr_get_current(&self) -> Option<V>;
    fn tr_get_current_ref(&self) -> Option<&V>;
    fn tr_get_current_mut(&mut self) -> Option<&mut V>;

    fn tr_as_slice(&self) -> &[V];
    fn tr_as_mut_slice(&mut self) -> &mut [V];

    fn tr_extend<T: IntoIterator<Item = V>>(&mut self, iter: T);
    fn tr_splice<T: IntoIterator<Item = V>,>(&mut self, range: Range<usize>, iter: T);
    fn tr_drain(&mut self, range: Range<usize>) -> impl Iterator<Item = V>;

    fn tr_len(&self) -> usize;
    fn tr_is_empty(&self) -> bool;
    fn tr_clear(&mut self);
}

impl<V: Clone + Copy> PurrTrack<V> for CursorVec<V> {
    fn tr_new() -> Self { CursorVec::new() }
    fn tr_with_capacity(_size: TrackCapacity) -> Self { CursorVec::new() }

    fn tr_get_cursor(&self) -> usize { self.get_cursor().unwrap_or_default() }
    fn tr_set_cursor(&mut self, index: usize) {
        self.set_cursor(index);
        self.update_cursor();
    }
    fn tr_step_cursor(&mut self) {
        let index= self.tr_get_cursor();
        self.tr_set_cursor(index + 1);
    }

    fn tr_get(&self, index: usize) -> Option<V> { self.get(index).copied() }
    fn tr_get_ref(&self, index: usize) -> Option<&V> { self.get(index) }
    fn tr_get_mut(&mut self, index: usize) -> Option<&mut V> { self.get_mut(index) }

    fn tr_get_current(&self) -> Option<V> { 
        let index = self.tr_get_cursor();
        self.get(index).copied()
    }
    fn tr_get_current_ref(&self) -> Option<&V> { 
        let index = self.tr_get_cursor();
        self.get(index)
    }
    fn tr_get_current_mut(&mut self) -> Option<&mut V> { 
        let index = self.tr_get_cursor();
        self.get_mut(index)
    }

    fn tr_as_slice(&self) -> &[V] { self.as_slice() }
    fn tr_as_mut_slice(&mut self) -> &mut [V] { self.as_mut_slice() }

    fn tr_extend<T: IntoIterator<Item = V>>(&mut self, iter: T) { self.extend(iter); }
    fn tr_splice<T: IntoIterator<Item = V>,>(&mut self, range: Range<usize>, iter: T) { self.splice(range, iter); }
    fn tr_drain(&mut self, range: Range<usize>) -> impl Iterator<Item = V> { self.drain(range) }

    fn tr_len(&self) -> usize { self.len() }
    fn tr_is_empty(&self) -> bool { self.is_empty() }
    fn tr_clear(&mut self) {
        self.tr_set_cursor(0);
        self.clear();
    }
}
use tokio::sync::mpsc::Sender;

use wibr::New;

use crate::PurrCommand;
use crate::PurrReply;
use crate::cat_depot::train_core::InsertPosition;
use crate::cat_depot::train_core::PurrStep;
use crate::cat_depot::train_core::PurrTrainEvent;
use crate::cat_depot::train_rule::PurrRule;
use crate::cat_malloc::train_route::RouteBox;
use crate::cat_telegraph::dispatcher_link::PurrStation;
use crate::cat_telegraph::dispatcher_error::*;
use crate::cat_telegraph::dispatcher_core::*;

pub type CallCommand<Key: PurrKey, T: PurrStep> = PurrCall<Key, T>;
pub type AnswerReply<Key: PurrKey, T: PurrStep, U: PurrRule> = PurrAnswer<CallCommand<Key, T>, Key, T, U>;
pub type AnswerReplyKind<Key: PurrKey, T: PurrStep, U: PurrRule> = PurrAnswerKind<CallCommand<Key, T>, Key, T, U>;

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurrCallMask(pub u16);

impl PurrCallMask {
    #[inline(always)] pub fn new() -> Self { Self(0) }
    #[inline(always)] pub fn block<T: PurrStep>(&mut self, call: PurrCallKind<T>) { self.0 |= 1 << call.as_u16(); }
    #[inline(always)] pub fn unblock<T: PurrStep>(&mut self, call: PurrCallKind<T>) { self.0 &= !(1 << call.as_u16()); }
    #[inline(always)] pub fn is_blocked<T: PurrStep>(&self, call: PurrCallKind<T>) -> bool { self.0 & (1 << call.as_u16()) != 0 }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrCommand, New)]
pub struct PurrCall<Key: PurrKey, T: PurrStep> {
    pub key: Key,
    pub kind: PurrCallKind<T>
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum PurrCallKind<T: PurrStep> {
    CreatePurrStation { channel_cap: usize },
    RemovePurrStation,
    Attach { carriage: T },
    Replace { carriage: T },
    RerouteAt { carriage: T, position: InsertPosition },
    ClearLine,
    ShrinkLine { line_length: usize },
    GetCurrent,
    GetCursor,
    LineLen,
    LineIsEmpty,
    LineSnapshot,
    AdvanceTrain
}

impl<T: PurrStep> PurrCallKind<T> {
    #[inline(always)] pub fn as_u16(&self) -> u16 { unsafe { *(self as *const Self as *const u16) } }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, PurrReply, New)]
pub struct PurrAnswer<Command: PurrCommand, Key: PurrKey, T: PurrStep, U: PurrRule> {
    pub result: DispatcherResult<PurrAnswerKind<Command, Key, T, U>>
}

impl<Command, Key, T, U> PurrAnswer<Command, Key, T, U>
where
    Command: PurrCommand,
    Key: PurrKey,
    T: PurrStep,
    U: PurrRule
{
    #[inline] pub fn from_kind(kind: PurrAnswerKind<Command, Key, T, U>) -> Self { Self::new(Ok(kind)) }
    #[inline] pub fn from_error(error: DispatcherError) -> Self { Self::new(Err(error)) }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug)]
pub enum PurrAnswerKind<Command: PurrCommand, Key: PurrKey, T: PurrStep, U: PurrRule> {
    CreatePurrStation { station: PurrStation<Command, PurrAnswer<Command, Key, T, U>, Key> },
    RemovePurrStation { tx: Sender<PurrAnswer<Command, Key, T, U>> },
    Attach,
    Replace,
    RerouteAt,
    ClearLine,
    ShrinkLine,
    GetCurrent { current: Option<RouteBox<T, U>> },
    GetCursor { cursor: usize },
    LineLen { len: usize },
    LineIsEmpty { empty: bool },
    LineSnapshot { snapshot: Vec<RouteBox<T, U>> },
    AdvanceTrain { event: PurrTrainEvent<T, U> },
}
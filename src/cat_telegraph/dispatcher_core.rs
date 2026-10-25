use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use core::net::{Ipv4Addr, Ipv6Addr, IpAddr};
use core::net::{SocketAddrV4, SocketAddrV6, SocketAddr};

use tokio::sync::mpsc::{Receiver, Sender};

use wibr::Make;

use crate::cat_depot::train_core::PurrStep;
use crate::cat_depot::train_error::{TrainError, TrainResult};
use crate::cat_depot::train_route::RouteScheduler;
use crate::cat_depot::train_rule::PurrRule;
use crate::cat_depot::train_track::PurrTrack;
use crate::cat_malloc::train::PurrTrain;
use crate::cat_malloc::train_route::RouteBox;
use crate::cat_malloc::train_siding::PurrSiding;

use super::dispatcher_error::*;
use super::dispatcher::PurrDispatcher;

#[macro_export]
macro_rules! key {
    { $($ty:ty),* $(,)? } => { $( impl purrgress::cat_telegraph::dispatcher_core::PurrKey for $ty {} )* };
}

key!{
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    bool, char, &'static str,
    Ipv4Addr, Ipv6Addr, IpAddr,
    SocketAddrV4, SocketAddrV6, SocketAddr
}

pub trait PurrKey: Debug + Copy + Eq + Hash {}

pub trait PurrCommand {}

pub trait PurrReply {}

pub trait PurrHotLine<Reply: PurrReply> {
    type Key: PurrKey;

    fn add_route(&mut self, key: Self::Key, channel_cap: usize) -> DispatcherResult<Receiver<Reply>>;
    fn replace_route(&mut self, key: Self::Key, channel_cap: usize) -> DispatcherResult<Receiver<Reply>>;
    fn delete_route(&mut self, key: Self::Key) -> DispatcherResult<Sender<Reply>>;

    fn get_route(&self, key: Self::Key) -> DispatcherResult<&Sender<Reply>>;
    fn get_route_mut(&mut self, key: Self::Key) -> DispatcherResult<&mut Sender<Reply>>;
}
pub trait PurrOperatorOutput {
    type Key: PurrKey;
}
pub trait PurrOperator<Cargo: PurrCargo> {
    type Output: PurrOperatorOutput;

    type Step: PurrStep;
    type Rule: PurrRule;

    type Command: PurrCommand;
    type Reply: PurrReply;

    fn command<Line>(
        &mut self,dispatcher: &mut PurrDispatcher<Self::Command, Self::Reply, Line, Cargo>, command: Self::Command
    ) -> DispatcherResult<Self::Output> where Line: PurrHotLine<Self::Reply, Key = <Self::Output as PurrOperatorOutput>::Key>, Cargo: PurrCargo<Track: PurrTrack<Step = Self::Step, Rule = Self::Rule>>;

    fn reply<Line>(&mut self, hotline: &Line, input: Self::Output) -> DispatcherResult<()> where Line: PurrHotLine<Self::Reply, Key = <Self::Output as PurrOperatorOutput>::Key>;

    async fn reply_async<Line>(&mut self, hotline: &Line, input: Self::Output) -> DispatcherResult<()> where Line: PurrHotLine<Self::Reply, Key = <Self::Output as PurrOperatorOutput>::Key>;
}

pub trait PurrCargo {
    type Track: PurrTrack;

    fn train_ref(&self) -> &PurrTrain<Self::Track>;
    fn train_mut(&mut self) -> &mut PurrTrain<Self::Track>;

    #[inline] fn cargo_ref<C>(&self) -> Option<&C> where Self: UnpackCargo<C> { self.unpack_ref() }
    #[inline] fn cargo_mut<C>(&mut self) -> Option<&mut C> where Self: UnpackCargo<C> { self.unpack_mut() }
}

pub trait UnpackCargo<C> {
    fn unpack_ref(&self) -> Option<&C>;
    fn unpack_mut(&mut self) -> Option<&mut C>;
}

impl<S: PurrTrack> PurrCargo for PurrTrain<S> {
    type Track = S;

    #[inline] fn train_ref(&self) -> &PurrTrain<Self::Track> { self }
    #[inline] fn train_mut(&mut self) -> &mut PurrTrain<Self::Track> { self }
}

impl<S: PurrTrack, Route: RouteScheduler> UnpackCargo<Route> for PurrTrain<S> {
    #[inline] fn unpack_ref(&self) -> Option<&Route> { None }
    #[inline] fn unpack_mut(&mut self) -> Option<&mut Route> { None }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Make)]
pub struct PhantomRoute<T, U> {
    #[Some(PhantomData)] _step: PhantomData<T>,
    #[Some(PhantomData)] _rule: PhantomData<U>
}

impl<T, U> RouteScheduler for PhantomRoute<T, U>
where
    T: PurrStep,
    U: PurrRule
{
    type Step = T;
    type Rule = U;

    #[cold] fn route_as_slice(&self, _carriage: Self::Step) -> TrainResult<&[RouteBox<Self::Step, Self::Rule>], Self::Step> { Err(TrainError::PhantomRoute) }
    #[cold] fn load(&self, _purr_siding: &mut PurrSiding<Self::Step, Self::Rule>, _carriage: Self::Step) -> TrainResult<(), Self::Step> { Err(TrainError::PhantomRoute) }
}
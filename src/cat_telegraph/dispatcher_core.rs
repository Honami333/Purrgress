use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::Debug;
use std::hash::Hash;

use core::net::{Ipv4Addr, Ipv6Addr, IpAddr};
use core::net::{SocketAddrV4, SocketAddrV6, SocketAddr};

use tokio::sync::mpsc::{self, Receiver, Sender};

use wibr::{Make, New};

use crate::cat_depot::train_track::PurrTrack;
use crate::cat_malloc::train::PurrTrain;

use super::dispatcher_error::*;
use super::dispatcher::PurrDispatcher;

pub const FAST_ROUTES_CAPACITY: usize = 16;

pub type DoubleHotLineDef <Reply, K> = DoubleHotLine<Reply, K, FAST_ROUTES_CAPACITY>;

pub type FastHotLineDef <Reply, K> = FastHotLine<Reply, K, FAST_ROUTES_CAPACITY>;

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

pub trait PurrOperator {
    type Output;
    type Command: PurrCommand;
    type Reply: PurrReply;

    fn command<Reply, Line, Cargo>(
        &mut self,dispatcher: &mut PurrDispatcher<Self::Command, Reply, Line, Cargo>, command: Self::Command
    ) -> DispatcherResult<Self::Output> where Reply: PurrReply, Line: PurrHotLine<Reply>, Cargo: PurrCargo;

    fn reply<Line>(&mut self, hotline: &Line, input: &Self::Output) -> DispatcherResult<()> where Line: PurrHotLine<Self::Reply>;
}

pub trait PurrCargo {
    type Track: PurrTrack;

    fn train_ref(&self) -> &PurrTrain<Self::Track>;
    fn train_mut(&mut self) -> &mut PurrTrain<Self::Track>;

    fn cargo_ref<C>(&self) -> Option<&C> where Self: UnpackCargo<C> { self.unpack_ref() }
    fn cargo_mut<C>(&mut self) -> Option<&mut C> where Self: UnpackCargo<C> { self.unpack_mut() }
}

pub trait UnpackCargo<C> {
    fn unpack_ref(&self) -> Option<&C>;
    fn unpack_mut(&mut self) -> Option<&mut C>;
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DoubleLineMode {
    #[default] Dynamic,
    Fast
}

#[macro_export]
macro_rules! double_line {
    ($len:expr) => { purrgress::cat_telegraph::dispatcher_types::DoubleHotLine::<_, _, $len>::new() };
}

#[macro_export]
macro_rules! fast_line {
    ($len:expr) => { purrgress::cat_telegraph::dispatcher_types::FastHotLine::<_, _, $len>::new() };
}

#[derive(Debug, Make)]
pub struct DoubleHotLine<Reply: PurrReply, K: PurrKey, const N: usize = FAST_ROUTES_CAPACITY> {
    #[Some(DynamicHotLine::new())] pub dynamic: DynamicHotLine<Reply, K>,
    #[Some(FastHotLine::new())] pub fast: FastHotLine<Reply, K, N>,
    #[Some(DoubleLineMode ::default())] pub line_mode: DoubleLineMode,
    #[Some(0)] pub line_count: usize,
}

impl<Reply: PurrReply, K: PurrKey> Default for DoubleHotLineDef<Reply, K> {
    fn default() -> Self { Self::new() }
}

impl<Reply: PurrReply, K: PurrKey, const N: usize> PurrHotLine<Reply> for DoubleHotLine<Reply, K, N> {
    type Key = K;
    
    fn add_route(&mut self, key: Self::Key, channel_cap: usize) -> DispatcherResult<Receiver<Reply>> {
        let result = match self.line_mode {
            DoubleLineMode::Dynamic => self.dynamic.add_route(key, channel_cap),
            DoubleLineMode::Fast => self.fast.add_route(key, channel_cap)
        };
        self.line_count += 1;
        result
    }

    fn replace_route(&mut self, key: Self::Key, channel_cap: usize) -> DispatcherResult<Receiver<Reply>> {
        match self.line_mode {
            DoubleLineMode::Dynamic => self.dynamic.replace_route(key, channel_cap),
            DoubleLineMode::Fast => self.fast.replace_route(key, channel_cap)
        }
    }

    fn delete_route(&mut self, key: Self::Key) -> DispatcherResult<Sender<Reply>> {
        let result = match self.line_mode {
            DoubleLineMode::Dynamic => self.dynamic.delete_route(key),
            DoubleLineMode::Fast => self.fast.delete_route(key)
        };
        self.line_count -= 1;
        result
    }

    fn get_route(&self, key: Self::Key) -> DispatcherResult<&Sender<Reply>> {
        match self.line_mode {
            DoubleLineMode::Dynamic => self.dynamic.get_route(key),
            DoubleLineMode::Fast => self.fast.get_route(key)
        }
    }

    fn get_route_mut(&mut self, key: Self::Key) -> DispatcherResult<&mut Sender<Reply>> {
        match self.line_mode {
            DoubleLineMode::Dynamic => self.dynamic.get_route_mut(key),
            DoubleLineMode::Fast => self.fast.get_route_mut(key)
        }
    }
}

impl<Reply: PurrReply, K: PurrKey, const N: usize> DoubleHotLine<Reply, K, N> {
    pub fn is_dynamic(&self) -> bool { self.line_mode == DoubleLineMode::Dynamic }
    pub fn is_fast(&self) -> bool { self.line_mode == DoubleLineMode::Fast }

    pub fn swap_mode(&mut self) {
        self.line_mode = match self.line_mode {
            DoubleLineMode::Dynamic => DoubleLineMode ::Fast,
            DoubleLineMode::Fast => DoubleLineMode ::Dynamic,
        };
    }

    pub fn swap_mode_and_move(&mut self) -> DispatcherResult<()> {
        match self.line_mode {
            DoubleLineMode::Fast => { self.to_dynamic(); Ok(()) },
            DoubleLineMode::Dynamic => self.to_fast()
        }
    }

    pub fn to_fast(&mut self) -> DispatcherResult<()> {
        let free_route_count = self.fast.routes.iter().filter(|s| s.is_none()).count();
        match free_route_count >= self.dynamic.routes.len() {
            true => {
                for (fast_route, dyn_route) in self.fast.routes.iter_mut().filter(|s| s.is_none()).zip(self.dynamic.routes.drain()) {
                    *fast_route = Some(KeySenderReply::new(dyn_route.1, dyn_route.0));
                };
                self.line_mode = DoubleLineMode::Fast;
                Ok(())
            },
            false => Err(DispatcherError::HotLineOverFlow),
        }
    }

    pub fn to_dynamic(&mut self) {
        for fast_route in self.fast.routes.iter_mut() {
            if let Some(fast) = fast_route.take() { self.dynamic.routes.insert(fast.key, fast.sender); };
        };
        self.line_mode = DoubleLineMode::Dynamic;
    }
}

#[derive(Debug, Make)]
pub struct FastHotLine<Reply: PurrReply, K: PurrKey, const N: usize = FAST_ROUTES_CAPACITY> {
    #[Some([const { None }; N])] pub routes: [Option<KeySenderReply<Reply, K>>; N],
}

impl<Reply: PurrReply, K: PurrKey> Default for FastHotLineDef<Reply, K> {
    fn default() -> Self { Self::new() }
}

impl<Reply: PurrReply, K: PurrKey, const N: usize> PurrHotLine<Reply> for FastHotLine<Reply, K, N> {
    type Key = K;
    
    fn add_route(&mut self, key: Self::Key, channel_cap: usize) -> DispatcherResult<Receiver<Reply>> {
        if self.routes.iter().flatten().any(|s| s.key == key) { return Err(DispatcherError::DuplicateRoute); }
        let sender = self.routes.iter_mut().find(|s| s.is_none()).ok_or(DispatcherError::HotLineOverFlow)?;
        let (tx, rx) = mpsc::channel(channel_cap);
        *sender = Some(KeySenderReply::new(tx, key));
        Ok(rx)
    }

    fn replace_route(&mut self, key: Self::Key, channel_cap: usize) -> DispatcherResult<Receiver<Reply>> {
        for key_sender in self.routes.iter_mut().flatten() {
            if key_sender.key == key {
                let (tx, rx) = mpsc::channel(channel_cap);
                *key_sender = KeySenderReply::new(tx, key);
                return Ok(rx);
            };
        };
        Err(DispatcherError::RouteNotFound)
    }

    fn delete_route(&mut self, key: Self::Key) -> DispatcherResult<Sender<Reply>> {
        let sender = self.routes.iter_mut().find(|s| s.as_ref().is_some_and(|s| s.key == key)).ok_or(DispatcherError::RouteNotFound)?;
        sender.take().map(|s| s.sender).ok_or(DispatcherError::RouteNotFound)
    }

    fn get_route(&self, key: Self::Key) -> DispatcherResult<&Sender<Reply>> {
        self.routes.iter().flatten().find(|s| s.key == key).map(|s| &s.sender).ok_or(DispatcherError::RouteNotFound)
    }

    fn get_route_mut(&mut self, key: Self::Key) -> DispatcherResult<&mut Sender<Reply>> {
        self.routes.iter_mut().flatten().find(|s| s.key == key).map(|s| &mut s.sender).ok_or(DispatcherError::RouteNotFound)
    }
}


#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, New)]
pub struct KeySenderReply<Reply: PurrReply, K: PurrKey> {
    pub sender: Sender<Reply>,
    pub key: K,
}

#[derive(Debug, Make)]
pub struct DynamicHotLine<Reply: PurrReply, K: PurrKey> {
    #[Some(HashMap::new())] pub routes: HashMap<K, Sender<Reply>>,
}

impl<Reply: PurrReply, K: PurrKey> PurrHotLine<Reply> for DynamicHotLine<Reply, K> {
    type Key = K;
    
    fn add_route(&mut self, key: Self::Key, channel_cap: usize) -> DispatcherResult<Receiver<Reply>> {
        let Entry::Vacant(entry) = self.routes.entry(key) else { return Err(DispatcherError::DuplicateRoute); };
        let (tx, rx) = mpsc::channel(channel_cap);
        entry.insert(tx);
        Ok(rx)
    }

    fn replace_route(&mut self, key: Self::Key, channel_cap: usize) -> DispatcherResult<Receiver<Reply>> {
        let sender = self.routes.get_mut(&key).ok_or(DispatcherError::RouteNotFound)?;
        let (tx, rx) = mpsc::channel(channel_cap);
        *sender = tx;
        Ok(rx)
    }

    fn delete_route(&mut self, key: Self::Key) -> DispatcherResult<Sender<Reply>> {
        self.routes.remove(&key).ok_or(DispatcherError::RouteNotFound)
    }

    fn get_route(&self, key: Self::Key) -> DispatcherResult<&Sender<Reply>> {
        self.routes.get(&key).ok_or(DispatcherError::RouteNotFound)
    }

    fn get_route_mut(&mut self, key: Self::Key) -> DispatcherResult<&mut Sender<Reply>> {
        self.routes.get_mut(&key).ok_or(DispatcherError::RouteNotFound)
    }
}
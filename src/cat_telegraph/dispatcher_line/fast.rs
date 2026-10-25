use std::fmt::Debug;

use tokio::sync::mpsc::{self, Receiver, Sender};

use wibr::{Make, New};

use crate::cat_telegraph::dispatcher_error::*;
use crate::cat_telegraph::dispatcher_core::*;

pub const FAST_ROUTES_CAPACITY: usize = 16;

pub type FastHotLineDef <Reply, K> = FastHotLine<Reply, K, FAST_ROUTES_CAPACITY>;

#[macro_export]
macro_rules! fast_line {
    ($len:expr) => { purrgress::cat_telegraph::dispatcher_types::FastHotLine::<_, _, $len>::new() };
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, New)]
pub struct KeySenderReply<Reply: PurrReply, K: PurrKey> {
    pub sender: Sender<Reply>,
    pub key: K,
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
    
    #[inline(never)]
    fn add_route(&mut self, key: Self::Key, channel_cap: usize) -> DispatcherResult<Receiver<Reply>> {
        if self.routes.iter().flatten().any(|s| s.key == key) { return Err(DispatcherError::DuplicateRoute); }
        let sender = self.routes.iter_mut().find(|s| s.is_none()).ok_or(DispatcherError::HotLineOverFlow)?;
        let (tx, rx) = mpsc::channel(channel_cap);
        *sender = Some(KeySenderReply::new(tx, key));
        Ok(rx)
    }

    #[inline(never)]
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

    #[inline(always)]
    fn delete_route(&mut self, key: Self::Key) -> DispatcherResult<Sender<Reply>> {
        let sender = self.routes.iter_mut().find(|s| s.as_ref().is_some_and(|s| s.key == key)).ok_or(DispatcherError::RouteNotFound)?;
        sender.take().map(|s| s.sender).ok_or(DispatcherError::RouteNotFound)
    }

    #[inline(always)]
    fn get_route(&self, key: Self::Key) -> DispatcherResult<&Sender<Reply>> {
        self.routes.iter().flatten().find(|s| s.key == key).map(|s| &s.sender).ok_or(DispatcherError::RouteNotFound)
    }

    #[inline(always)]
    fn get_route_mut(&mut self, key: Self::Key) -> DispatcherResult<&mut Sender<Reply>> {
        self.routes.iter_mut().flatten().find(|s| s.key == key).map(|s| &mut s.sender).ok_or(DispatcherError::RouteNotFound)
    }
}
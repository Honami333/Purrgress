use std::fmt::Debug;
use std::hash::Hash;

use tokio::sync::mpsc::{Receiver, Sender};

use wibr::Make;

use crate::cat_telegraph::dispatcher_error::*;
use crate::cat_telegraph::dispatcher_core::*;

use super::dynamic::DynamicHotLine;
use super::fast::*;

pub type DoubleHotLineDef <Reply, K> = DoubleHotLine<Reply, K, FAST_ROUTES_CAPACITY>;

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

#[derive(Debug, Make)]
pub struct DoubleHotLine<Reply: PurrReply, K: PurrKey, const N: usize = FAST_ROUTES_CAPACITY> {
    #[Some(DynamicHotLine::new())] pub dynamic: DynamicHotLine<Reply, K>,
    #[Some(FastHotLine::new())] pub fast: FastHotLine<Reply, K, N>,
    #[Some(DoubleLineMode ::default())] pub line_mode: DoubleLineMode,
    #[Some(0)] pub line_count: usize,
}

impl<Reply: PurrReply, K: PurrKey> Default for DoubleHotLineDef<Reply, K> {
    #[inline] fn default() -> Self { Self::new() }
}

impl<Reply: PurrReply, K: PurrKey, const N: usize> PurrHotLine<Reply> for DoubleHotLine<Reply, K, N> {
    type Key = K;
    
    #[inline]
    fn add_route(&mut self, key: Self::Key, channel_cap: usize) -> DispatcherResult<Receiver<Reply>> {
        let result = match self.line_mode {
            DoubleLineMode::Dynamic => self.dynamic.add_route(key, channel_cap),
            DoubleLineMode::Fast => self.fast.add_route(key, channel_cap)
        };
        self.line_count += 1;
        result
    }

    #[inline]
    fn replace_route(&mut self, key: Self::Key, channel_cap: usize) -> DispatcherResult<Receiver<Reply>> {
        match self.line_mode {
            DoubleLineMode::Dynamic => self.dynamic.replace_route(key, channel_cap),
            DoubleLineMode::Fast => self.fast.replace_route(key, channel_cap)
        }
    }

    #[inline]
    fn delete_route(&mut self, key: Self::Key) -> DispatcherResult<Sender<Reply>> {
        let result = match self.line_mode {
            DoubleLineMode::Dynamic => self.dynamic.delete_route(key),
            DoubleLineMode::Fast => self.fast.delete_route(key)
        };
        self.line_count -= 1;
        result
    }

    #[inline]
    fn get_route(&self, key: Self::Key) -> DispatcherResult<&Sender<Reply>> {
        match self.line_mode {
            DoubleLineMode::Dynamic => self.dynamic.get_route(key),
            DoubleLineMode::Fast => self.fast.get_route(key)
        }
    }

    #[inline]
    fn get_route_mut(&mut self, key: Self::Key) -> DispatcherResult<&mut Sender<Reply>> {
        match self.line_mode {
            DoubleLineMode::Dynamic => self.dynamic.get_route_mut(key),
            DoubleLineMode::Fast => self.fast.get_route_mut(key)
        }
    }
}

impl<Reply: PurrReply, K: PurrKey, const N: usize> DoubleHotLine<Reply, K, N> {
    #[inline] pub fn is_dynamic(&self) -> bool { self.line_mode == DoubleLineMode::Dynamic }

    #[inline] pub fn is_fast(&self) -> bool { self.line_mode == DoubleLineMode::Fast }

    #[inline]
    pub fn swap_mode(&mut self) {
        self.line_mode = match self.line_mode {
            DoubleLineMode::Dynamic => DoubleLineMode ::Fast,
            DoubleLineMode::Fast => DoubleLineMode ::Dynamic,
        };
    }

    #[inline]
    pub fn swap_mode_and_move(&mut self) -> DispatcherResult<()> {
        match self.line_mode {
            DoubleLineMode::Fast => { self.to_dynamic(); Ok(()) },
            DoubleLineMode::Dynamic => self.to_fast()
        }
    }

    #[inline(never)]
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

    #[inline(never)]
    pub fn to_dynamic(&mut self) {
        for fast_route in self.fast.routes.iter_mut() {
            if let Some(fast) = fast_route.take() { self.dynamic.routes.insert(fast.key, fast.sender); };
        };
        self.line_mode = DoubleLineMode::Dynamic;
    }
}
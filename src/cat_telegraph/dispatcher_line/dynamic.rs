use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::Debug;

use tokio::sync::mpsc::{self, Receiver, Sender};

use wibr::Make;

use crate::cat_telegraph::dispatcher_error::*;
use crate::cat_telegraph::dispatcher_core::*;

#[derive(Debug, Make)]
pub struct DynamicHotLine<Reply: PurrReply, K: PurrKey> {
    #[Some(HashMap::new())] pub routes: HashMap<K, Sender<Reply>>,
}

impl<Reply: PurrReply, K: PurrKey> PurrHotLine<Reply> for DynamicHotLine<Reply, K> {
    type Key = K;
    
    #[inline(never)]
    fn add_route(&mut self, key: Self::Key, channel_cap: usize) -> DispatcherResult<Receiver<Reply>> {
        let Entry::Vacant(entry) = self.routes.entry(key) else { return Err(DispatcherError::DuplicateRoute); };
        let (tx, rx) = mpsc::channel(channel_cap);
        entry.insert(tx);
        Ok(rx)
    }

    #[inline(never)]
    fn replace_route(&mut self, key: Self::Key, channel_cap: usize) -> DispatcherResult<Receiver<Reply>> {
        let sender = self.routes.get_mut(&key).ok_or(DispatcherError::RouteNotFound)?;
        let (tx, rx) = mpsc::channel(channel_cap);
        *sender = tx;
        Ok(rx)
    }

    #[inline(always)]
    fn delete_route(&mut self, key: Self::Key) -> DispatcherResult<Sender<Reply>> {
        self.routes.remove(&key).ok_or(DispatcherError::RouteNotFound)
    }

    #[inline(always)]
    fn get_route(&self, key: Self::Key) -> DispatcherResult<&Sender<Reply>> {
        self.routes.get(&key).ok_or(DispatcherError::RouteNotFound)
    }

    #[inline(always)]
    fn get_route_mut(&mut self, key: Self::Key) -> DispatcherResult<&mut Sender<Reply>> {
        self.routes.get_mut(&key).ok_or(DispatcherError::RouteNotFound)
    }
}
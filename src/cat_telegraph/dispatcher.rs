use std::collections::HashMap;

use rkyv::rancor::Error as RkyvError;
use rkyv::{Archive, Deserialize,};
use rkyv::api::high::HighDeserializer;
use rkyv::api::high::HighValidator;
use rkyv::bytecheck::CheckBytes;

use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;
use wibr::MakeFull;

use crate::cat_malloc::train_route::RouteBox;
use crate::types::PurrStep;

use crate::cat_malloc::purr_train::*;
use crate::cat_malloc::train_types::*;

use super::dispatcher_types::*;
use super::station_link::*;


#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, MakeFull)]
#[Extern(channel_cap: usize)]
#[Code({
    let (tx, rx) = mpsc::channel(channel_cap);
    let (line_tx, line_rx) = mpsc::channel(channel_cap);
})]
#[Ret((Self, PurrLink<T, U, C>))]
#[RetWith((Self, PurrLink::new(tx, line_rx)))]
pub struct PurrDispatcher<T: PurrStep, U: PurrRule, C: PurrKey, S: PurrTrack<RouteBox<T, U>> = cursorvec::CursorVec<RouteBox<T, U>>> {
    #[Some(PurrTrain::new())] pub purr_train: PurrTrain<T, U, S>,
    #[Some(rx)] pub channel: Receiver<DispatcherCommand<C>>,
    #[Some(line_tx)] pub line_channel: Sender<DispatcherReply<T, U>>,
    #[Some(0)] pub line_count: usize,
    #[Some(Default::default())] pub fast_routes: [KeySenderReply<T, U, C>; FAST_ROUTES_CAPACITY],
    #[Some(HashMap::new())] pub dynamic_routes: HashMap<C, Sender<DispatcherReply<T, U>>>
}

impl<T, U, C, S> PurrDispatcher<T, U, C, S> 
where 
    T: PurrStep,
    U: PurrRule,
    C: PurrKey,
    S: PurrTrack<RouteBox<T, U>>
{
    pub fn add_route(&mut self, key: C, action: RouteAction, channel_cap: usize) -> Result<Receiver<DispatcherReply<T, U>>, PurrError> {
        let (tx, rx) = mpsc::channel(channel_cap);
        if self.line_count < FAST_ROUTES_CAPACITY {
            if matches!(action, RouteAction::Default | RouteAction::AddConfigure { check_duplicates: true, .. }) {
                for key_sender_reply in self.fast_routes.iter() {
                    if key_sender_reply.sender.is_some() && key_sender_reply.key == key {
                        return Err( PurrError::DuplicateInFastRoutes("Route addition failed: The key already exists in the fast routes array.".to_string()));
                    };
                };
                if matches!(action, RouteAction::AddConfigure { check_duplicates_migrate: true, .. }) && self.dynamic_routes.contains_key(&key) {
                    return Err( PurrError::DuplicateInDynamicRoutes("Route addition failed: The key already exists in the dynamic routes map (cross-check).".to_string()) );
                };
            };
            for key_sender_reply in self.fast_routes.iter_mut() {
                if key_sender_reply.sender.is_none() {
                    key_sender_reply.sender = Some(tx);
                    key_sender_reply.key = key;
                    break;
                };
            };
        } else {
            if matches!(action, RouteAction::Default | RouteAction::AddConfigure { check_duplicates: true, .. }) && self.dynamic_routes.contains_key(&key) {
                return Err( PurrError::DuplicateInDynamicRoutes("Route addition failed: The key already exists in the dynamic routes map.".to_string()) );
            };
            if matches!(action, RouteAction::AddConfigure { check_duplicates_migrate: true, .. }) {
                for key_sender_reply in self.fast_routes.iter() {
                    if key_sender_reply.sender.is_some() && key_sender_reply.key == key { 
                        return Err( PurrError::DuplicateInFastRoutes("Route addition failed: The key already exists in the fast routes array (cross-check).".to_string()) );
                    };
                };
            };
            if matches!(action, RouteAction::Default | RouteAction::AddConfigure { migrate_to_dynamic: true, .. }) {
                for key_sender_reply in self.fast_routes.iter_mut() {
                    if let Some(sender) = key_sender_reply.sender.take() {
                        self.dynamic_routes.insert(key_sender_reply.key, sender);
                    };
                };
                self.fast_routes = Default::default();
            };
            self.dynamic_routes.insert(key, tx);
        };
        self.line_count += 1;
        Ok(rx)
    }

    pub fn delete_route(&mut self, key: C, action: RouteAction) {
        if self.line_count <= FAST_ROUTES_CAPACITY || matches!(action, RouteAction::DeleteConfigure { delete_everywhere: true, .. }) {
            let key_sender_reply_op = self.fast_routes.iter_mut().find(|k| k.key == key && k.sender.is_some());
            if let Some(key_sender_reply) = key_sender_reply_op {
                key_sender_reply.sender = None;
                key_sender_reply.key = Default::default();
                self.line_count -= 1;
            };
        };
        if self.line_count > FAST_ROUTES_CAPACITY || matches!(action, RouteAction::DeleteConfigure { delete_everywhere: true, .. }) {
            if self.dynamic_routes.remove(&key).is_some() { self.line_count -= 1; };
            if !(self.line_count <= FAST_ROUTES_CAPACITY && matches!(action, RouteAction::Default | RouteAction::DeleteConfigure { migrate_to_fast: true, .. })) { return; }
            for (sender_key, sender) in self.dynamic_routes.drain() {
                for key_sender_reply in self.fast_routes.iter_mut() {
                    if key_sender_reply.sender.is_none() {
                        key_sender_reply.sender = Some(sender);
                        key_sender_reply.key = sender_key;
                        break;
                    };
                };
            };
        };
    }

    pub fn get_route(&self, key: C, action: RouteAction) -> Option<&Sender<DispatcherReply<T, U>>> {
        if self.line_count <= FAST_ROUTES_CAPACITY || matches!(action, RouteAction::GetConfigure { find_all: true }) {
            let key_sender_reply_op = self.fast_routes.iter().find(|k| k.key == key && k.sender.is_some());
            if let Some(key_sender_reply) = key_sender_reply_op {
                return key_sender_reply.sender.as_ref();
            };
        };
        if self.line_count > FAST_ROUTES_CAPACITY || matches!(action, RouteAction::GetConfigure { find_all: true }) {
            return self.dynamic_routes.get(&key);
        };
        None
    }
}

impl<T, U, C, S> PurrDispatcher<T, U, C, S> 
where 
    T: PurrStep + Archive,
    T::Archived: 
        for<'a> CheckBytes<HighValidator<'a, RkyvError>> 
            + Deserialize<T, HighDeserializer<RkyvError>>,
    U: PurrRule + Archive,
    U::Archived: 
        for<'a> CheckBytes<HighValidator<'a, RkyvError>> 
            + Deserialize<U, HighDeserializer<RkyvError>>,
    C: PurrKey,
    S: PurrTrack<RouteBox<T, U>>
{
    pub async fn read_command(&mut self) -> Option<DispatcherCommand<C>> {
        self.channel.recv().await
    }

    pub async fn dispatch(&mut self, action: RouteAction) -> Result<Option<C>, PurrError> {
        let dispatcher_command = self.read_command().await;
        if let Some(command) = dispatcher_command {
            let (reply_key, reply) = match command {
                DispatcherCommand::AddRoute { key, action, channel_cap } => {
                    let rx_reply  = self.add_route(key, action, channel_cap)?;
                    self.line_channel.send(DispatcherReply::PurrStation { rx_reply }).await.map_err(|e| PurrError::Internal(e.to_string()))?;
                    return Ok(Some(key));
                }
                DispatcherCommand::DeleteRoute { key, action } => {
                    self.delete_route(key, action);
                    return Ok(Some(key));
                }
                DispatcherCommand::Attach { siding_data, key } => { 
                    self.purr_train.attach_bytes(siding_data)?;
                    (key, DispatcherReply::None)
                },
                DispatcherCommand::Replace { siding_data, key } => {
                    self.purr_train.replace_bytes(siding_data)?;
                    (key, DispatcherReply::None)
                },
                DispatcherCommand::RerouteAt { siding_data, position, key } => {
                    self.purr_train.reroute_at_bytes(siding_data, position)?;
                    (key, DispatcherReply::None)
                },
                DispatcherCommand::ShrinkLine { line_length, key } => {
                    self.purr_train.shrink_line(line_length);
                    (key, DispatcherReply::None)
                },
                DispatcherCommand::AdvanceTrain { key } => {
                    let event = self.purr_train.advance_train();
                    (key, DispatcherReply::Advance { event })
                },
                DispatcherCommand::GetCurrent { key } => {
                    let route_box = self.purr_train.get_current();
                    (key, DispatcherReply::Current { route_box })
                },
                DispatcherCommand::GetCursor {key } => {
                    let cursor = self.purr_train.get_cursor();
                    (key, DispatcherReply::Cursor { cursor })
                },
            };
            if let Some(sender) = self.get_route(reply_key, action) {
                sender.send(reply).await.map_err(|e| PurrError::Internal(e.to_string()))?;
            };
            return Ok(Some(reply_key));
        }
        Ok(None)
    }
}

use std::collections::HashMap;

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
    #[Some(rx)] pub channel: Receiver<DispatcherCommand<T, C>>,
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
    pub fn add_route(&mut self, key: C, route_action: RouteAction, channel_cap: usize) -> Result<Receiver<DispatcherReply<T, U>>, DispatcherError> {
        let (tx, rx) = mpsc::channel(channel_cap);
        if self.line_count < FAST_ROUTES_CAPACITY {
            if matches!(route_action, RouteAction::Default | RouteAction::AddConfigure { check_duplicates: true, .. }) {
                for key_sender_reply in self.fast_routes.iter() {
                    if key_sender_reply.sender.is_some() && key_sender_reply.key == key {
                        return Err( DispatcherError::DuplicateInFastRoutes);
                    };
                };
                if matches!(route_action, RouteAction::AddConfigure { check_duplicates_migrate: true, .. }) && self.dynamic_routes.contains_key(&key) {
                    return Err( DispatcherError::DuplicateInDynamicRoutes );
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
            if matches!(route_action, RouteAction::Default | RouteAction::AddConfigure { check_duplicates: true, .. }) && self.dynamic_routes.contains_key(&key) {
                return Err( DispatcherError::DuplicateInDynamicRoutes );
            };
            if matches!(route_action, RouteAction::AddConfigure { check_duplicates_migrate: true, .. }) {
                for key_sender_reply in self.fast_routes.iter() {
                    if key_sender_reply.sender.is_some() && key_sender_reply.key == key { 
                        return Err( DispatcherError::DuplicateInFastRoutes );
                    };
                };
            };
            if matches!(route_action, RouteAction::Default | RouteAction::AddConfigure { migrate_to_dynamic: true, .. }) {
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

    pub fn delete_route(&mut self, key: C, route_action: RouteAction) {
        if self.line_count <= FAST_ROUTES_CAPACITY || matches!(route_action, RouteAction::DeleteConfigure { delete_everywhere: true, .. }) {
            let key_sender_reply_op = self.fast_routes.iter_mut().find(|k| k.key == key && k.sender.is_some());
            if let Some(key_sender_reply) = key_sender_reply_op {
                key_sender_reply.sender = None;
                key_sender_reply.key = Default::default();
                self.line_count -= 1;
            };
        };
        if self.line_count > FAST_ROUTES_CAPACITY || matches!(route_action, RouteAction::DeleteConfigure { delete_everywhere: true, .. }) {
            if self.dynamic_routes.remove(&key).is_some() { self.line_count -= 1; };
            if !(self.line_count <= FAST_ROUTES_CAPACITY && matches!(route_action, RouteAction::Default | RouteAction::DeleteConfigure { migrate_to_fast: true, .. })) { return; }
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

    pub fn get_route(&self, key: C, route_action: RouteAction) -> Option<&Sender<DispatcherReply<T, U>>> {
        if self.line_count <= FAST_ROUTES_CAPACITY || matches!(route_action, RouteAction::GetConfigure { find_all: true }) {
            let key_sender_reply_op = self.fast_routes.iter().find(|k| k.key == key && k.sender.is_some());
            if let Some(key_sender_reply) = key_sender_reply_op {
                return key_sender_reply.sender.as_ref();
            };
        };
        if self.line_count > FAST_ROUTES_CAPACITY || matches!(route_action, RouteAction::GetConfigure { find_all: true }) {
            return self.dynamic_routes.get(&key);
        };
        None
    }
}

impl<T, U, C, S> PurrDispatcher<T, U, C, S> 
where 
    T: PurrStep,
    U: PurrRule,
    C: PurrKey,
    S: PurrTrack<RouteBox<T, U>>
{
    pub async fn read_command(&mut self) -> Option<DispatcherCommand<T, C>> { self.channel.recv().await }

    pub async fn dispatch(&mut self, route_action: RouteAction) -> Result<DispatchData<T, C>, DispatcherError> {
        let mut dispatch_data = DispatchData::new();

        let dispatcher_command = self.read_command().await;
        if let Some(command) = dispatcher_command {
            let (reply_key, reply) = match command {
                DispatcherCommand::AddRoute { key, action, channel_cap } => {
                    dispatch_data.action = Some(action);
                    let rx_reply  = self.add_route(key, action, channel_cap)?;
                    self.line_channel.send(DispatcherReply::PurrStation { rx_reply }).await.map_err(|_| DispatcherError::LineChannelClosed)?;
                    return Ok(dispatch_data);
                }
                DispatcherCommand::DeleteRoute { key, action } => {
                    dispatch_data.action = Some(action);
                    self.delete_route(key, action);
                    return Ok(dispatch_data);
                }
                DispatcherCommand::Attach { carriage, key } => { 
                    dispatch_data.carriage = Some(carriage);
                    (key, DispatcherReply::None)
                },
                DispatcherCommand::Replace { carriage, key } => {
                    dispatch_data.carriage = Some(carriage);
                    (key, DispatcherReply::None)
                },
                DispatcherCommand::RerouteAt { carriage, position, key } => {
                    dispatch_data.carriage = Some(carriage);
                    dispatch_data.insert_position = Some(position);
                    (key, DispatcherReply::None)
                },
                DispatcherCommand::ShrinkLine { line_length, key } => {
                    dispatch_data.line_length = Some(line_length);
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
            if let Some(sender) = self.get_route(reply_key, route_action) {
                sender.send(reply).await.map_err(|_| DispatcherError::RouteChannelClosed)?;
            };
            dispatch_data.key = Some(reply_key);
            dispatch_data.command = Some(command);
            return Ok(dispatch_data);
        }
        Ok(dispatch_data)
    }
}


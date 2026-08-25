use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;

use wibr::{New, Make};

use crate::types::PurrStep;

use crate::cat_malloc::train_types::*;

use super::dispatcher_types::*;


#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, Make)]
pub struct KeySenderReply<T: PurrStep, U: PurrRule, C: PurrKey> {
    #[None] pub sender: Option<Sender<DispatcherReply<T, U>>>,
    #[Some(C::default())] pub key: C,
}

impl<T: PurrStep, U: PurrRule, C: PurrKey> Default for KeySenderReply<T, U, C> {
    fn default() -> Self { Self::new() }
}

impl<T, U, C> KeySenderReply<T, U, C> 
where 
    T: PurrStep,
    U: PurrRule,
    C: PurrKey
{
    pub fn set_sender(&mut self, sender: Sender<DispatcherReply<T, U>>) { self.sender = Some(sender); }
    pub fn set_key(&mut self, key: C) { self.key = key; }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, Make)]
pub struct PurrStation<T: PurrStep, U: PurrRule, C: PurrKey> {
    pub main_channel: Sender<DispatcherCommand<T, C>>,
    pub my_channel: Receiver<DispatcherReply<T, U>>,
    pub key: C
}

impl<T, U, C> PurrStation<T, U, C>
where
    T: PurrStep,
    U: PurrRule,
    C: PurrKey
{
    pub fn get_key(&self) -> C { self.key }

    pub async fn send_command(&mut self, command: DispatcherCommand<T, C>) -> Result<Option<DispatcherReply<T, U>>, DispatcherError> {
        self.main_channel.send(command).await.map_err(|e| DispatcherError::Internal(e.to_string()))?;
        Ok(self.my_channel.recv().await)
    }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, New)]
pub struct PurrLink<T: PurrStep, U: PurrRule, C: PurrKey> {
    pub channel: Sender<DispatcherCommand<T, C>>,
    pub line_channel: Receiver<DispatcherReply<T, U>>
}

impl<T, U, C> PurrLink<T, U, C> 
where
    T: PurrStep,
    U: PurrRule,
    C: PurrKey
{
    pub async fn new_route(&mut self, key: C, action: RouteAction, channel_cap: usize) -> Result<Option<DispatcherReply<T, U>>, DispatcherError> {
        self.channel.send(DispatcherCommand::AddRoute { key, action, channel_cap }).await.map_err(|e| DispatcherError::Internal(e.to_string()))?;
        Ok(self.line_channel.recv().await)
    }

    pub async fn delete_route(&self, key: C, action: RouteAction) -> Result<(), DispatcherError> {
        self.channel.send(DispatcherCommand::DeleteRoute { key, action }).await.map_err(|e| DispatcherError::Internal(e.to_string()))
    }

    pub fn new_line(&self) -> Sender<DispatcherCommand<T, C>> { self.channel.clone() }
}

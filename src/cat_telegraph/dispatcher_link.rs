use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;

use wibr::New;

use super::dispatcher_error::*;
use super::dispatcher_core::*;

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, New)]
pub struct PurrStation<Command: PurrCommand, Reply: PurrReply, K: PurrKey> {
    pub main_channel: Sender<Command>,
    pub line_channel: Receiver<Reply>,
    pub key: K
}

impl<Command, Reply, K> PurrStation<Command, Reply, K> 
where
    Command: PurrCommand,
    Reply: PurrReply,
    K: PurrKey
{
    #[inline]
    pub fn send_command(&self, command: Command) -> DispatcherResult<()> {
        self.main_channel.try_send(command).map_err(|e| e.into())
    }

    #[inline]
    pub fn read_reply(&mut self) -> Option<DispatcherResult<Reply>> {
        map_try_recv(self.line_channel.try_recv(), DispatcherError::LineChannelClosed)
    }

     #[inline(never)]
    pub async  fn send_command_async(&self, command: Command) -> DispatcherResult<()> {
        self.main_channel.send(command).await.map_err(|_| DispatcherError::LineChannelClosed)
    }

     #[inline(never)]
    pub async fn read_reply_async(&mut self) -> DispatcherResult<Reply> {
        self.line_channel.recv().await.ok_or(DispatcherError::LineChannelClosed)
    }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, New)]
pub struct PurrLink<Command: PurrCommand, Reply: PurrReply> {
    pub main_channel: Sender<Command>,
    pub link_channel: Receiver<Reply>
}

impl<Command, Reply> PurrLink<Command, Reply> 
where
    Command: PurrCommand,
    Reply: PurrReply
{
    #[inline]
    pub fn send_command(&self, command: Command) -> DispatcherResult<()> {
        self.main_channel.try_send(command).map_err(|e| e.into())
    }

    #[inline]
    pub fn read_reply(&mut self) -> Option<DispatcherResult<Reply>> {
        map_try_recv(self.link_channel.try_recv(), DispatcherError::LinkChannelClosed)
    }

    #[inline(never)]
    pub async  fn send_command_async(&self, command: Command) -> DispatcherResult<()> {
        self.main_channel.send(command).await.map_err(|_| DispatcherError::LinkChannelClosed)
    }

     #[inline(never)]
    pub async fn read_reply_async(&mut self) -> DispatcherResult<Reply> {
        self.link_channel.recv().await.ok_or(DispatcherError::LinkChannelClosed)
    }
}

#[derive(Debug, New)]
pub struct PurrDispatcherLink<Command: PurrCommand, Reply: PurrReply> {
    pub main_channel: Receiver<Command>,
    pub link_channel: Sender<Reply>,
}
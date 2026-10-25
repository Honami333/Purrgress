use wibr::MakeFull;

use crate::cat_depot::train_track::PurrTrack;
use crate::cat_telegraph::dispatcher_link::PurrLink;

use super::dispatcher_link::PurrDispatcherLink;
use super::dispatcher_error::*;
use super::dispatcher_core::*;

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, MakeFull)]
#[Extern(channel_cap: usize)]
#[Code({
    let (link_tx, line_rx) = tokio::sync::mpsc::channel(channel_cap);
    let (main_tx, main_rx) = tokio::sync::mpsc::channel(channel_cap);
})]
#[Ret((Self, PurrLink<Command, Reply>))]
#[RetWith((Self, PurrLink::new(main_tx, line_rx)))]
pub struct PurrDispatcher<Command: PurrCommand, Reply: PurrReply, Line: PurrHotLine<Reply>, Cargo: PurrCargo> {
    #[Some(PurrDispatcherLink::new(main_rx, link_tx))] pub link: PurrDispatcherLink<Command, Reply>,
    pub cargo: Cargo,
    pub hotline: Line
}

impl<Command, Reply, Line, Cargo, Key> PurrDispatcher<Command, Reply, Line, Cargo> 
where 
    Command: PurrCommand,
    Reply: PurrReply,
    Line: PurrHotLine<Reply, Key = Key>,
    Cargo: PurrCargo,
    Key: PurrKey
{
    #[inline]
    pub fn dispatch<O: PurrOperator<Cargo, Output: PurrOperatorOutput<Key = Key>, Command = Command, Reply = Reply>>(&mut self, operator: &mut O) -> DispatcherResult<Option<O::Output>> where Cargo: PurrCargo<Track: PurrTrack<Step = O::Step, Rule = O::Rule>> {
        let Some(command) = map_try_recv(self.link.main_channel.try_recv(), DispatcherError::CommandChannelClosed) else { return Ok(None); };
        operator.command(self, command?).map(Some)
    }

    #[inline]
    pub fn send_reply<O: PurrOperator<Cargo, Output: PurrOperatorOutput<Key = Key>, Command = Command, Reply = Reply>>(&self, operator: &mut O, input: O::Output) -> DispatcherResult<()> {
        operator.reply(&self.hotline, input)
    }

     #[inline(never)]
    pub async fn dispatch_async<O: PurrOperator<Cargo, Output: PurrOperatorOutput<Key = Key>, Command = Command, Reply = Reply>>(&mut self, operator: &mut O) -> DispatcherResult<Option<O::Output>> where Cargo: PurrCargo<Track: PurrTrack<Step = O::Step, Rule = O::Rule>> {
        let command = self.link.main_channel.recv().await.ok_or(DispatcherError::CommandChannelClosed);
        operator.command(self, command?).map(Some)
    }

     #[inline(never)]
    pub async fn send_reply_async<O: PurrOperator<Cargo, Output: PurrOperatorOutput<Key = Key>, Command = Command, Reply = Reply>>(&self, operator: &mut O, input: O::Output) -> DispatcherResult<()> {
        operator.reply_async(&self.hotline, input).await
    }
}

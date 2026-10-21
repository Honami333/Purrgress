use wibr::New;

use super::dispatcher_link::PurrDispatcherLink;
use super::dispatcher_error::*;
use super::dispatcher_core::*;

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, New)]
pub struct PurrDispatcher<Command: PurrCommand, Reply: PurrReply, Line: PurrHotLine<Reply>, Cargo: PurrCargo> {
    pub purr_train: Cargo,
    pub link: PurrDispatcherLink<Command, Reply>,
    pub hotline: Line
}

impl<Command, Reply, Line, Cargo> PurrDispatcher<Command, Reply, Line, Cargo> 
where 
    Command: PurrCommand,
    Reply: PurrReply,
    Line: PurrHotLine<Reply>,
    Cargo: PurrCargo
{
    pub fn dispatch<O: PurrOperator<Command = Command>>(&mut self, operator: &mut O) -> DispatcherResult<Option<O::Output>> {
        let Some(command) = map_try_recv(self.link.main_channel.try_recv(), DispatcherError::CommandChannelClosed) else { return Ok(None); };
        operator.command(self, command?).map(Some)
    }

    pub fn send_reply<O: PurrOperator<Command = Command, Reply = Reply>>(&self, operator: &mut O, input: &O::Output) -> DispatcherResult<()> {
        operator.reply(&self.hotline, input)
    }
}
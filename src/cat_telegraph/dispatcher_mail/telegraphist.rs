use wibr::Make;
use wibr::New;

use crate::cat_depot::train_core::PurrStep;
use crate::cat_depot::train_error::TrainResult;
use crate::cat_depot::train_route::RouteScheduler;
use crate::cat_depot::train_rule::PurrRule;
use crate::cat_depot::train_track::PurrTrack;
use crate::cat_malloc::train::PurrTrain;
use crate::cat_malloc::train_route::RouteBox;
use crate::cat_telegraph::dispatcher::PurrDispatcher;
use crate::cat_telegraph::dispatcher_link::PurrLink;
use crate::cat_telegraph::dispatcher_link::PurrStation;
use crate::cat_telegraph::dispatcher_error::*;
use crate::cat_telegraph::dispatcher_core::*;

use super::back_and_forth::*;

pub type PurrTelegraphistPhantom<'a, T, U, Command: PurrCommand, Reply: PurrReply> = PurrTelegraphist<'a, PhantomRoute<T, U>, Command, Reply>;

pub trait CallAnswerLine<Key, T, U>: PurrHotLine<PurrAnswer<PurrCall<Key, T>, Key, T, U>, Key = Key>
where
    Key: PurrKey,
    T: PurrStep,
    U: PurrRule
{}

impl<Line, Key, T, U> CallAnswerLine<Key, T, U> for Line
where
    Line: PurrHotLine<PurrAnswer<PurrCall<Key, T>, Key, T, U>, Key = Key>,
    Key: PurrKey,
    T: PurrStep,
    U: PurrRule
{}


#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, New)]
pub struct PurrTelegram<Key: PurrKey, Reply: PurrReply> {
    pub key: Key,
    pub reply: Reply
}

impl<Key: PurrKey, Reply: PurrReply> PurrOperatorOutput for PurrTelegram<Key, Reply> { type Key = Key; }

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, Clone, Make)]
pub struct PurrTelegraphist<'a, Route: RouteScheduler, Command: PurrCommand, Reply: PurrReply> {
    #[Some(PurrCallMask::new())] pub blocked_calls: PurrCallMask,
    pub link: &'a PurrLink<Command, Reply>,
    pub purr_route: Option<&'a Route>
}

impl<'a, T, U, Command, Reply> PurrTelegraphistPhantom<'a, T, U, Command, Reply>
where
    Command: PurrCommand,
    Reply: PurrReply,
    T: PurrStep,
    U: PurrRule,
{
    #[inline] pub fn with_phantom(link: &'a PurrLink<Command, Reply>) -> Self { Self::new(link, None) }
}

impl<'a, Cargo, Route, Key, T, U> PurrOperator<Cargo> for PurrTelegraphist<'a, Route, CallCommand<Key, T>, AnswerReply<Key, T, U>>
where
    Cargo: PurrCargo + UnpackCargo<Route>,
    Route: RouteScheduler<Step = T, Rule = U>,
    Key: PurrKey,
    T: PurrStep,
    U: PurrRule
{
    type Output = PurrTelegram<Key, Self::Reply>;

    type Step = T;
    type Rule = U;

    type Command = CallCommand<Key, T>;
    type Reply = AnswerReply<Key, T, U>;

    #[inline]
    fn command<Line>(
        &mut self, dispatcher: &mut PurrDispatcher<Self::Command, Self::Reply, Line, Cargo>, command: Self::Command
    ) -> DispatcherResult<Self::Output> where Line: PurrHotLine<Self::Reply, Key = <Self::Output as PurrOperatorOutput>::Key>, Cargo: PurrCargo<Track: PurrTrack<Step = Self::Step, Rule = Self::Rule>> {
        self.create_output(dispatcher, command)
    }

    #[inline]
    fn reply<Line>(&mut self, hotline: &Line, input: Self::Output) -> DispatcherResult<()> where Line: PurrHotLine<Self::Reply, Key = <Self::Output as PurrOperatorOutput>::Key> {
        hotline.get_route(input.key)?.try_send(input.reply)?;
        Ok(())
    }

    #[inline(never)]
    async fn reply_async<Line>(&mut self, hotline: &Line, input: Self::Output) -> DispatcherResult<()> where Line: PurrHotLine<Self::Reply, Key = <Self::Output as PurrOperatorOutput>::Key> {
        hotline.get_route(input.key)?.send(input.reply).await.map_err(|_| DispatcherError::LineChannelClosed)
    }
}

impl<'a, Route, Key, T, U> PurrTelegraphist<'a, Route, CallCommand<Key, T>, AnswerReply<Key, T, U>>
where
    Route: RouteScheduler<Step = T, Rule = U>,
    Key: PurrKey,
    T: PurrStep,
    U: PurrRule
{
    #[inline] pub fn block_call(&mut self, call: PurrCallKind<T>) { self.blocked_calls.block(call); }
    #[inline] pub fn unblock_call(&mut self, call: PurrCallKind<T>) { self.blocked_calls.unblock(call); }
    #[inline] pub fn block_calls(&mut self, calls: &[PurrCallKind<T>]) { for &call in calls { self.blocked_calls.block(call); }; }
    #[inline] pub fn unblock_calls(&mut self, calls: &[PurrCallKind<T>]) { for &call in calls { self.blocked_calls.unblock(call); }; }
    #[inline] pub fn is_blocked(&self, call: PurrCallKind<T>) -> bool { self.blocked_calls.is_blocked(call) }

    #[inline(never)]
    pub fn create_output<Line, Cargo>(
        &mut self, dispatcher: &mut PurrDispatcher<CallCommand<Key, T>, AnswerReply<Key, T, U>, Line, Cargo>, command: CallCommand<Key, T>
    ) -> DispatcherResult<PurrTelegram<Key, AnswerReply<Key, T, U>>>
        where
            Self: PurrOperator<Cargo, Step = T, Rule = U>,
            Line: CallAnswerLine<Key, T, U>,
            Cargo: PurrCargo<Track: PurrTrack<Step = T, Rule = U>> + UnpackCargo<Route>
    {
        let key = command.key;
        if self.is_blocked(command.kind) { return Err(command.kind.into()); };
        let reply = match command.kind {
            PurrCallKind::CreatePurrStation { channel_cap } => {
                let result = match dispatcher.hotline.add_route(key, channel_cap) {
                    Ok(rx) => Ok(PurrAnswerKind::CreatePurrStation { station: PurrStation::new(self.link.main_channel.clone(), rx, key) }),
                    Err(err) => Err(err)
                };
                PurrAnswer::new(result)
            },
            PurrCallKind::RemovePurrStation => {
                let result = match dispatcher.hotline.delete_route(key) {
                    Ok(tx) => Ok(PurrAnswerKind::RemovePurrStation { tx }),
                    Err(err) => Err(err)
                };
                PurrAnswer::new(result)
            },
            PurrCallKind::Attach { carriage } => {
                unsafe { PurrAnswer::new(self.slice_to_train(|t, s| t.attach_slice(s), dispatcher, carriage, PurrAnswerKind::Attach)) }
            },
            PurrCallKind::Replace { carriage } => {
                unsafe { PurrAnswer::new(self.slice_to_train(|t, s| t.replace_slice(s), dispatcher, carriage, PurrAnswerKind::Replace)) }
            },
            PurrCallKind::RerouteAt { carriage, position } => {
                unsafe { PurrAnswer::new(self.slice_to_train(|t, s| t.reroute_at_slice(s, position), dispatcher, carriage, PurrAnswerKind::RerouteAt)) }
            },
            PurrCallKind::ClearLine => {
                dispatcher.cargo.train_mut().clear_line();
                PurrAnswer::from_kind(PurrAnswerKind::ClearLine)
            },
            PurrCallKind::ShrinkLine { line_length } => {
                dispatcher.cargo.train_mut().shrink_line(line_length);
                PurrAnswer::from_kind(PurrAnswerKind::ShrinkLine)
            },
            PurrCallKind::GetCurrent => {
                PurrAnswer::from_kind(PurrAnswerKind::GetCurrent { current: dispatcher.cargo.train_ref().get_current() })
            },
            PurrCallKind::GetCursor => {
                PurrAnswer::from_kind(PurrAnswerKind::GetCursor { cursor: dispatcher.cargo.train_ref().get_cursor() })
            },
            PurrCallKind::LineLen => {
                PurrAnswer::from_kind(PurrAnswerKind::LineLen { len: dispatcher.cargo.train_ref().line_len() })
            },
            PurrCallKind::LineIsEmpty => {
                PurrAnswer::from_kind(PurrAnswerKind::LineIsEmpty { empty: dispatcher.cargo.train_ref().line_is_empty() } )
            },
            PurrCallKind::LineSnapshot => {
                PurrAnswer::from_kind(PurrAnswerKind::LineSnapshot { snapshot: dispatcher.cargo.train_ref().line_as_slice().to_vec() })
            },
            PurrCallKind::AdvanceTrain => {
                PurrAnswer::from_kind(PurrAnswerKind::AdvanceTrain { event: dispatcher.cargo.train_mut().advance_train() })
            },
        };
        Ok(PurrTelegram::new(key, reply))
    }

    /// # Safety
    /// * `cargo_ref`` and `train_mut`` must not intersect, giving ref to different data
    #[inline(never)]
    pub unsafe fn slice_to_train<Line, Cargo, F>(
        &mut self,
        f: F,
        dispatcher: &mut PurrDispatcher<CallCommand<Key, T>, AnswerReply<Key, T, U>, Line, Cargo>,
        carriage: T,
        kind:  AnswerReplyKind<Key, T, U>
    ) -> DispatcherResult<AnswerReplyKind<Key, T, U>>
        where
            Line: CallAnswerLine<Key, T, U>,
            Cargo: PurrCargo<Track: PurrTrack<Step = T, Rule = U>> + UnpackCargo<Route>,
            F: FnOnce(&mut PurrTrain<<Cargo as PurrCargo>::Track>, &[RouteBox<T, U>]) -> TrainResult<(), T>
    {
        let r = match (self.purr_route, dispatcher.cargo.cargo_ref::<Route>()) {
            (Some(r), _) => Ok(r as *const Route),
            (None, Some(r)) => Ok(r as *const Route),
            (None, None) => Err(DispatcherError::SidingNotFound)
        }?;
        let slice = unsafe { (*r).route_as_slice(carriage) }?;
        match f(dispatcher.cargo.train_mut(), slice) {
            Ok(_) => Ok(kind),
            Err(e) => Err(e.into())
        }
    }
}
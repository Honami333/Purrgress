use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::error::TryRecvError;

use crate::cat_depot::train_core::PurrStep;
use crate::cat_depot::train_error::TrainError;
use crate::cat_depot::train_error::TrainErrorKind;

pub type DispatcherResult<T> = Result<T, DispatcherError>;

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DispatcherError {
    TrainError { error: TrainErrorKind },
    CallBlocked { message: &'static str },
    RouteNotFound,
    DuplicateRoute,
    SidingNotFound,
    HotLineOverFlow,
    MissingCallError,
    LineChannelClosed,
    LinkChannelClosed,
    CommandChannelFull,
    RouteChannelClosed,
    CommandChannelClosed,
    CustomError { message: &'static str }
}
 
impl Display for DispatcherError {
    #[cold]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::TrainError { error } => write!(f, "Train error: {error}"),
            Self::CallBlocked { message } => write!(f, "Call is blocked by the dispatcher: {message}"),
            Self::RouteNotFound => write!(f, "Route not found: no route is registered for the given key"),
            Self::DuplicateRoute => write!(f, "Route addition failed: the key already exists in the fast routes array"),
            Self::SidingNotFound => write!(f, "Siding not found: no route is set on the telegraphist and the cargo does not contain one"),
            Self::HotLineOverFlow => write!(f, "Hot line overflow: all route slots are occupied"),
            Self::MissingCallError => write!(f, "Unknown call: the code does not match any call"),
            Self::LineChannelClosed => write!(f, "Line channel is closed, receiver dropped"),
            Self::LinkChannelClosed => write!(f, "Link channel is closed, all senders dropped"),
            Self::CommandChannelFull => write!(f, "Command channel is full, command was not sent"),
            Self::RouteChannelClosed => write!(f, "Route reply channel is closed, receiver dropped"),
            Self::CommandChannelClosed => write!(f, "Command channel is closed, all senders dropped"),
            Self::CustomError { message } => write!(f, "Custom error: {message}")
        }
    }
}

impl Error for DispatcherError {}

impl<T: PurrStep> From<TrainError<T>> for DispatcherError {
    #[inline]
    fn from(value: TrainError<T>) -> Self {
        DispatcherError::TrainError { error: value.into() }
    }
}

impl<T> From<TrySendError<T>> for DispatcherError {
    #[inline]
    fn from(value: TrySendError<T>) -> Self {
        match value {
            TrySendError::Full(_) => Self::CommandChannelFull,
            TrySendError::Closed(_) => Self::CommandChannelClosed
        }
    }
}

#[inline]
pub fn map_try_recv<V>(result: Result<V, TryRecvError>, error: DispatcherError) -> Option<DispatcherResult<V>>{
    match result {
        Ok(v) => Some(Ok(v)),
        Err(TryRecvError::Disconnected) => Some(Err(error)),
        Err(TryRecvError::Empty) => None
    }
}
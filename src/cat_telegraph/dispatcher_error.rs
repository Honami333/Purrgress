use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::error::TryRecvError;

pub type DispatcherResult<T> = Result<T, DispatcherError>;

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DispatcherError {
    DuplicateRoute,
    RouteNotFound,
    HotLineOverFlow,
    LineChannelClosed,
    LinkChannelClosed,
    RouteChannelClosed,
    CommandChannelClosed,
    CommandChannelFull,
    CustomError { code: u32 }
}

impl Display for DispatcherError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::CommandChannelClosed => write!(f, "Command channel is closed, all senders dropped"),
            Self::CommandChannelFull => write!(f, "Command channel is full, command was not sent"),
            Self::LinkChannelClosed => write!(f, "Link channel is closed, all senders dropped"),
            Self::DuplicateRoute => write!(f, "Route addition failed: The key already exists in the fast routes array."),
            Self::RouteNotFound => write!(f, ""),
            Self::HotLineOverFlow => write!(f, ""),
            Self::LineChannelClosed => write!(f, "Line channel is closed, receiver dropped"),
            Self::RouteChannelClosed => write!(f, "Route reply channel is closed, receiver dropped"),
            Self::CustomError { code } => write!(f, "Custom error code: {code}")
        }
    }
}

impl Error for DispatcherError {}

impl<T> From<TrySendError<T>> for DispatcherError {
    fn from(value: TrySendError<T>) -> Self {
        match value {
            TrySendError::Full(_) => Self::CommandChannelFull,
            TrySendError::Closed(_) => Self::CommandChannelClosed
        }
    }
}

pub fn map_try_recv<V>(result: Result<V, TryRecvError>, error: DispatcherError) -> Option<DispatcherResult<V>>{
    match result {
        Ok(v) => Some(Ok(v)),
        Err(TryRecvError::Disconnected) => Some(Err(error)),
        Err(TryRecvError::Empty) => None
    }
}
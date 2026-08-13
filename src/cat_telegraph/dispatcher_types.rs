use std::fmt::Debug;
use std::hash::Hash;
use std::fmt;
use std::error::Error;

use tokio::sync::mpsc::Receiver;

use bytes::Bytes;

use crate::types::PurrStep;
use crate::types::InsertPosition;

use crate::cat_malloc::train_route::*;
use crate::cat_malloc::train_types::*;

pub const FAST_ROUTES_CAPACITY: usize = 16;


pub trait PurrKey: Debug + Default + PartialEq + Eq + Hash + Clone + Copy {}

impl<T: Debug + Default + PartialEq + Eq + Hash + Clone + Copy> PurrKey for T {}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[derive(Debug, Clone)]
pub enum DispatcherCommand<C> {
    AddRoute { key: C, action: RouteAction, channel_cap: usize },
    DeleteRoute { key: C, action: RouteAction },
    Attach { siding_data: Bytes, key: C },
    Replace { siding_data: Bytes, key: C },
    RerouteAt { siding_data: Bytes, position: InsertPosition, key: C },
    ShrinkLine { line_length: usize, key: C },
    AdvanceTrain { key: C },
    GetCurrent { key: C },
    GetCursor { key: C },
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[derive(Debug)]
pub enum DispatcherReply<T: PurrStep, U: PurrRule> {
    PurrStation { rx_reply: Receiver<DispatcherReply<T, U>> },
    Advance { event: PurrTrainEvent<T, U> },
    Cursor { cursor: usize },
    Current { route_box: Option<RouteBox<T, U>>},
    None
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PurrError {
    DuplicateInFastRoutes(String),
    DuplicateInDynamicRoutes(String),
    CursorNotFound,
    Internal(String),
}

impl fmt::Display for PurrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PurrError::DuplicateInFastRoutes(msg) => write!(f, "Fast Routes Error: {}", msg),
            PurrError::DuplicateInDynamicRoutes(msg) => write!(f, "Dynamic Routes Error: {}", msg),
            PurrError::CursorNotFound => write!(f, "Cursor Error"),
            PurrError::Internal(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl Error for PurrError {}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RouteAction {
    Default,
    AddConfigure { check_duplicates: bool, check_duplicates_migrate: bool, migrate_to_dynamic: bool },
    DeleteConfigure { delete_everywhere: bool, migrate_to_fast: bool },
    GetConfigure { find_all: bool }
}
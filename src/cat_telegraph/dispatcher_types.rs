use std::fmt::Debug;
use std::hash::Hash;
use std::fmt;
use std::error::Error;

use tokio::sync::mpsc::Receiver;

use wibr::MakeFull;

use crate::types::PurrStep;
use crate::types::InsertPosition;

use crate::cat_malloc::train_route::*;
use crate::cat_malloc::train_types::*;

pub const FAST_ROUTES_CAPACITY: usize = 16;


pub trait PurrKey: Debug + Default + PartialEq + Eq + Hash + Copy {}

impl<T: Debug + Default + PartialEq + Eq + Hash + Copy> PurrKey for T {}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub enum DispatcherCommand<T: PurrStep, C: PurrKey> {
    AddRoute { key: C, action: RouteAction, channel_cap: usize },
    DeleteRoute { key: C, action: RouteAction },
    Attach { carriage: T, key: C },
    Replace { carriage: T, key: C },
    RerouteAt { carriage: T, position: InsertPosition, key: C },
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
pub enum DispatcherError {
    DuplicateInFastRoutes,
    DuplicateInDynamicRoutes,
    CursorNotFound,
    LineChannelClosed,
    RouteChannelClosed,
    Internal(String),
}

impl fmt::Display for DispatcherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DispatcherError::DuplicateInFastRoutes => write!(f, "Route addition failed: The key already exists in the fast routes array."),
            DispatcherError::DuplicateInDynamicRoutes => write!(f, "Route addition failed: The key already exists in the dynamic routes map (cross-check)."),
            DispatcherError::CursorNotFound => write!(f, "Cursor Error"),
            DispatcherError::LineChannelClosed => write!(f, "Line channel is closed, receiver dropped"),
            DispatcherError::RouteChannelClosed => write!(f, "Route reply channel is closed, receiver dropped"),
            DispatcherError::Internal(msg) => write!(f, "Internal Error: {}", msg),
            
            
        }
    }
}

impl Error for DispatcherError {}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RouteAction {
    Default,
    AddConfigure { check_duplicates: bool, check_duplicates_migrate: bool, migrate_to_dynamic: bool },
    DeleteConfigure { delete_everywhere: bool, migrate_to_fast: bool },
    GetConfigure { find_all: bool }
}


#[derive(Debug, Clone, Copy, MakeFull)]
pub struct DispatchData<T: PurrStep, C: PurrKey> {
    #[None] pub command: Option<DispatcherCommand<T, C>>,
    #[None] pub key: Option<C>,
    #[None] pub carriage: Option<T>,
    #[None] pub insert_position: Option<InsertPosition>,
    #[None] pub line_length: Option<usize>,
    #[None] pub action: Option<RouteAction>
}
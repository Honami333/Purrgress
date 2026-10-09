use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as RmtResult;
use std::hash::Hash;

use super::train_core::PurrStep;

pub type TrainResult<V, T> = Result<V, TrainError<T>>;


#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrainError<T: PurrStep> {
    RecursiveCarriage { carriage: T },
    UnscheduledParam { carriage: T },
    IndexOutOfBounds { index: usize },
    DuplicateCarriage  { carriage: T },
    CouplingAlreadyDefined,
    EmptyCoupling ,
    StorageOverflow { capacity: usize },
    CustomError { code: u32 }
}

impl<T: PurrStep> Error for TrainError<T> {}

impl<T: PurrStep> Display for TrainError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> RmtResult {
        match self {
            TrainError::RecursiveCarriage { carriage } => write!(f, "Recursive stage error carriage: {:?}", carriage),
            TrainError::UnscheduledParam { carriage } => write!(f, "Couldn't find: {:?}", carriage),
            TrainError::IndexOutOfBounds { index } => write!(f, "Couldn't find parametr by: {:?}", index),
            TrainError::DuplicateCarriage { carriage } => write!(f, "Carriage is already present in the train sequence: {:?}", carriage),
            TrainError::CouplingAlreadyDefined => write!(f, "Coupling configuration is already defined and cannot be overwritten"),
            TrainError::EmptyCoupling => write!(f, "Cannot set coupling: the provided carriage list is empty"),
            TrainError::StorageOverflow { capacity } => write!(f, "Storage overflow: capacity {capacity} exceeded"),
            TrainError::CustomError { code } => write!(f, "Custom error code: {code}")
        }
    }
}
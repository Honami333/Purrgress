use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::hash::Hash;

use super::train_core::PurrStep;

pub type TrainResult<V, T> = Result<V, TrainError<T>>;

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrainError<T: PurrStep> {
    PhantomRoute,
    EmptyCoupling,
    StorageOverflow { capacity: usize },
    IndexOutOfBounds { index: usize },
    UnscheduledParam { carriage: T },
    DuplicateCarriage { carriage: T },
    RecursiveCarriage { carriage: T },
    CouplingAlreadyDefined,
    CustomError { message: &'static str }
}
 
impl<T: PurrStep> Display for TrainError<T> {
    #[cold]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::PhantomRoute => write!(f, "No route available"),
            Self::EmptyCoupling => write!(f, "Cannot set coupling: the provided carriage list is empty"),
            Self::StorageOverflow { capacity } => write!(f, "Storage overflow: capacity {capacity} exceeded"),
            Self::IndexOutOfBounds { index } => write!(f, "Couldn't find parametr by: {:?}", index),
            Self::UnscheduledParam { carriage } => write!(f, "Couldn't find: {:?}", carriage),
            Self::DuplicateCarriage { carriage } => write!(f, "Carriage is already present in the train sequence: {:?}", carriage),
            Self::RecursiveCarriage { carriage } => write!(f, "Recursive stage error carriage: {:?}", carriage),
            Self::CouplingAlreadyDefined => write!(f, "Coupling configuration is already defined and cannot be overwritten"),
            Self::CustomError { message } => write!(f, "Custom error: {message}")
        }
    }
}

impl<T: PurrStep> Error for TrainError<T> {}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrainErrorKind {
    RecursiveCarriage,
    UnscheduledParam,
    IndexOutOfBounds { index: usize },
    DuplicateCarriage,
    PhantomRoute,
    CouplingAlreadyDefined,
    EmptyCoupling,
    StorageOverflow { capacity: usize },
    CustomError { message: &'static str }
}

impl<T: PurrStep> From<TrainError<T>> for TrainErrorKind {
    #[inline]
    fn from(value: TrainError<T>) -> Self {
        match value {
            TrainError::RecursiveCarriage { .. } => TrainErrorKind::RecursiveCarriage,
            TrainError::UnscheduledParam { .. } => TrainErrorKind::UnscheduledParam,
            TrainError::IndexOutOfBounds { index } => TrainErrorKind::IndexOutOfBounds { index },
            TrainError::DuplicateCarriage { .. } => TrainErrorKind::DuplicateCarriage,
            TrainError::CouplingAlreadyDefined => TrainErrorKind::CouplingAlreadyDefined,
            TrainError::EmptyCoupling => TrainErrorKind::EmptyCoupling,
            TrainError::PhantomRoute => TrainErrorKind::PhantomRoute,
            TrainError::StorageOverflow { capacity } => TrainErrorKind::StorageOverflow { capacity },
            TrainError::CustomError { message } => TrainErrorKind::CustomError { message },
        }
    }
}

impl Display for TrainErrorKind {
    #[cold]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::RecursiveCarriage => write!(f, "Recursive stage error carriage"),
            Self::UnscheduledParam => write!(f, "Couldn't find carriage"),
            Self::PhantomRoute => write!(f, "No route available"),
            Self::IndexOutOfBounds { index } => write!(f, "Couldn't find parametr by: {:?}", index),
            Self::DuplicateCarriage => write!(f, "Carriage is already present in the train sequence"),
            Self::CouplingAlreadyDefined => write!(f, "Coupling configuration is already defined and cannot be overwritten"),
            Self::EmptyCoupling => write!(f, "Cannot set coupling: the provided carriage list is empty"),
            Self::StorageOverflow { capacity } => write!(f, "Storage overflow: capacity {capacity} exceeded"),
            Self::CustomError { message } => write!(f, "Custom error: {message}")
        }
    }
}
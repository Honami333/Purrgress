use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as RmtResult;


use crate::types::PurrStep;


pub type TrainResult<V, T> = Result<V, TrainError<T>>;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrainError<T: PurrStep> {
    RecursiveStage { carriage: T },
    UnscheduledParam { carriage: T },
    IndexOutOfBounds { index: usize }
}

impl<T: PurrStep> Error for TrainError<T> {}

impl<T: PurrStep> Display for TrainError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> RmtResult {
        match self {
            TrainError::RecursiveStage { carriage } => write!(f, "Recursive stage error carriage: {:?}", carriage),
            TrainError::UnscheduledParam { carriage } => write!(f, "Couldn't find: {:?}", carriage),
            TrainError::IndexOutOfBounds { index } => write!(f, "Couldn't find parametr by: {:?}", index),
        }
    }
}

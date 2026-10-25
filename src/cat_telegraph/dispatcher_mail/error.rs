use crate::cat_depot::train_core::PurrStep;
use crate::cat_telegraph::dispatcher_error::*;

use super::back_and_forth::PurrCallKind;

impl<T: PurrStep> From<PurrCallKind<T>> for DispatcherError {
    #[inline]
    fn from(value: PurrCallKind<T>) -> Self {
        let message = match value {
            PurrCallKind::CreatePurrStation { .. } => "CreatePurrStation",
            PurrCallKind::RemovePurrStation => "RemovePurrStation",
            PurrCallKind::Attach { .. } => "Attach",
            PurrCallKind::Replace { .. } => "Replace",
            PurrCallKind::RerouteAt { .. } => "RerouteAt",
            PurrCallKind::ClearLine => "ClearLine",
            PurrCallKind::ShrinkLine { .. } => "ShrinkLine",
            PurrCallKind::GetCurrent => "GetCurrent",
            PurrCallKind::GetCursor => "GetCursor",
            PurrCallKind::LineLen => "LineLen",
            PurrCallKind::LineIsEmpty => "LineIsEmpty",
            PurrCallKind::LineSnapshot => "LineSnapshot",
            PurrCallKind::AdvanceTrain => "AdvanceTrain",
        };
        DispatcherError::CallBlocked { message }
    }
}
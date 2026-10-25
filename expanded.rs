#![feature(prelude_import)]
#![warn(missing_docs)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
extern crate self as purrgress;
pub use purrgress_macros::{PurrStep, PurrRule};
pub mod cat_depot {
    pub mod train_conditions {
        use std::fmt::Debug;
        use wibr::Make;
        pub trait PurrCondition: Copy {
            fn is_finished(&self) -> bool;
        }
        pub struct PurrInstant;
        #[automatically_derived]
        impl ::core::fmt::Debug for PurrInstant {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "PurrInstant")
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for PurrInstant {}
        #[automatically_derived]
        impl ::core::clone::Clone for PurrInstant {
            #[inline]
            fn clone(&self) -> PurrInstant {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for PurrInstant {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PurrInstant {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PurrInstant {
            #[inline]
            fn eq(&self, other: &PurrInstant) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for PurrInstant {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {}
        }
        #[automatically_derived]
        impl ::core::hash::Hash for PurrInstant {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {}
        }
        impl PurrCondition for PurrInstant {
            fn is_finished(&self) -> bool {
                true
            }
        }
        pub struct PurrTimer {
            pub duration: f32,
            #[Some(0.0)]
            pub time_elapsed: f32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PurrTimer {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "PurrTimer",
                    "duration",
                    &self.duration,
                    "time_elapsed",
                    &&self.time_elapsed,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for PurrTimer {}
        #[automatically_derived]
        impl ::core::clone::Clone for PurrTimer {
            #[inline]
            fn clone(&self) -> PurrTimer {
                let _: ::core::clone::AssertParamIsClone<f32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for PurrTimer {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PurrTimer {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PurrTimer {
            #[inline]
            fn eq(&self, other: &PurrTimer) -> bool {
                self.duration == other.duration
                    && self.time_elapsed == other.time_elapsed
            }
        }
        impl PurrTimer {
            ///The New function with arguments not baked as None (empty) or Some(expr) (default)
            pub fn new(duration: f32) -> Self {
                Self {
                    duration,
                    time_elapsed: 0.0,
                }
            }
        }
        impl PurrTimer {
            pub fn tick(&mut self, delta: f32) {
                self.time_elapsed += delta;
            }
        }
        impl PurrCondition for PurrTimer {
            fn is_finished(&self) -> bool {
                self.time_elapsed >= self.duration
            }
        }
        pub struct PurrFlag {
            #[Some(false)]
            pub state: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PurrFlag {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "PurrFlag",
                    "state",
                    &&self.state,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for PurrFlag {}
        #[automatically_derived]
        impl ::core::clone::Clone for PurrFlag {
            #[inline]
            fn clone(&self) -> PurrFlag {
                let _: ::core::clone::AssertParamIsClone<bool>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for PurrFlag {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PurrFlag {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PurrFlag {
            #[inline]
            fn eq(&self, other: &PurrFlag) -> bool {
                self.state == other.state
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for PurrFlag {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<bool>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for PurrFlag {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.state, state)
            }
        }
        impl PurrFlag {
            ///The New function with arguments not baked as None (empty) or Some(expr) (default)
            pub fn new() -> Self {
                Self { state: false }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for PurrFlag {
            #[inline]
            fn default() -> PurrFlag {
                PurrFlag {
                    state: ::core::default::Default::default(),
                }
            }
        }
        impl PurrFlag {
            pub fn from_flag(state: bool) -> Self {
                Self { state }
            }
            pub fn set(&mut self, new_state: bool) {
                self.state = new_state;
            }
            pub fn reverse(&mut self) {
                self.state = !self.state;
            }
        }
        impl PurrCondition for PurrFlag {
            fn is_finished(&self) -> bool {
                self.state
            }
        }
    }
    pub mod train_core {
        use std::fmt::Debug;
        use std::hash::Hash;
        use crate::cat_malloc::train_route::RouteBox;
        use super::train_error::{TrainError, TrainResult};
        use super::train_rule::PurrRule;
        pub const DEFAULT_CAPACITY: usize = 16;
        pub trait PurrStep: Debug + Copy + PartialEq + Eq + Hash {}
        pub enum InsertPosition {
            #[default]
            Forward,
            Index(usize),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for InsertPosition {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    InsertPosition::Forward => {
                        ::core::fmt::Formatter::write_str(f, "Forward")
                    }
                    InsertPosition::Index(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Index",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for InsertPosition {}
        #[automatically_derived]
        impl ::core::clone::Clone for InsertPosition {
            #[inline]
            fn clone(&self) -> InsertPosition {
                let _: ::core::clone::AssertParamIsClone<usize>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for InsertPosition {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for InsertPosition {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for InsertPosition {
            #[inline]
            fn eq(&self, other: &InsertPosition) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            InsertPosition::Index(__self_0),
                            InsertPosition::Index(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for InsertPosition {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<usize>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for InsertPosition {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state);
                match self {
                    InsertPosition::Index(__self_0) => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for InsertPosition {
            #[inline]
            fn default() -> InsertPosition {
                Self::Forward
            }
        }
        impl InsertPosition {
            pub fn get(self, index: usize, max_index: usize) -> usize {
                let index = match self {
                    InsertPosition::Forward => index,
                    InsertPosition::Index(i) => i.saturating_add(index),
                };
                index.min(max_index)
            }
        }
        pub enum SaveMode {
            #[default]
            Clear,
            Keep,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SaveMode {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        SaveMode::Clear => "Clear",
                        SaveMode::Keep => "Keep",
                    },
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for SaveMode {}
        #[automatically_derived]
        impl ::core::clone::Clone for SaveMode {
            #[inline]
            fn clone(&self) -> SaveMode {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for SaveMode {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SaveMode {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SaveMode {
            #[inline]
            fn eq(&self, other: &SaveMode) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for SaveMode {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {}
        }
        #[automatically_derived]
        impl ::core::hash::Hash for SaveMode {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for SaveMode {
            #[inline]
            fn default() -> SaveMode {
                Self::Clear
            }
        }
        pub enum DuplicatePolicy {
            #[default]
            Overwrite,
            Ignore,
            RaiseError,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DuplicatePolicy {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        DuplicatePolicy::Overwrite => "Overwrite",
                        DuplicatePolicy::Ignore => "Ignore",
                        DuplicatePolicy::RaiseError => "RaiseError",
                    },
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for DuplicatePolicy {}
        #[automatically_derived]
        impl ::core::clone::Clone for DuplicatePolicy {
            #[inline]
            fn clone(&self) -> DuplicatePolicy {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for DuplicatePolicy {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DuplicatePolicy {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DuplicatePolicy {
            #[inline]
            fn eq(&self, other: &DuplicatePolicy) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for DuplicatePolicy {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {}
        }
        #[automatically_derived]
        impl ::core::hash::Hash for DuplicatePolicy {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DuplicatePolicy {
            #[inline]
            fn default() -> DuplicatePolicy {
                Self::Overwrite
            }
        }
        impl DuplicatePolicy {
            pub fn should_overwrite<T: PurrStep>(
                self,
                carriage: T,
            ) -> TrainResult<bool, T> {
                match self {
                    DuplicatePolicy::RaiseError => {
                        Err(TrainError::DuplicateCarriage {
                            carriage,
                        })
                    }
                    DuplicatePolicy::Ignore => Ok(false),
                    DuplicatePolicy::Overwrite => Ok(true),
                }
            }
        }
        pub enum ChangeRuleCount {
            #[default]
            All,
            Count(usize),
            First,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ChangeRuleCount {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ChangeRuleCount::All => ::core::fmt::Formatter::write_str(f, "All"),
                    ChangeRuleCount::Count(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Count",
                            &__self_0,
                        )
                    }
                    ChangeRuleCount::First => {
                        ::core::fmt::Formatter::write_str(f, "First")
                    }
                }
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for ChangeRuleCount {}
        #[automatically_derived]
        impl ::core::clone::Clone for ChangeRuleCount {
            #[inline]
            fn clone(&self) -> ChangeRuleCount {
                let _: ::core::clone::AssertParamIsClone<usize>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ChangeRuleCount {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ChangeRuleCount {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ChangeRuleCount {
            #[inline]
            fn eq(&self, other: &ChangeRuleCount) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            ChangeRuleCount::Count(__self_0),
                            ChangeRuleCount::Count(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ChangeRuleCount {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<usize>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for ChangeRuleCount {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state);
                match self {
                    ChangeRuleCount::Count(__self_0) => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ChangeRuleCount {
            #[inline]
            fn default() -> ChangeRuleCount {
                Self::All
            }
        }
        impl ChangeRuleCount {
            pub fn is_final_rule_count(&mut self) -> bool {
                match self {
                    ChangeRuleCount::First => true,
                    ChangeRuleCount::Count(count) => {
                        *count = count.saturating_sub(1);
                        *count == 0
                    }
                    ChangeRuleCount::All => false,
                }
            }
        }
        pub enum PurrTrainEvent<T: PurrStep, U: PurrRule> {
            Idle,
            Running(RouteBox<T, U>),
            Transition { from: RouteBox<T, U>, to: Option<RouteBox<T, U>> },
        }
        #[automatically_derived]
        impl<
            T: ::core::fmt::Debug + PurrStep,
            U: ::core::fmt::Debug + PurrRule,
        > ::core::fmt::Debug for PurrTrainEvent<T, U> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    PurrTrainEvent::Idle => ::core::fmt::Formatter::write_str(f, "Idle"),
                    PurrTrainEvent::Running(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Running",
                            &__self_0,
                        )
                    }
                    PurrTrainEvent::Transition { from: __self_0, to: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "Transition",
                            "from",
                            __self_0,
                            "to",
                            &__self_1,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::clone::Clone + PurrStep,
            U: ::core::clone::Clone + PurrRule,
        > ::core::clone::Clone for PurrTrainEvent<T, U> {
            #[inline]
            fn clone(&self) -> PurrTrainEvent<T, U> {
                match self {
                    PurrTrainEvent::Idle => PurrTrainEvent::Idle,
                    PurrTrainEvent::Running(__self_0) => {
                        PurrTrainEvent::Running(::core::clone::Clone::clone(__self_0))
                    }
                    PurrTrainEvent::Transition { from: __self_0, to: __self_1 } => {
                        PurrTrainEvent::Transition {
                            from: ::core::clone::Clone::clone(__self_0),
                            to: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                }
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::marker::Copy + PurrStep,
            U: ::core::marker::Copy + PurrRule,
        > ::core::marker::Copy for PurrTrainEvent<T, U> {}
        #[automatically_derived]
        impl<
            T: ::core::cmp::PartialEq + PurrStep,
            U: ::core::cmp::PartialEq + PurrRule,
        > ::core::marker::StructuralPartialEq for PurrTrainEvent<T, U> {}
        #[automatically_derived]
        impl<
            T: ::core::cmp::PartialEq + PurrStep,
            U: ::core::cmp::PartialEq + PurrRule,
        > ::core::cmp::PartialEq for PurrTrainEvent<T, U> {
            #[inline]
            fn eq(&self, other: &PurrTrainEvent<T, U>) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            PurrTrainEvent::Running(__self_0),
                            PurrTrainEvent::Running(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            PurrTrainEvent::Transition { from: __self_0, to: __self_1 },
                            PurrTrainEvent::Transition { from: __arg1_0, to: __arg1_1 },
                        ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::cmp::Eq + PurrStep,
            U: ::core::cmp::Eq + PurrRule,
        > ::core::cmp::Eq for PurrTrainEvent<T, U> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<RouteBox<T, U>>;
                let _: ::core::cmp::AssertParamIsEq<RouteBox<T, U>>;
                let _: ::core::cmp::AssertParamIsEq<Option<RouteBox<T, U>>>;
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::hash::Hash + PurrStep,
            U: ::core::hash::Hash + PurrRule,
        > ::core::hash::Hash for PurrTrainEvent<T, U> {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state);
                match self {
                    PurrTrainEvent::Running(__self_0) => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    PurrTrainEvent::Transition { from: __self_0, to: __self_1 } => {
                        ::core::hash::Hash::hash(__self_0, state);
                        ::core::hash::Hash::hash(__self_1, state)
                    }
                    _ => {}
                }
            }
        }
    }
    pub mod train_error {
        use std::error::Error;
        use std::fmt::Display;
        use std::fmt::Formatter;
        use std::fmt::Result as FmtResult;
        use std::hash::Hash;
        use super::train_core::PurrStep;
        pub type TrainResult<V, T> = Result<V, TrainError<T>>;
        pub enum TrainError<T: PurrStep> {
            RecursiveCarriage { carriage: T },
            UnscheduledParam { carriage: T },
            IndexOutOfBounds { index: usize },
            DuplicateCarriage { carriage: T },
            CouplingAlreadyDefined,
            EmptyCoupling,
            StorageOverflow { capacity: usize },
            CustomError { code: u32 },
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug + PurrStep> ::core::fmt::Debug for TrainError<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    TrainError::RecursiveCarriage { carriage: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "RecursiveCarriage",
                            "carriage",
                            &__self_0,
                        )
                    }
                    TrainError::UnscheduledParam { carriage: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "UnscheduledParam",
                            "carriage",
                            &__self_0,
                        )
                    }
                    TrainError::IndexOutOfBounds { index: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "IndexOutOfBounds",
                            "index",
                            &__self_0,
                        )
                    }
                    TrainError::DuplicateCarriage { carriage: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "DuplicateCarriage",
                            "carriage",
                            &__self_0,
                        )
                    }
                    TrainError::CouplingAlreadyDefined => {
                        ::core::fmt::Formatter::write_str(f, "CouplingAlreadyDefined")
                    }
                    TrainError::EmptyCoupling => {
                        ::core::fmt::Formatter::write_str(f, "EmptyCoupling")
                    }
                    TrainError::StorageOverflow { capacity: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "StorageOverflow",
                            "capacity",
                            &__self_0,
                        )
                    }
                    TrainError::CustomError { code: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "CustomError",
                            "code",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl<T: ::core::clone::Clone + PurrStep> ::core::clone::Clone for TrainError<T> {
            #[inline]
            fn clone(&self) -> TrainError<T> {
                match self {
                    TrainError::RecursiveCarriage { carriage: __self_0 } => {
                        TrainError::RecursiveCarriage {
                            carriage: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    TrainError::UnscheduledParam { carriage: __self_0 } => {
                        TrainError::UnscheduledParam {
                            carriage: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    TrainError::IndexOutOfBounds { index: __self_0 } => {
                        TrainError::IndexOutOfBounds {
                            index: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    TrainError::DuplicateCarriage { carriage: __self_0 } => {
                        TrainError::DuplicateCarriage {
                            carriage: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    TrainError::CouplingAlreadyDefined => {
                        TrainError::CouplingAlreadyDefined
                    }
                    TrainError::EmptyCoupling => TrainError::EmptyCoupling,
                    TrainError::StorageOverflow { capacity: __self_0 } => {
                        TrainError::StorageOverflow {
                            capacity: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    TrainError::CustomError { code: __self_0 } => {
                        TrainError::CustomError {
                            code: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                }
            }
        }
        #[automatically_derived]
        impl<T: ::core::marker::Copy + PurrStep> ::core::marker::Copy for TrainError<T> {}
        #[automatically_derived]
        impl<T: ::core::cmp::PartialEq + PurrStep> ::core::marker::StructuralPartialEq
        for TrainError<T> {}
        #[automatically_derived]
        impl<T: ::core::cmp::PartialEq + PurrStep> ::core::cmp::PartialEq
        for TrainError<T> {
            #[inline]
            fn eq(&self, other: &TrainError<T>) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            TrainError::RecursiveCarriage { carriage: __self_0 },
                            TrainError::RecursiveCarriage { carriage: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        (
                            TrainError::UnscheduledParam { carriage: __self_0 },
                            TrainError::UnscheduledParam { carriage: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        (
                            TrainError::IndexOutOfBounds { index: __self_0 },
                            TrainError::IndexOutOfBounds { index: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        (
                            TrainError::DuplicateCarriage { carriage: __self_0 },
                            TrainError::DuplicateCarriage { carriage: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        (
                            TrainError::StorageOverflow { capacity: __self_0 },
                            TrainError::StorageOverflow { capacity: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        (
                            TrainError::CustomError { code: __self_0 },
                            TrainError::CustomError { code: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl<T: ::core::cmp::Eq + PurrStep> ::core::cmp::Eq for TrainError<T> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<T>;
                let _: ::core::cmp::AssertParamIsEq<usize>;
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl<T: ::core::hash::Hash + PurrStep> ::core::hash::Hash for TrainError<T> {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state);
                match self {
                    TrainError::RecursiveCarriage { carriage: __self_0 } => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    TrainError::UnscheduledParam { carriage: __self_0 } => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    TrainError::IndexOutOfBounds { index: __self_0 } => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    TrainError::DuplicateCarriage { carriage: __self_0 } => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    TrainError::StorageOverflow { capacity: __self_0 } => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    TrainError::CustomError { code: __self_0 } => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    _ => {}
                }
            }
        }
        impl<T: PurrStep> Error for TrainError<T> {}
        impl<T: PurrStep> Display for TrainError<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                match self {
                    Self::RecursiveCarriage { carriage } => {
                        f.write_fmt(
                            format_args!(
                                "Recursive stage error carriage: {0:?}",
                                carriage,
                            ),
                        )
                    }
                    Self::UnscheduledParam { carriage } => {
                        f.write_fmt(format_args!("Couldn\'t find: {0:?}", carriage))
                    }
                    Self::IndexOutOfBounds { index } => {
                        f.write_fmt(
                            format_args!("Couldn\'t find parametr by: {0:?}", index),
                        )
                    }
                    Self::DuplicateCarriage { carriage } => {
                        f.write_fmt(
                            format_args!(
                                "Carriage is already present in the train sequence: {0:?}",
                                carriage,
                            ),
                        )
                    }
                    Self::CouplingAlreadyDefined => {
                        f.write_fmt(
                            format_args!(
                                "Coupling configuration is already defined and cannot be overwritten",
                            ),
                        )
                    }
                    Self::EmptyCoupling => {
                        f.write_fmt(
                            format_args!(
                                "Cannot set coupling: the provided carriage list is empty",
                            ),
                        )
                    }
                    Self::StorageOverflow { capacity } => {
                        f.write_fmt(
                            format_args!(
                                "Storage overflow: capacity {0} exceeded",
                                capacity,
                            ),
                        )
                    }
                    Self::CustomError { code } => {
                        f.write_fmt(format_args!("Custom error code: {0}", code))
                    }
                }
            }
        }
    }
    pub mod train_route {
        use crate::cat_malloc::train_siding::PurrSiding;
        use super::train_core::PurrStep;
        use super::train_rule::PurrRule;
        use super::train_error::TrainResult;
        pub trait RouteScheduler {
            type Step: PurrStep;
            type Rule: PurrRule;
            fn load(
                &self,
                purr_siding: &mut PurrSiding<Self::Step, Self::Rule>,
                carriage: Self::Step,
            ) -> TrainResult<(), Self::Step>;
        }
    }
    pub mod train_rule {
        use std::fmt::Debug;
        use crate::PurrRule;
        use super::train_conditions;
        pub enum PurrRuleLite {
            Instant(train_conditions::PurrInstant),
            Flag(train_conditions::PurrFlag),
            Timer(train_conditions::PurrTimer),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PurrRuleLite {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    PurrRuleLite::Instant(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Instant",
                            &__self_0,
                        )
                    }
                    PurrRuleLite::Flag(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Flag",
                            &__self_0,
                        )
                    }
                    PurrRuleLite::Timer(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Timer",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for PurrRuleLite {}
        #[automatically_derived]
        impl ::core::clone::Clone for PurrRuleLite {
            #[inline]
            fn clone(&self) -> PurrRuleLite {
                let _: ::core::clone::AssertParamIsClone<train_conditions::PurrInstant>;
                let _: ::core::clone::AssertParamIsClone<train_conditions::PurrFlag>;
                let _: ::core::clone::AssertParamIsClone<train_conditions::PurrTimer>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for PurrRuleLite {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PurrRuleLite {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PurrRuleLite {
            #[inline]
            fn eq(&self, other: &PurrRuleLite) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            PurrRuleLite::Instant(__self_0),
                            PurrRuleLite::Instant(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (PurrRuleLite::Flag(__self_0), PurrRuleLite::Flag(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (
                            PurrRuleLite::Timer(__self_0),
                            PurrRuleLite::Timer(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl purrgress::cat_depot::train_rule::UnpackRule<train_conditions::PurrInstant>
        for PurrRuleLite {
            fn unpack(self) -> Option<train_conditions::PurrInstant> {
                if let Self::Instant(field) = self { Some(field) } else { None }
            }
            fn unpack_ref(&self) -> Option<&train_conditions::PurrInstant> {
                if let Self::Instant(field) = self { Some(field) } else { None }
            }
            fn unpack_mut(&mut self) -> Option<&mut train_conditions::PurrInstant> {
                if let Self::Instant(field) = self { Some(field) } else { None }
            }
        }
        #[automatically_derived]
        impl From<train_conditions::PurrInstant> for PurrRuleLite {
            fn from(value: train_conditions::PurrInstant) -> Self {
                Self::Instant(value)
            }
        }
        #[automatically_derived]
        impl purrgress::cat_depot::train_rule::UnpackRule<train_conditions::PurrFlag>
        for PurrRuleLite {
            fn unpack(self) -> Option<train_conditions::PurrFlag> {
                if let Self::Flag(field) = self { Some(field) } else { None }
            }
            fn unpack_ref(&self) -> Option<&train_conditions::PurrFlag> {
                if let Self::Flag(field) = self { Some(field) } else { None }
            }
            fn unpack_mut(&mut self) -> Option<&mut train_conditions::PurrFlag> {
                if let Self::Flag(field) = self { Some(field) } else { None }
            }
        }
        #[automatically_derived]
        impl From<train_conditions::PurrFlag> for PurrRuleLite {
            fn from(value: train_conditions::PurrFlag) -> Self {
                Self::Flag(value)
            }
        }
        #[automatically_derived]
        impl purrgress::cat_depot::train_rule::UnpackRule<train_conditions::PurrTimer>
        for PurrRuleLite {
            fn unpack(self) -> Option<train_conditions::PurrTimer> {
                if let Self::Timer(field) = self { Some(field) } else { None }
            }
            fn unpack_ref(&self) -> Option<&train_conditions::PurrTimer> {
                if let Self::Timer(field) = self { Some(field) } else { None }
            }
            fn unpack_mut(&mut self) -> Option<&mut train_conditions::PurrTimer> {
                if let Self::Timer(field) = self { Some(field) } else { None }
            }
        }
        #[automatically_derived]
        impl From<train_conditions::PurrTimer> for PurrRuleLite {
            fn from(value: train_conditions::PurrTimer) -> Self {
                Self::Timer(value)
            }
        }
        #[automatically_derived]
        impl purrgress::cat_depot::train_rule::PurrRule for PurrRuleLite {
            fn is_finished(&self) -> bool {
                match self {
                    Self::Instant(field) => {
                        <train_conditions::PurrInstant as purrgress::cat_depot::train_conditions::PurrCondition>::is_finished(
                            field,
                        )
                    }
                    Self::Flag(field) => {
                        <train_conditions::PurrFlag as purrgress::cat_depot::train_conditions::PurrCondition>::is_finished(
                            field,
                        )
                    }
                    Self::Timer(field) => {
                        <train_conditions::PurrTimer as purrgress::cat_depot::train_conditions::PurrCondition>::is_finished(
                            field,
                        )
                    }
                }
            }
        }
        impl PurrRuleLite {
            pub fn instant() -> Self {
                Self::Instant(train_conditions::PurrInstant)
            }
            pub fn timer(duration: f32) -> Self {
                Self::Timer(train_conditions::PurrTimer::new(duration))
            }
            pub fn flag() -> Self {
                Self::Flag(train_conditions::PurrFlag::new())
            }
        }
        pub trait PurrRule: PartialEq + Copy {
            fn is_finished(&self) -> bool;
            fn as_rule<C: Copy>(&self) -> Option<C>
            where
                Self: UnpackRule<C>,
            {
                self.unpack()
            }
            fn as_ref_rule<C: Copy>(&self) -> Option<&C>
            where
                Self: UnpackRule<C>,
            {
                self.unpack_ref()
            }
            fn as_mut_rule<C: Copy>(&mut self) -> Option<&mut C>
            where
                Self: UnpackRule<C>,
            {
                self.unpack_mut()
            }
        }
        pub trait UnpackRule<C: Copy> {
            fn unpack(self) -> Option<C>;
            fn unpack_ref(&self) -> Option<&C>;
            fn unpack_mut(&mut self) -> Option<&mut C>;
        }
    }
    pub mod train_track {
        use std::ops::RangeBounds;
        use crate::cat_malloc::train_route::RouteBox;
        use super::train_core::PurrStep;
        use super::train_error::TrainResult;
        use super::train_rule::PurrRule;
        pub trait PurrTrack {
            type Step: PurrStep;
            type Rule: PurrRule;
            fn tr_new() -> Self;
            fn tr_with_capacity(capacity: usize) -> Self;
            fn tr_get_cursor(&self) -> usize;
            fn tr_set_cursor(&mut self, index: usize) -> TrainResult<(), Self::Step>;
            fn tr_step_cursor(&mut self) -> TrainResult<(), Self::Step>;
            fn tr_get(&self, index: usize) -> Option<RouteBox<Self::Step, Self::Rule>>;
            fn tr_get_ref(
                &self,
                index: usize,
            ) -> Option<&RouteBox<Self::Step, Self::Rule>>;
            fn tr_get_mut(
                &mut self,
                index: usize,
            ) -> Option<&mut RouteBox<Self::Step, Self::Rule>>;
            fn tr_get_current(&self) -> Option<RouteBox<Self::Step, Self::Rule>>;
            fn tr_get_current_ref(&self) -> Option<&RouteBox<Self::Step, Self::Rule>>;
            fn tr_get_current_mut(
                &mut self,
            ) -> Option<&mut RouteBox<Self::Step, Self::Rule>>;
            fn tr_as_slice(&self) -> &[RouteBox<Self::Step, Self::Rule>];
            fn tr_as_mut_slice(&mut self) -> &mut [RouteBox<Self::Step, Self::Rule>];
            fn tr_extend<I: IntoIterator<Item = RouteBox<Self::Step, Self::Rule>>>(
                &mut self,
                iter: I,
            ) -> TrainResult<(), Self::Step>;
            fn tr_splice<
                'a,
                I: IntoIterator<Item = RouteBox<Self::Step, Self::Rule>> + 'a,
            >(
                &'a mut self,
                range: impl RangeBounds<usize>,
                replace_with: I,
            ) -> TrainResult<
                impl Iterator<Item = RouteBox<Self::Step, Self::Rule>>,
                Self::Step,
            >;
            fn tr_drain(
                &mut self,
                range: impl RangeBounds<usize>,
            ) -> TrainResult<
                impl Iterator<Item = RouteBox<Self::Step, Self::Rule>>,
                Self::Step,
            >;
            fn tr_is_last(&self) -> bool;
            fn tr_len(&self) -> usize;
            fn tr_is_empty(&self) -> bool;
            fn tr_clear(&mut self);
        }
    }
}
pub mod cat_malloc {
    pub mod train {
        use wibr::New;
        use crate::cat_depot::train_cursor::PurrCursor;
        use crate::cat_depot::train_error::TrainResult;
        use crate::cat_depot::train_track::PurrTrack;
        use crate::cat_depot::train_rule::PurrRule;
        use crate::cat_depot::train_core::*;
        use super::train_route::RouteBox;
        use super::train_siding::PurrSiding;
        pub type CursorTrain<T, U> = PurrTrain<PurrCursor<RouteBox<T, U>>>;
        pub struct PurrTrain<S> {
            pub line: S,
        }
        #[automatically_derived]
        impl<S: ::core::fmt::Debug> ::core::fmt::Debug for PurrTrain<S> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "PurrTrain",
                    "line",
                    &&self.line,
                )
            }
        }
        #[automatically_derived]
        impl<S: ::core::clone::Clone> ::core::clone::Clone for PurrTrain<S> {
            #[inline]
            fn clone(&self) -> PurrTrain<S> {
                PurrTrain {
                    line: ::core::clone::Clone::clone(&self.line),
                }
            }
        }
        #[automatically_derived]
        impl<S: ::core::marker::Copy> ::core::marker::Copy for PurrTrain<S> {}
        #[automatically_derived]
        impl<S: ::core::cmp::PartialEq> ::core::marker::StructuralPartialEq
        for PurrTrain<S> {}
        #[automatically_derived]
        impl<S: ::core::cmp::PartialEq> ::core::cmp::PartialEq for PurrTrain<S> {
            #[inline]
            fn eq(&self, other: &PurrTrain<S>) -> bool {
                self.line == other.line
            }
        }
        #[automatically_derived]
        impl<S: ::core::cmp::Eq> ::core::cmp::Eq for PurrTrain<S> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<S>;
            }
        }
        #[automatically_derived]
        impl<S: ::core::hash::Hash> ::core::hash::Hash for PurrTrain<S> {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.line, state)
            }
        }
        impl<S> PurrTrain<S> {
            ///The New function with all structure arguments
            pub fn new(line: S) -> Self {
                Self { line }
            }
        }
        impl<S> PurrTrain<S>
        where
            S: PurrTrack,
        {
            pub fn get_current(&self) -> Option<RouteBox<S::Step, S::Rule>> {
                self.line.tr_get_current()
            }
            pub fn get_current_ref(&self) -> Option<&RouteBox<S::Step, S::Rule>> {
                self.line.tr_get_current_ref()
            }
            pub fn get_current_mut(
                &mut self,
            ) -> Option<&mut RouteBox<S::Step, S::Rule>> {
                self.line.tr_get_current_mut()
            }
            pub fn get_cursor(&self) -> usize {
                self.line.tr_get_cursor()
            }
            pub fn line_as_slice(&self) -> &[RouteBox<S::Step, S::Rule>] {
                self.line.tr_as_slice()
            }
            pub fn line_as_mut_slice(&mut self) -> &mut [RouteBox<S::Step, S::Rule>] {
                self.line.tr_as_mut_slice()
            }
            pub fn line_is_empty(&self) -> bool {
                self.line.tr_is_empty()
            }
            pub fn line_len(&self) -> usize {
                self.line.tr_len()
            }
            pub fn clear_line(&mut self) {
                self.line.tr_clear();
            }
            pub fn attach(
                &mut self,
                purr_siding: &mut PurrSiding<S::Step, S::Rule>,
            ) -> TrainResult<(), S::Step> {
                self.line.tr_extend(purr_siding.line.drain(..))
            }
            pub fn attach_slice(
                &mut self,
                slice: &[RouteBox<S::Step, S::Rule>],
            ) -> TrainResult<(), S::Step> {
                self.line.tr_extend(slice.iter().copied())
            }
            pub fn replace(
                &mut self,
                purr_siding: &mut PurrSiding<S::Step, S::Rule>,
            ) -> TrainResult<(), S::Step> {
                self.clear_line();
                self.attach(purr_siding)
            }
            pub fn replace_slice(
                &mut self,
                slice: &[RouteBox<S::Step, S::Rule>],
            ) -> TrainResult<(), S::Step> {
                self.clear_line();
                self.attach_slice(slice)
            }
            pub fn reroute_at(
                &mut self,
                purr_siding: &mut PurrSiding<S::Step, S::Rule>,
                insert_position: InsertPosition,
            ) -> TrainResult<(), S::Step> {
                let cursor = self.line.tr_get_cursor();
                let index = insert_position.get(cursor, self.line_len());
                drop(self.line.tr_splice(index..index, purr_siding.line.drain(..))?);
                Ok(())
            }
            pub fn reroute_at_slice(
                &mut self,
                slice: &[RouteBox<S::Step, S::Rule>],
                insert_position: InsertPosition,
            ) -> TrainResult<(), S::Step> {
                let cursor = self.line.tr_get_cursor();
                let index = insert_position.get(cursor, self.line_len());
                drop(self.line.tr_splice(index..index, slice.iter().copied())?);
                Ok(())
            }
            pub fn shrink_line(&mut self, max_cursor: usize) {
                let cursor = self.line.tr_get_cursor();
                if cursor > max_cursor {
                    drop(self.line.tr_drain(0..cursor));
                    let stepped = self.line.tr_set_cursor(0);
                    if true {
                        if !stepped.is_ok() {
                            ::core::panicking::panic("assertion failed: stepped.is_ok()")
                        }
                    }
                }
            }
            pub fn advance_train(&mut self) -> PurrTrainEvent<S::Step, S::Rule> {
                if let Some(route_box) = self.get_current() {
                    if route_box.rule.is_finished() {
                        if self.line.tr_is_last() {
                            self.clear_line();
                            return PurrTrainEvent::Transition {
                                from: route_box,
                                to: None,
                            };
                        }
                        let stepped = self.line.tr_step_cursor();
                        if true {
                            if !stepped.is_ok() {
                                ::core::panicking::panic(
                                    "assertion failed: stepped.is_ok()",
                                )
                            }
                        }
                        let to_route_box = self.get_current();
                        return PurrTrainEvent::Transition {
                            from: route_box,
                            to: to_route_box,
                        };
                    }
                    return PurrTrainEvent::Running(route_box);
                }
                PurrTrainEvent::Idle
            }
        }
    }
    pub mod train_design {
        use std::collections::HashMap;
        use wibr::Make;
        use crate::cat_depot::train_rule::PurrRule;
        use crate::cat_depot::train_core::*;
        use crate::cat_depot::train_error::*;
        /// **PurrDesign** is a raw draft that stores blueprints.
        ///
        /// It is intended for convenient (human-friendly) addition of stages and nested chains.
        /// Each stage is bound to its corresponding condition and an optional chain
        pub struct PurrDesign<T: PurrStep, U: PurrRule> {
            /// blueprints is a storage for all drafts for specific carriages in a human-friendly format
            /// Direct interaction with the field removes verification for carriage overwrites and invalid coupling states of DesignBox
            /// Pub for raw interaction with blueprints and writing custom logic for PurrDesign
            #[Some(HashMap::with_capacity(DEFAULT_CAPACITY))]
            pub blueprints: HashMap<T, DesignBox<T, U>>,
        }
        #[automatically_derived]
        impl<
            T: ::core::fmt::Debug + PurrStep,
            U: ::core::fmt::Debug + PurrRule,
        > ::core::fmt::Debug for PurrDesign<T, U> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "PurrDesign",
                    "blueprints",
                    &&self.blueprints,
                )
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::clone::Clone + PurrStep,
            U: ::core::clone::Clone + PurrRule,
        > ::core::clone::Clone for PurrDesign<T, U> {
            #[inline]
            fn clone(&self) -> PurrDesign<T, U> {
                PurrDesign {
                    blueprints: ::core::clone::Clone::clone(&self.blueprints),
                }
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::cmp::PartialEq + PurrStep,
            U: ::core::cmp::PartialEq + PurrRule,
        > ::core::marker::StructuralPartialEq for PurrDesign<T, U> {}
        #[automatically_derived]
        impl<
            T: ::core::cmp::PartialEq + PurrStep,
            U: ::core::cmp::PartialEq + PurrRule,
        > ::core::cmp::PartialEq for PurrDesign<T, U> {
            #[inline]
            fn eq(&self, other: &PurrDesign<T, U>) -> bool {
                self.blueprints == other.blueprints
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::cmp::Eq + PurrStep,
            U: ::core::cmp::Eq + PurrRule,
        > ::core::cmp::Eq for PurrDesign<T, U> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<HashMap<T, DesignBox<T, U>>>;
            }
        }
        impl<T: PurrStep, U: PurrRule> PurrDesign<T, U> {
            ///The New function with arguments not baked as None (empty) or Some(expr) (default)
            pub fn new() -> Self {
                Self {
                    blueprints: HashMap::with_capacity(DEFAULT_CAPACITY),
                }
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::default::Default + PurrStep,
            U: ::core::default::Default + PurrRule,
        > ::core::default::Default for PurrDesign<T, U> {
            #[inline]
            fn default() -> PurrDesign<T, U> {
                PurrDesign {
                    blueprints: ::core::default::Default::default(),
                }
            }
        }
        impl<T, U> PurrDesign<T, U>
        where
            T: PurrStep,
            U: PurrRule,
        {
            /// Raw method for adding single to blueprints
            ///
            /// # Warning
            /// If the carriage is already in blueprints, the method will overwrite it
            pub fn single(&mut self, carriage: T, rule: U) {
                let design = DesignBox::new(rule);
                self.add_blueprints(carriage, design);
            }
            /// Raw method for adding chain to blueprints
            ///
            /// # Warning
            /// If the carriage is already in blueprints, the method will overwrite it
            pub fn chain(&mut self, carriage: T, rule: U, coupling: Vec<T>) {
                let mut design = DesignBox::new(rule);
                design.set_coupling(coupling);
                self.add_blueprints(carriage, design);
            }
            /// The method attempts to add single to blueprints
            ///
            /// # Errors
            /// * `TrainError::DuplicateCarriage { carriage: T }` - if the carriage is already in blueprints
            pub fn try_single(&mut self, carriage: T, rule: U) -> TrainResult<(), T> {
                if self.contains_carriage(carriage) {
                    return Err(TrainError::DuplicateCarriage {
                        carriage,
                    });
                }
                self.single(carriage, rule);
                Ok(())
            }
            /// The method attempts to add chain to blueprints
            ///
            /// # Errors
            /// * `TrainError::DuplicateCarriage { carriage: T }` - if the carriage is already in blueprints
            /// * `TrainError::EmptyCoupling` - if the passed coupling contains no carriage
            pub fn try_chain(
                &mut self,
                carriage: T,
                rule: U,
                coupling: Vec<T>,
            ) -> TrainResult<(), T> {
                if self.contains_carriage(carriage) {
                    return Err(TrainError::DuplicateCarriage {
                        carriage,
                    });
                }
                let mut design = DesignBox::new(rule);
                design.validated_set_coupling(coupling)?;
                self.add_blueprints(carriage, design);
                Ok(())
            }
            /// Raw method for adding DesignBox to blueprints
            ///
            /// # Warning
            /// If the carriage is already in blueprints, the method will overwrite it
            pub fn add_blueprints(&mut self, carriage: T, design: DesignBox<T, U>) {
                self.blueprints.insert(carriage, design);
            }
            /// Checks for the presence of the carriage in blueprints
            pub fn contains_carriage(&self, carriage: T) -> bool {
                self.blueprints.contains_key(&carriage)
            }
            /// Completely clears blueprints
            pub fn clear(&mut self) {
                self.blueprints.clear();
            }
        }
        /// **DesignBox** - this is a container designed for a specific carriage inside PurrDesign that stores its rule and coupling
        pub struct DesignBox<T: PurrStep, U: PurrRule> {
            /// rule is a specific execution rule for a carriage within the line structure of PurrTrain
            /// Pub for convenient work with the raw rule and writing custom logic for DesignBox
            pub rule: U,
            /// coupling is an optional nesting and chain vector.
            /// It is required to trigger long chains or deep nesting; see PurrRoute.construct_schedule() for more details.
            /// Direct interaction with the field removes verification for overwrites and invalid coupling states - Some(Vec::new())
            /// Pub for convenient work with the raw coupling and writing custom logic for DesignBox
            #[None]
            pub coupling: Option<Vec<T>>,
        }
        #[automatically_derived]
        impl<
            T: ::core::fmt::Debug + PurrStep,
            U: ::core::fmt::Debug + PurrRule,
        > ::core::fmt::Debug for DesignBox<T, U> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "DesignBox",
                    "rule",
                    &self.rule,
                    "coupling",
                    &&self.coupling,
                )
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::clone::Clone + PurrStep,
            U: ::core::clone::Clone + PurrRule,
        > ::core::clone::Clone for DesignBox<T, U> {
            #[inline]
            fn clone(&self) -> DesignBox<T, U> {
                DesignBox {
                    rule: ::core::clone::Clone::clone(&self.rule),
                    coupling: ::core::clone::Clone::clone(&self.coupling),
                }
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::cmp::PartialEq + PurrStep,
            U: ::core::cmp::PartialEq + PurrRule,
        > ::core::marker::StructuralPartialEq for DesignBox<T, U> {}
        #[automatically_derived]
        impl<
            T: ::core::cmp::PartialEq + PurrStep,
            U: ::core::cmp::PartialEq + PurrRule,
        > ::core::cmp::PartialEq for DesignBox<T, U> {
            #[inline]
            fn eq(&self, other: &DesignBox<T, U>) -> bool {
                self.rule == other.rule && self.coupling == other.coupling
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::cmp::Eq + PurrStep,
            U: ::core::cmp::Eq + PurrRule,
        > ::core::cmp::Eq for DesignBox<T, U> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<U>;
                let _: ::core::cmp::AssertParamIsEq<Option<Vec<T>>>;
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::hash::Hash + PurrStep,
            U: ::core::hash::Hash + PurrRule,
        > ::core::hash::Hash for DesignBox<T, U> {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.rule, state);
                ::core::hash::Hash::hash(&self.coupling, state)
            }
        }
        impl<T: PurrStep, U: PurrRule> DesignBox<T, U> {
            ///The New function with arguments not baked as None (empty) or Some(expr) (default)
            pub fn new(rule: U) -> Self {
                Self { rule, coupling: None }
            }
        }
        impl<T, U> DesignBox<T, U>
        where
            T: PurrStep,
            U: PurrRule,
        {
            /// Raw method for setting coupling
            ///
            /// # Warning
            /// If coupling is already set, the method will overwrite it
            /// If you pass an empty coupling, the method will write it as Some(Vec::new())
            pub fn set_coupling(&mut self, coupling: Vec<T>) {
                self.coupling = Some(coupling);
            }
            /// Unlike set_coupling, it validates and returns TrainResult<(), T>
            ///
            /// # Errors
            /// * `TrainError::CouplingAlreadyDefined` - if coupling is already defined
            /// * `TrainError::EmptyCoupling` - if the passed coupling contains no carriage
            ///
            /// # Warning
            /// In case the coupling is already defined and you pass a new empty coupling
            /// The method will return exactly TrainError::CouplingAlreadyDefined due to priority
            pub fn validated_set_coupling(
                &mut self,
                coupling: Vec<T>,
            ) -> TrainResult<(), T> {
                if self.coupling_is_some() {
                    return Err(TrainError::CouplingAlreadyDefined);
                }
                if coupling.is_empty() {
                    return Err(TrainError::EmptyCoupling);
                }
                self.set_coupling(coupling);
                Ok(())
            }
            /// Checks for the existence of the coupling
            pub fn coupling_is_some(&self) -> bool {
                self.coupling.is_some()
            }
            /// Checks for the absence of the coupling
            pub fn coupling_is_none(&self) -> bool {
                self.coupling.is_none()
            }
        }
    }
    pub mod train_route {
        use std::collections::HashMap;
        use wibr::{New, Make};
        use crate::cat_depot::train_core::*;
        use crate::cat_depot::train_route::RouteScheduler;
        use crate::cat_depot::train_rule::PurrRule;
        use crate::cat_depot::train_error::*;
        use super::train_siding::PurrSiding;
        use super::train_design::PurrDesign;
        /// **PurrRoute** is an active list of routes that stores expanded chains of Vec
        ///
        /// It is intended for efficient work with routes (More details in PurrSiding methods)
        /// Each stage from PurrDesign is bound to its corresponding Vec at the end of which there is always RouteBox - of the current carriage + rule from DesignBox
        pub struct PurrRoute<T: PurrStep, U: PurrRule> {
            /// schedule is a storage for all baked routes in the form of Vec
            ///
            /// Direct interaction with the field for writing to it is not recommended as:
            /// You lose convenient recursive baking (Which automatically reads PurrDesign and bakes with checks for carriage presence and cycling) and duplicate policy
            /// Pub for raw interaction with schedule and writing custom logic for PurrRoute
            #[Some(HashMap::with_capacity(DEFAULT_CAPACITY))]
            pub schedule: HashMap<T, Vec<RouteBox<T, U>>>,
            /// visited_buffer is a buffer that is filled in the bake_carriage method via recursive traversal
            ///
            /// Needed for reuse, stores inside itself carriages that have already been in the chain
            /// Pub for raw interaction with schedule and writing custom logic for PurrRoute
            ///
            /// # Example
            /// Vec(TestCarriage::Component, TestCarriage::Component) by carriage TestCarriage::Pipeline - valid
            ///
            /// Vec(TestCarriage::Component, TestCarriage::Pipeline) by carriage TestCarriage::Pipeline - not valid Err(TrainError::RecursiveCarriage { TestCarriage::Pipeline })
            #[Some(Vec::with_capacity(DEFAULT_CAPACITY))]
            pub visited_buffer: Vec<T>,
        }
        #[automatically_derived]
        impl<
            T: ::core::fmt::Debug + PurrStep,
            U: ::core::fmt::Debug + PurrRule,
        > ::core::fmt::Debug for PurrRoute<T, U> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "PurrRoute",
                    "schedule",
                    &self.schedule,
                    "visited_buffer",
                    &&self.visited_buffer,
                )
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::clone::Clone + PurrStep,
            U: ::core::clone::Clone + PurrRule,
        > ::core::clone::Clone for PurrRoute<T, U> {
            #[inline]
            fn clone(&self) -> PurrRoute<T, U> {
                PurrRoute {
                    schedule: ::core::clone::Clone::clone(&self.schedule),
                    visited_buffer: ::core::clone::Clone::clone(&self.visited_buffer),
                }
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::cmp::PartialEq + PurrStep,
            U: ::core::cmp::PartialEq + PurrRule,
        > ::core::marker::StructuralPartialEq for PurrRoute<T, U> {}
        #[automatically_derived]
        impl<
            T: ::core::cmp::PartialEq + PurrStep,
            U: ::core::cmp::PartialEq + PurrRule,
        > ::core::cmp::PartialEq for PurrRoute<T, U> {
            #[inline]
            fn eq(&self, other: &PurrRoute<T, U>) -> bool {
                self.schedule == other.schedule
                    && self.visited_buffer == other.visited_buffer
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::cmp::Eq + PurrStep,
            U: ::core::cmp::Eq + PurrRule,
        > ::core::cmp::Eq for PurrRoute<T, U> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<HashMap<T, Vec<RouteBox<T, U>>>>;
                let _: ::core::cmp::AssertParamIsEq<Vec<T>>;
            }
        }
        impl<T: PurrStep, U: PurrRule> PurrRoute<T, U> {
            ///The New function with arguments not baked as None (empty) or Some(expr) (default)
            pub fn new() -> Self {
                Self {
                    schedule: HashMap::with_capacity(DEFAULT_CAPACITY),
                    visited_buffer: Vec::with_capacity(DEFAULT_CAPACITY),
                }
            }
        }
        impl<T, U> PurrRoute<T, U>
        where
            T: PurrStep,
            U: PurrRule,
        {
            /// construct_schedule is a method for baking a template from PurrDesign
            /// Inside it performs a recursive traversal via bake_carriage, unfolding even the most unusual chains
            ///
            /// # Parameters
            /// * `SaveMode` - whether to keep already baked routes or completely clear schedule
            /// * `DuplicatePolicy` - how to handle duplicates:
            ///   * `Overwrite` - overwrite existing entries
            ///   * `Ignore` - skip duplicate carriages
            ///   * `RaiseError` - throw an error on duplicate detection
            ///
            /// # Errors
            /// * `TrainError::DuplicateCarriage` - if DuplicatePolicy::RaiseError is chosen and a carriage value already exists in schedule
            /// * `TrainError::RecursiveCarriage` - if during chain unfolding, a carriage appears that causes infinite recursion
            /// * `TrainError::UnscheduledParam` - if a carriage from the chain was not found inside PurrDesign
            pub fn construct_schedule(
                &mut self,
                purr_design: &PurrDesign<T, U>,
                save_mode: SaveMode,
                duplicate_policy: DuplicatePolicy,
            ) -> TrainResult<(), T> {
                if #[allow(non_exhaustive_omitted_patterns)]
                match save_mode {
                    SaveMode::Clear => true,
                    _ => false,
                } {
                    self.schedule.clear();
                }
                for &carriage in purr_design.blueprints.keys() {
                    if self.contains_carriage(carriage) {
                        duplicate_policy.should_overwrite(carriage)?;
                    }
                }
                for &carriage in purr_design.blueprints.keys() {
                    if self.contains_carriage(carriage)
                        && !duplicate_policy.should_overwrite(carriage)?
                    {
                        continue;
                    }
                    let mut bake_buffer = Vec::with_capacity(DEFAULT_CAPACITY);
                    self.visited_buffer.clear();
                    self.bake_carriage(carriage, purr_design, &mut bake_buffer)?;
                    self.schedule.insert(carriage, bake_buffer);
                }
                Ok(())
            }
            /// bake_carriage is a method used by construct_schedule
            /// It is public due to the public API policy, not intended for regular use
            ///
            /// # Errors
            /// * `TrainError::RecursiveCarriage` - if during chain unfolding, a carriage appears that causes infinite recursion
            /// * `TrainError::UnscheduledParam` - if a carriage from either the argument or within the chain could not be found
            pub fn bake_carriage(
                &mut self,
                carriage: T,
                purr_design: &PurrDesign<T, U>,
                bake_buffer: &mut Vec<RouteBox<T, U>>,
            ) -> TrainResult<(), T> {
                if self.visited_buffer.contains(&carriage) {
                    return Err(TrainError::RecursiveCarriage {
                        carriage,
                    });
                }
                let design_box = purr_design
                    .blueprints
                    .get(&carriage)
                    .ok_or(TrainError::UnscheduledParam {
                        carriage,
                    })?;
                self.visited_buffer.push(carriage);
                if let Some(route_carriage_vec) = &design_box.coupling {
                    for &route_carriage in route_carriage_vec {
                        self.bake_carriage(route_carriage, purr_design, bake_buffer)?;
                    }
                }
                self.visited_buffer.pop();
                let route_box = RouteBox::new(carriage, design_box.rule);
                bake_buffer.push(route_box);
                Ok(())
            }
            /// Checks for the presence of the carriage in schedule
            pub fn contains_carriage(&self, carriage: T) -> bool {
                self.schedule.contains_key(&carriage)
            }
        }
        impl<T, U> RouteScheduler for PurrRoute<T, U>
        where
            T: PurrStep,
            U: PurrRule,
        {
            type Step = T;
            type Rule = U;
            fn load(
                &self,
                purr_siding: &mut PurrSiding<Self::Step, Self::Rule>,
                carriage: Self::Step,
            ) -> TrainResult<(), Self::Step> {
                if let Some(branch) = self.schedule.get(&carriage) {
                    purr_siding.line.extend_from_slice(branch);
                    return Ok(());
                }
                Err(TrainError::UnscheduledParam {
                    carriage,
                })
            }
        }
        /// **RouteBox** is a convenient container for a carriage and its rule
        ///
        /// This structure is used throughout the entire Route - Siding - Train cycle
        /// It runs inside Train and all interaction happens with it during and after execution
        pub struct RouteBox<T: PurrStep, U: PurrRule> {
            /// carriage is a specific stage connected to a concrete rule via the rule field
            /// Pub field for direct interaction with the carriage and writing custom logic for RouteBox
            pub carriage: T,
            /// rule is a specific execution rule for a carriage within the line structure of PurrTrain
            /// Pub field for direct interaction with the rule and writing custom logic for RouteBox
            pub rule: U,
        }
        #[automatically_derived]
        impl<
            T: ::core::fmt::Debug + PurrStep,
            U: ::core::fmt::Debug + PurrRule,
        > ::core::fmt::Debug for RouteBox<T, U> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "RouteBox",
                    "carriage",
                    &self.carriage,
                    "rule",
                    &&self.rule,
                )
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::clone::Clone + PurrStep,
            U: ::core::clone::Clone + PurrRule,
        > ::core::clone::Clone for RouteBox<T, U> {
            #[inline]
            fn clone(&self) -> RouteBox<T, U> {
                RouteBox {
                    carriage: ::core::clone::Clone::clone(&self.carriage),
                    rule: ::core::clone::Clone::clone(&self.rule),
                }
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::marker::Copy + PurrStep,
            U: ::core::marker::Copy + PurrRule,
        > ::core::marker::Copy for RouteBox<T, U> {}
        #[automatically_derived]
        impl<
            T: ::core::cmp::PartialEq + PurrStep,
            U: ::core::cmp::PartialEq + PurrRule,
        > ::core::marker::StructuralPartialEq for RouteBox<T, U> {}
        #[automatically_derived]
        impl<
            T: ::core::cmp::PartialEq + PurrStep,
            U: ::core::cmp::PartialEq + PurrRule,
        > ::core::cmp::PartialEq for RouteBox<T, U> {
            #[inline]
            fn eq(&self, other: &RouteBox<T, U>) -> bool {
                self.carriage == other.carriage && self.rule == other.rule
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::cmp::Eq + PurrStep,
            U: ::core::cmp::Eq + PurrRule,
        > ::core::cmp::Eq for RouteBox<T, U> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<T>;
                let _: ::core::cmp::AssertParamIsEq<U>;
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::hash::Hash + PurrStep,
            U: ::core::hash::Hash + PurrRule,
        > ::core::hash::Hash for RouteBox<T, U> {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.carriage, state);
                ::core::hash::Hash::hash(&self.rule, state)
            }
        }
        impl<T: PurrStep, U: PurrRule> RouteBox<T, U> {
            ///The New function with all structure arguments
            pub fn new(carriage: T, rule: U) -> Self {
                Self { carriage, rule }
            }
        }
    }
    pub mod train_siding {
        use wibr::Make;
        use crate::cat_depot::train_core::*;
        use crate::cat_depot::train_route::RouteScheduler;
        use crate::cat_depot::train_rule::PurrRule;
        use crate::cat_depot::train_error::*;
        use super::train_route::RouteBox;
        pub struct PurrSiding<T: PurrStep, U: PurrRule> {
            #[Some(Vec::with_capacity(DEFAULT_CAPACITY))]
            pub line: Vec<RouteBox<T, U>>,
        }
        #[automatically_derived]
        impl<
            T: ::core::fmt::Debug + PurrStep,
            U: ::core::fmt::Debug + PurrRule,
        > ::core::fmt::Debug for PurrSiding<T, U> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "PurrSiding",
                    "line",
                    &&self.line,
                )
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::clone::Clone + PurrStep,
            U: ::core::clone::Clone + PurrRule,
        > ::core::clone::Clone for PurrSiding<T, U> {
            #[inline]
            fn clone(&self) -> PurrSiding<T, U> {
                PurrSiding {
                    line: ::core::clone::Clone::clone(&self.line),
                }
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::cmp::PartialEq + PurrStep,
            U: ::core::cmp::PartialEq + PurrRule,
        > ::core::marker::StructuralPartialEq for PurrSiding<T, U> {}
        #[automatically_derived]
        impl<
            T: ::core::cmp::PartialEq + PurrStep,
            U: ::core::cmp::PartialEq + PurrRule,
        > ::core::cmp::PartialEq for PurrSiding<T, U> {
            #[inline]
            fn eq(&self, other: &PurrSiding<T, U>) -> bool {
                self.line == other.line
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::cmp::Eq + PurrStep,
            U: ::core::cmp::Eq + PurrRule,
        > ::core::cmp::Eq for PurrSiding<T, U> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_fields_are_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<Vec<RouteBox<T, U>>>;
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::hash::Hash + PurrStep,
            U: ::core::hash::Hash + PurrRule,
        > ::core::hash::Hash for PurrSiding<T, U> {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.line, state)
            }
        }
        impl<T: PurrStep, U: PurrRule> PurrSiding<T, U> {
            ///The New function with arguments not baked as None (empty) or Some(expr) (default)
            pub fn new() -> Self {
                Self {
                    line: Vec::with_capacity(DEFAULT_CAPACITY),
                }
            }
        }
        impl<T, U> PurrSiding<T, U>
        where
            T: PurrStep,
            U: PurrRule,
        {
            pub fn launch(
                &mut self,
                carriage: T,
                save_mode: SaveMode,
                route_scheduler: &impl RouteScheduler<Step = T, Rule = U>,
            ) -> TrainResult<(), T> {
                if #[allow(non_exhaustive_omitted_patterns)]
                match save_mode {
                    SaveMode::Clear => true,
                    _ => false,
                } {
                    self.clear_line();
                }
                route_scheduler.load(self, carriage)
            }
            pub fn change_rule(
                &mut self,
                carriage: T,
                rule: U,
                mut change_rule_count: ChangeRuleCount,
            ) {
                for route_box in self.line.iter_mut() {
                    if #[allow(non_exhaustive_omitted_patterns)]
                    match change_rule_count {
                        ChangeRuleCount::Count(0) => true,
                        _ => false,
                    } {
                        return;
                    }
                    if carriage == route_box.carriage {
                        route_box.rule = rule;
                        if change_rule_count.is_final_rule_count() {
                            return;
                        }
                    }
                }
            }
            pub fn change_rule_few(
                &mut self,
                route_boxes: &[RouteBox<T, U>],
                change_rule_count: ChangeRuleCount,
            ) {
                for &route_box in route_boxes.iter() {
                    let mut route_rule_count = change_rule_count;
                    for main_route_box in self.line.iter_mut() {
                        if #[allow(non_exhaustive_omitted_patterns)]
                        match route_rule_count {
                            ChangeRuleCount::Count(0) => true,
                            _ => false,
                        } {
                            break;
                        }
                        if main_route_box.carriage == route_box.carriage {
                            main_route_box.rule = route_box.rule;
                            if route_rule_count.is_final_rule_count() {
                                break;
                            }
                        }
                    }
                }
            }
            pub fn line_is_empty(&self) -> bool {
                self.line.is_empty()
            }
            pub fn line_len(&self) -> usize {
                self.line.len()
            }
            pub fn clear_line(&mut self) {
                self.line.clear();
            }
        }
    }
}

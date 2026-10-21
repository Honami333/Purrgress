use std::ops::{Bound, RangeBounds};

use wibr::MakeFull;

use crate::cat_malloc::train_route::RouteBox;

use super::train_error::TrainResult;
use super::train_track::PurrTrack;
use super::train_core::PurrStep;
use super::train_rule::PurrRule;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, MakeFull)]
#[Fn(with_container)]
pub struct PurrCursor<V> {
    pub vector: Vec<V>,
    #[Some(0)] pub cursor: usize
}

impl<V> PurrCursor<V> {
    pub fn new() -> Self { Self::with_container(Vec::new()) }
    pub fn with_capacity(capacity: usize) -> Self { Self::with_container(Vec::with_capacity(capacity)) }
    pub fn clamp_cursor_to_len(&mut self) { self.cursor = self.cursor.min(self.vector.len()); }
    pub fn clamp_cursor_to_capacity(&mut self) { self.cursor = self.cursor.min(self.vector.capacity()); }
}

impl<T, U> PurrTrack for PurrCursor<RouteBox<T, U>>
where
    T: PurrStep,
    U: PurrRule
{
    type Step = T;
    type Rule = U;

    fn tr_new() -> Self { Self::new() }
    fn tr_with_capacity(capacity: usize) -> Self { Self::with_capacity(capacity) }
    fn tr_get_cursor(&self) -> usize { self.cursor }

    fn tr_set_cursor(&mut self, index: usize) -> TrainResult<(), Self::Step> {
        self.cursor = index;
        Ok(())
    }

    fn tr_step_cursor(&mut self) -> TrainResult<(), Self::Step> {
        self.tr_set_cursor(self.cursor.saturating_add(1))?;
        Ok(())
    }

    fn tr_get(&self, index: usize) -> Option<RouteBox<Self::Step, Self::Rule>> {  self.vector.get(index).copied() }
    fn tr_get_ref(&self, index: usize) -> Option<&RouteBox<Self::Step, Self::Rule>> { self.vector.get(index) }
    fn tr_get_mut(&mut self, index: usize) -> Option<&mut RouteBox<Self::Step, Self::Rule>> { self.vector.get_mut(index) }

    fn tr_get_current(&self) -> Option<RouteBox<Self::Step, Self::Rule>> { self.vector.get(self.cursor).copied() }
    fn tr_get_current_ref(&self) -> Option<&RouteBox<Self::Step, Self::Rule>> { self.vector.get(self.cursor) }
    fn tr_get_current_mut(&mut self) -> Option<&mut RouteBox<Self::Step, Self::Rule>> { self.vector.get_mut(self.cursor) }

    fn tr_as_slice(&self) -> &[RouteBox<Self::Step, Self::Rule>] { self.vector.as_slice() }
    fn tr_as_mut_slice(&mut self) -> &mut [RouteBox<Self::Step, Self::Rule>] { self.vector.as_mut_slice() }

    fn tr_extend<I: IntoIterator<Item = RouteBox<Self::Step, Self::Rule>>>(&mut self, iter: I) -> TrainResult<(), Self::Step> {
        self.vector.extend(iter);
        Ok(())
    }

    fn tr_splice<'a, I: IntoIterator<Item = RouteBox<Self::Step, Self::Rule>> + 'a>(
        &'a mut self, range: impl std::ops::RangeBounds<usize>, replace_with: I
    ) -> TrainResult<impl Iterator<Item = RouteBox<Self::Step, Self::Rule>>, Self::Step> {
        Ok(self.vector.splice(range, replace_with))
    }

    fn tr_drain(&mut self, range: impl std::ops::RangeBounds<usize>) -> TrainResult<impl Iterator<Item = RouteBox<Self::Step, Self::Rule>>, Self::Step> {
        let start = start_bound(&range);
        let end = end_bound(&range, self.tr_len());
        let k = end.saturating_sub(start);
        if self.cursor >= end {
            self.cursor -= k;
        } else if self.cursor >= start && self.cursor < end {
            self.cursor = start;
        };
        Ok(self.vector.drain(range))
    }
    
    fn tr_is_last(&self) -> bool { self.cursor == self.tr_len().saturating_sub(1) }
    fn tr_len(&self) -> usize { self.vector.len() }
    fn tr_is_empty(&self) -> bool { self.vector.is_empty() }

    fn tr_clear(&mut self) {
        self.vector.clear();
        self.cursor = Default::default();
    }
}

pub fn start_bound(range: &impl RangeBounds<usize>) -> usize {
    match range.start_bound() {
        Bound::Included(&c) => c,
        Bound::Excluded(&c) => c + 1,
        Bound::Unbounded => 0
    }
}

pub fn end_bound(range: &impl RangeBounds<usize>, max_range: usize) -> usize {
    match range.end_bound() {
        Bound::Included(&e) => e + 1,
        Bound::Excluded(&e) => e,
        Bound::Unbounded => max_range
    }
}

#[cfg(test)]
mod purr_cursor_test {
    use super::*;

    #[test]
    fn cursor_starts_at_zero() {
        assert_eq!(PurrCursor::<i32>::new().cursor, 0);
        assert_eq!(PurrCursor::<i32>::with_capacity(10).cursor, 0);
        assert_eq!(PurrCursor::with_container(vec![1, 2, 3]).cursor, 0);
    }

    #[test]
    fn clamp_cursor_to_len_cases() {
        let mut purr_cursor = PurrCursor::with_container(vec![1, 2, 3]);

        purr_cursor.cursor = 10;
        purr_cursor.clamp_cursor_to_len();
        assert_eq!(purr_cursor.cursor, 3);

        purr_cursor.cursor = 3;
        purr_cursor.clamp_cursor_to_len();
        assert_eq!(purr_cursor.cursor, 3);

        purr_cursor.cursor = 2;
        purr_cursor.clamp_cursor_to_len();
        assert_eq!(purr_cursor.cursor, 2);

        purr_cursor.cursor = 0;
        purr_cursor.clamp_cursor_to_len();
        assert_eq!(purr_cursor.cursor, 0);

        let mut empty_purr_cursor = PurrCursor::<i32>::new();
        empty_purr_cursor.cursor = 5;
        empty_purr_cursor.clamp_cursor_to_len();
        assert_eq!(empty_purr_cursor.cursor, 0);
    }

    #[test]
    fn clamp_cursor_to_capacity_cases() {
        let mut purr_cursor = PurrCursor::<i32>::with_capacity(10);
        let vector_capacity = purr_cursor.vector.capacity();

        purr_cursor.cursor = vector_capacity + 5;
        purr_cursor.clamp_cursor_to_capacity();
        assert_eq!(purr_cursor.cursor, vector_capacity);

        purr_cursor.cursor = vector_capacity;
        purr_cursor.clamp_cursor_to_capacity();
        assert_eq!(purr_cursor.cursor, vector_capacity);

        purr_cursor.cursor = 3;
        purr_cursor.clamp_cursor_to_capacity();
        assert_eq!(purr_cursor.cursor, 3);
    }

    #[test]
    fn start_bound_cases() {
        assert_eq!(start_bound(&(5..)), 5);
        assert_eq!(start_bound(&(5..10)), 5);
        assert_eq!(start_bound(&(0..)), 0);
        assert_eq!(start_bound(&(..)), 0);
        assert_eq!(start_bound(&(..5)), 0);
        assert_eq!(start_bound(&(Bound::Excluded(5), Bound::Unbounded)), 6);
        assert_eq!(start_bound(&(Bound::Excluded(0), Bound::Unbounded)), 1);
    }

    #[test]
    fn end_bound_cases() {
        assert_eq!(end_bound(&(..=5), 10), 6);
        assert_eq!(end_bound(&(..5), 10), 5);
        assert_eq!(end_bound(&(5..10), 20), 10);
        assert_eq!(end_bound(&(5..), 10), 10);
        assert_eq!(end_bound(&(..), 10), 10);
        assert_eq!(end_bound(&(0..=0), 10), 1);
        assert_eq!(end_bound(&(..0), 10), 0);

        assert_eq!(end_bound(&(..=15), 10), 16);
        assert_eq!(end_bound(&(..15), 10), 15);
        assert_eq!(end_bound(&(..=10), 10), 11);
        assert_eq!(end_bound(&(..10), 10), 10);
    }
}
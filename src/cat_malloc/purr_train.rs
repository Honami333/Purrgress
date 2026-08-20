use std::marker::PhantomData;

use wibr::Make;

use crate::types::PurrStep;
use crate::types::InsertPosition;

use super::train_route::*;
use super::train_siding::*;
use super::train_types::*;





#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[derive(Debug, Make)]
pub struct PurrTrain<T: PurrStep, U: PurrRule, S: PurrTrack<RouteBox<T, U>> = cursorvec::CursorVec<RouteBox<T, U>>> {
    #[Some(S::tr_new())] pub line: S,
    #[Some(Default::default())] _mark_t: PhantomData<T>,
    #[Some(Default::default())]_mark_u: PhantomData<U>
}

impl<T, U, S> Default for PurrTrain<T, U, S> 
where 
    T: PurrStep,
    U: PurrRule,
    S: PurrTrack<RouteBox<T, U>>
{
    fn default() -> Self { Self::new() }
}

impl<T, U, S> PurrTrain<T, U, S> 
where 
    T: PurrStep,
    U: PurrRule,
    S: PurrTrack<RouteBox<T, U>>
{
    pub fn attach(&mut self, purr_siding: &mut PurrSiding<T, U>) {
        self.line.tr_extend(purr_siding.main_train.drain(..));
    }

    pub fn replace(&mut self, purr_siding: &mut PurrSiding<T, U>) {
        self.line.tr_clear();
        self.attach(purr_siding);
    }

    pub fn reroute_at(&mut self, purr_siding: &mut PurrSiding<T, U>, insert_position: InsertPosition) {
        let cursor_pos = self.line.tr_get_cursor();
        let mut index = match insert_position {
            InsertPosition::Forward => cursor_pos,
            InsertPosition::Index(i) => i + cursor_pos
        };
        index = index.min(self.line.tr_len());

        self.line.tr_splice(index..index, purr_siding.main_train.drain(..));
    }

    pub fn shrink_line(&mut self, line_length: usize) {
        let cursor_idx = self.line.tr_get_cursor();
        if cursor_idx > line_length && cursor_idx != 0 {
            let _ = self.line.tr_drain(0..cursor_idx); 
            self.line.tr_set_cursor(0);
        };
    }

    pub fn get_current(&self) -> Option<RouteBox<T, U>> {
        self.line.tr_get_current()
    }

    pub fn get_current_ref(&self) -> Option<&RouteBox<T, U>> {
        self.line.tr_get_current_ref()
    }

    pub fn get_current_mut(&mut self) -> Option<&mut RouteBox<T, U>> {
        let index = self.get_cursor();
        self.line.tr_get_mut(index)
    }

    pub fn get_cursor(&self) -> usize {
        self.line.tr_get_cursor()
    }

    pub fn get_line(&self) -> &[RouteBox<T, U>] {
        self.line.tr_as_slice()
    }

    pub fn get_mut_line(&mut self) -> &mut [RouteBox<T, U>] {
        self.line.tr_as_mut_slice()
    }

    pub fn advance_train(&mut self) -> PurrTrainEvent<T, U> {
        if let Some(route_box) = self.get_current() {
            match route_box.rule.is_finished() {
                true => {
                    if self.line.tr_get_cursor() >= self.line.tr_len() - 1 {
                        self.line.tr_clear();
                        return PurrTrainEvent::Transition { from: route_box, to: None };
                    };
                    self.line.tr_step_cursor();
                    let cursor_state = self.get_current();
                    return PurrTrainEvent::Transition { from: route_box, to: cursor_state };
                },
                false => return PurrTrainEvent::Running(route_box)
            };
        };
        PurrTrainEvent::Idle
    }
}
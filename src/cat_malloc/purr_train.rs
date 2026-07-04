use crate::types::{PurrStep, PurrEvent};
use crate::types::InsertPosition;

use cursorvec::CursorVec;

use super::train_route::*;
use super::train_siding::*;
use super::train_types::*;


#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug)]
pub struct PurrTrain<T: PurrStep, U: PurrRule> {
    line: CursorVec<RouteBox<T, U>>
}

impl<T, U> Default for PurrTrain<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, U> PurrTrain<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    pub fn new() -> Self {
        Self {
            line: CursorVec::new()
        }
    }

    pub fn attach(&mut self, purr_siding: &mut PurrSiding<T, U>) {
        self.line.extend(purr_siding.main_train.drain(..));
    }

    pub fn replace(&mut self, purr_siding: &mut PurrSiding<T, U>) {
        self.line.clear();

        self.line.extend(purr_siding.main_train.drain(..));
    }

    pub fn reroute_at(&mut self, purr_siding: &mut PurrSiding<T, U>, insert_position: InsertPosition) {
        let index = match insert_position {
            InsertPosition::Forward => 0_usize,
            InsertPosition::Index(i) => i
        };

        for (i, route_box) in (index..).zip(purr_siding.main_train.drain(..)) {
            self.line.insert(i, route_box);
        };
    }

    pub fn shrink_line(&mut self, line_length: usize) {
        self.line.clear();

        if self.line.capacity() > line_length {
            self.line.shrink_to_fit(); 
        };
    }

    pub fn get_current(&self) -> Option<RouteBox<T, U>> {
        self.line.get_current().value().cloned()
    }

    pub fn get_current_mut(&mut self) -> Option<&mut RouteBox<T, U>> {
        let index = self.get_cursor()?;

        self.line.get_mut(index)
    }

    pub fn get_cursor(&self) -> Option<usize> {
        self.line.get_cursor()
    }

    pub fn get_line(&self) -> &[RouteBox<T, U>] {
        self.line.as_slice()
    }

    pub fn get_mut_line(&mut self) -> &mut [RouteBox<T, U>] {
        self.line.as_mut_slice()
    }

    pub fn advance_train(&mut self) -> PurrEvent<T> {
        let current_idx = self.line.get_cursor();
        let mut is_finished = false;
        let mut carriage = None;

        if let Some(i) = current_idx && let Some(route_box) = self.line.get_mut(i) {
            if route_box.rule.is_finished() {
                carriage = Some(route_box.carriage);
                is_finished = true;
            };

            if !is_finished {
                return PurrEvent::Running(route_box.carriage);
            };
        };

        if let Some(i) = current_idx && let Some(carr) = carriage && is_finished {
            if let Some(i) = current_idx && i == self.line.len() - 1 { 
                self.shrink_line(1000);
                
                return PurrEvent::Transition { from: carr, to: None };
            };

            self.line.set_cursor(i + 1);
            self.line.update_cursor();

            let cursor_state = self.line.get_current();

            let next_route_box = cursor_state.value();

            let next_carriage = next_route_box.map(|route_box| route_box.carriage);

            return PurrEvent::Transition { from: carr, to: next_carriage };
        };
        
    

        PurrEvent::Idle
    }
}


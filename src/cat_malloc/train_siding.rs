use crate::types::PurrStep;

use wibr::MakeFull;

use super::train_route::*;
use super::train_types::*;
use super::train_error::*;


#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, MakeFull)]
#[Extern(size: usize)]
pub struct PurrSiding<T: PurrStep, U: PurrRule> {
    #[Functional({ Vec::with_capacity(size) })] pub main_line: Vec<RouteBox<T, U>>,
}


impl<T, U> Default for PurrSiding<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    fn default() -> Self {
        Self::new(VECTOR_SIZE)
    }
}

impl<T, U> PurrSiding<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    pub fn launch(&mut self, carriage: T, buffer_mode: BufferMode, purr_route: &PurrRoute<T, U>) -> TrainResult<(), T> {
        if let Some(branch ) = purr_route.schedule.get(&carriage) {
            if matches!(buffer_mode, BufferMode::Clear) { self.clear_main_train(); };
            self.main_line.extend_from_slice(branch);
            return Ok(());
        };
        Err( TrainError::UnscheduledParam { carriage } )
    }

    pub fn change_rule(&mut self, carriage: T, rule: U) {
        for route_box in self.main_line.iter_mut() {
            if carriage == route_box.carriage { route_box.rule = rule; };
        };
    }

    pub fn change_rule_few(&mut self, route_boxs: &[RouteBox<T, U>]) {
        for main_route_box in self.main_line.iter_mut() {
            for &route_box in route_boxs.iter() {
                if main_route_box.carriage == route_box.carriage { main_route_box.rule = route_box.rule; };
            };
        };
    }

    pub fn clear_main_train(&mut self) {
        self.main_line.clear();
    }

    pub fn train_len(&self) -> usize {
        self.main_line.len()
    }
}

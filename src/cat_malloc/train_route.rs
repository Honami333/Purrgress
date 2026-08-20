use crate::types::PurrStep;

use std::collections::HashMap;

use anyhow::{anyhow, Result};

use wibr::New;

use super::train_design::*;
use super::train_types::*;



#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurrRoute<T: PurrStep, U: PurrRule> {
    pub schedule: HashMap<T, Vec<RouteBox<T, U>>>,
    pub bake_buffer: Vec<RouteBox<T, U>>,
    pub visited_buffer: Vec<T>,
}

impl<T, U> Default for PurrRoute<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    fn default() -> Self {
        Self::new(VECTOR_SIZE)
    }
}

impl<T, U> PurrRoute<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    pub fn new(size: usize) -> Self {
        Self {
            schedule: HashMap::new(),
            bake_buffer: Vec::with_capacity(size),
            visited_buffer: Vec::with_capacity(size)
        }
    }

    pub fn construct_schedule(&mut self, purr_design: &PurrDesign<T, U>, buffer_mode: BufferMode) -> Result<()> {
        if matches!(buffer_mode, BufferMode::Clear) { self.schedule.clear(); };

        for parametr in purr_design.blueprints.keys() {
            self.bake_buffer.clear();
            self.visited_buffer.clear();

            Self::bake_stage(parametr, purr_design, &mut self.bake_buffer, &mut self.visited_buffer)?;

            self.schedule.insert(*parametr, self.bake_buffer.clone());
        };

        Ok(())
    }

    pub fn bake_stage(
        parametr: &T,
        purr_design: &PurrDesign<T, U>,
        bake_buffer: &mut Vec<RouteBox<T, U>>,
        visited_buffer: &mut Vec<T>
    ) -> Result<()> {
        if visited_buffer.contains(parametr) {
            return Err( anyhow!("Recursive stage error carriage: {:?}", *parametr) );
        };

        if let Some(design_box) = purr_design.blueprints.get(parametr) {
            visited_buffer.push(*parametr);

            if let Some(route_parametr_vec) = &design_box.coupling {
                for route_parametr in route_parametr_vec {
                    Self::bake_stage(route_parametr, purr_design, bake_buffer, visited_buffer)?;
                };
            };

            visited_buffer.pop();

            let route_box = RouteBox::new(design_box.rule, *parametr);
            bake_buffer.push(route_box);
        };

        Ok(())
    }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, New)]
pub struct RouteBox<T: PurrStep, U: PurrRule> {
    pub rule: U,
    pub carriage: T
}
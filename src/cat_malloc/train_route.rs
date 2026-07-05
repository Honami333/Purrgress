use crate::types::PurrStep;

use std::collections::HashMap;

use anyhow::{anyhow, Result};

use super::train_design::*;
use super::train_types::*;


#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, Clone)]
pub struct PurrRoute<T: PurrStep, U: PurrRule> {
    pub schedule: HashMap<T, Vec<RouteBox<T, U>>>
}

impl<T, U> Default for PurrRoute<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, U> PurrRoute<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    pub fn new() -> Self {
        Self {
            schedule: HashMap::new()
        }
    }

    pub fn construct_schedule(&mut self, purr_design: &PurrDesign<T, U>) -> Result<()> {
        self.schedule.clear();

        for parametr in purr_design.blueprints.keys() {
            let mut route_vec = Vec::new();
            let mut visited = Vec::new();

            Self::bake_stage(parametr, purr_design, &mut route_vec, &mut visited)?;

            self.schedule.insert(*parametr, route_vec);
        };

        Ok(())
    }

    pub fn bake_stage(
        parametr: &T,
        purr_design: &PurrDesign<T, U>,
        route_vec: &mut Vec<RouteBox<T, U>>,
        visited: &mut Vec<T>
    ) -> Result<()> {
        if visited.contains(parametr) {
            return Err( anyhow!("Recursive stage error carriage: {:?}", *parametr) );
        };

        if let Some(design_box) = purr_design.blueprints.get(parametr) {
            visited.push(*parametr);

            if let Some(route_box_vec) = &design_box.coupling {
                for  route_box in route_box_vec {
                    Self::bake_stage(
                        route_box,
                        purr_design,
                        route_vec,
                        visited
                    )?;
                };
            };

            visited.pop();

            let route_box = RouteBox::new(design_box.rule.clone(), *parametr);
            route_vec.push(route_box);
        };

        Ok(())
    }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, Clone)]
pub struct RouteBox<T: PurrStep, U: PurrRule> {
    pub rule: U,
    pub carriage: T
}

impl<T, U> RouteBox<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    pub fn new(rule: U, carriage: T) -> Self {
        Self {
            rule,
            carriage
        }
    }
}

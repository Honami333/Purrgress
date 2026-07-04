use crate::types::PurrStep;

use std::collections::HashMap;

use super::train_types::*;


#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[derive(Debug, Clone)]
pub struct PurrDesign<T: PurrStep, U: PurrRule> {
    pub(crate) blueprints: HashMap<T, DesignBox<T, U>>
}

impl<T, U> Default for PurrDesign<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, U> PurrDesign<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    pub fn new() -> Self {
        Self {
            blueprints: HashMap::new()
        }
    }

    pub fn single(&mut self, parametr: T, rule: U) {
        let design = DesignBox::new(rule, None);

        self.blueprints.insert(parametr, design);
    }

    pub fn chain(&mut self, parametr: T, design: DesignBox<T, U>) {
        self.blueprints.insert(parametr, design);
    }
}

#[derive(Debug, Clone)]
pub struct DesignBox<T: PurrStep, U: PurrRule> {
    pub rule: U,
    pub coupling: Option<Vec<T>>
}

impl<T, U> DesignBox<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    pub fn new(rule: U, coupling: Option<Vec<T>>) -> Self {
        Self {
            rule,
            coupling
        }
    }
}
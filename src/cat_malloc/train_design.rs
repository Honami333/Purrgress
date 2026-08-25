use crate::types::PurrStep;

use std::collections::HashMap;

use wibr::Make;

use super::train_types::*;


#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Make)]
pub struct PurrDesign<T: PurrStep, U: PurrRule> {
    #[Some(HashMap::new())] pub blueprints: HashMap<T, DesignBox<T, U>>
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
    pub fn single(&mut self, parametr: T, rule: U) {
        let design = DesignBox::new(rule);

        self.blueprints.insert(parametr, design);
    }

    pub fn chain(&mut self, parametr: T, rule: U, coupling: Vec<T>) {
        let mut design = DesignBox::new(rule);
        design.set_coupling(coupling);
        self.blueprints.insert(parametr, design);
    }

    pub fn design_chain(&mut self, parametr: T, design: DesignBox<T, U>) {
        self.blueprints.insert(parametr, design);
    }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Make)]
pub struct DesignBox<T: PurrStep, U: PurrRule> {
    pub rule: U,
    #[None] pub coupling: Option<Vec<T>>
}

impl<T, U> DesignBox<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    pub fn set_coupling(&mut self, coupling: Vec<T>) {
        self.coupling = Some(coupling);
    }
}
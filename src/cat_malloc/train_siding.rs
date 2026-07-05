use crate::types::PurrStep;

use std::collections::HashSet;

use anyhow::{anyhow, Result};

use super::train_route::*;
use super::train_types::*;


#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
pub struct PurrSiding<T: PurrStep, U: PurrRule> {
    pub main_train: Vec<RouteBox<T, U>>,
    pub switches: Vec<usize>
}


impl<T, U> Default for PurrSiding<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, U> PurrSiding<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    pub fn new() -> Self {
        Self {
            main_train: Vec::new(),
            switches: Vec::new()
        }
    }

    pub fn launch(&mut self, parametr: T, buffer_mode: BufferMode, purr_route: &PurrRoute<T, U>) -> Result<()> {
        let op_train = purr_route.schedule.get(&parametr);

        if let Some(train) = op_train {
            if matches!(buffer_mode, BufferMode::Clear) { self.clear_main_train(); };

            self.main_train.extend(train.clone());
            return Ok(());
        };

        Err( anyhow!("Couldn't find {:?}", parametr) )
    }

    pub fn find_index(&mut self, parametr: T, buffer_mode: BufferMode) {
        if matches!(buffer_mode, BufferMode::Clear) { self.clear_switches(); };

        for (i, route_box) in self.main_train.iter().enumerate() {
            if parametr == route_box.carriage {
                self.switches.push(i);
            };
        };
    }

    pub fn find_index_few(&mut self, parametrs: &[T], buffer_mode: BufferMode) {
        if matches!(buffer_mode, BufferMode::Clear) { self.clear_switches(); };

        for (i, route_box) in self.main_train.iter().enumerate() {
            if parametrs.contains(&route_box.carriage) {
                self.switches.push(i);
            };
        };
    }

    pub fn find_index_many(&mut self, parametrs: &[T], buffer_mode: BufferMode) {
        if matches!(buffer_mode, BufferMode::Clear) { self.clear_switches(); };

        let target_set: HashSet<&T> = parametrs.iter().collect();

        for (i, route_box) in self.main_train.iter().enumerate() {
            if target_set.contains(&route_box.carriage) {
                self.switches.push(i);
            };
        };
    }

    pub fn clear_switches(&mut self) {
        self.switches.clear();
    }

    pub fn clear_main_train(&mut self) {
        self.main_train.clear();
    }

    pub fn get_switches(&self) -> &[usize] {
        &self.switches
    }

    pub fn train_len(&self) -> usize {
        self.main_train.len()
    }

    pub fn change_rule(&mut self, index: usize, rule: U) -> Result<()> {
        let op_route_box = self.main_train.get_mut(index);

        if let Some(route_box) = op_route_box {
            route_box.rule = rule;

            return Ok(());
        };

        Err( anyhow!("Couldn't find parametr by {:?}", index) )
    }
}

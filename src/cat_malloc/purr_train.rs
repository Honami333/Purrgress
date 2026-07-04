use crate::cat_stage_manager::condition::PurrCondition;
use crate::cat_stage_manager::{condition, manager_types::PurrVec};
use crate::cat_stage_manager::manager_types::{PurrStep, PurrEvent};
use crate::cat_stage_manager::manager_types::InsertPosition;

use cursorvec::CursorVec;

use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::{anyhow, Result};


pub struct PurrSiding<T: PurrStep> {
    main_train: Vec<RouteBox<T>>,
    switches: Vec<usize>
}


impl<T> Default for PurrSiding<T> 
where 
    T: PurrStep
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PurrSiding<T> 
where 
    T: PurrStep
{
    pub fn new() -> Self {
        Self {
            main_train: Vec::new(),
            switches: Vec::new()
        }
    }

    pub fn launch(&mut self, parametr: T, purr_route: &PurrRoute<T>) -> Result<()> {
        let op_train = purr_route.schedule.get(&parametr);

        if let Some(train) = op_train {
            self.main_train.clear();

            self.main_train.extend(train.clone());
            return Ok(());
        };

        Err( anyhow!("Couldn't find {:?}", parametr) )
    }

    pub fn find_index(&mut self, parametr: T) {
        for (i, route_box) in self.main_train.iter().enumerate() {
            if parametr == route_box.carriage {
                self.switches.push(i);
            };
        };
    }

    pub fn find_index_few(&mut self, parametrs: &[T]) {
        for (i, route_box) in self.main_train.iter().enumerate() {
            if parametrs.contains(&route_box.carriage) {
                self.switches.push(i);
            };
        };
    }

    pub fn find_index_many(&mut self, parametrs: &[T]) {
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

    pub fn get_switches(&self) -> &[usize] {
        &self.switches
    }

    pub fn train_len(&self) -> usize {
        self.main_train.len()
    }

    pub fn change_rule(&mut self, index: usize, rule: StandardRules) -> Result<()> {
        let op_route_box = self.main_train.get_mut(index);

        if let Some(route_box) = op_route_box {
            route_box.rule = rule;

            return Ok(());
        };

        Err( anyhow!("Couldn't find parametr by {:?}", index) )
    }
}

#[derive(Debug)]
pub struct PurrTrain<T: PurrStep> {
    line: CursorVec<RouteBox<T>>
}

impl<T> Default for PurrTrain<T> 
where 
    T: PurrStep
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PurrTrain<T> 
where 
    T: PurrStep
{
    pub fn new() -> Self {
        Self {
            line: CursorVec::new()
        }
    }

    pub fn attach(&mut self, purr_siding: &mut PurrSiding<T>) {
        self.line.extend(purr_siding.main_train.drain(..));
    }

    pub fn replace(&mut self, purr_siding: &mut PurrSiding<T>) {
        self.line.clear();

        self.line.extend(purr_siding.main_train.drain(..));
    }

    pub fn reroute_at(&mut self, purr_siding: &mut PurrSiding<T>, insert_position: InsertPosition) {
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

    pub fn get_current(&self) -> Option<RouteBox<T>> {
        self.line.get_current().value().cloned()
    }

    pub fn get_current_mut(&mut self) -> Option<&mut RouteBox<T>> {
        let index = self.get_cursor()?;

        self.line.get_mut(index)
    }

    pub fn get_cursor(&self) -> Option<usize> {
        self.line.get_cursor()
    }

    pub fn get_line(&self) -> &[RouteBox<T>] {
        self.line.as_slice()
    }

    pub fn get_mut_line(&mut self) -> &mut [RouteBox<T>] {
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

#[derive(Debug, Clone)]
pub struct PurrDesign<T: PurrStep> {
    blueprints: HashMap<T, DesignBox<T>>
}

impl<T> Default for PurrDesign<T> 
where 
    T: PurrStep
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PurrDesign<T> 
where 
    T: PurrStep
{
    pub fn new() -> Self {
        Self {
            blueprints: HashMap::new()
        }
    }

    pub fn single(&mut self, parametr: T, rule: StandardRules) {
        let design = DesignBox::new(rule, None);

        self.blueprints.insert(parametr, design);
    }

    pub fn chain(&mut self, parametr: T, design: DesignBox<T>) {
        self.blueprints.insert(parametr, design);
    }
}

#[derive(Debug, Clone)]
pub struct PurrRoute<T: PurrStep> {
    schedule: HashMap<T, Vec<RouteBox<T>>>
}

impl<T> Default for PurrRoute<T> 
where 
    T: PurrStep
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PurrRoute<T> 
where 
    T: PurrStep
{
    pub fn new() -> Self {
        Self {
            schedule: HashMap::new()
        }
    }

    pub fn construct_schedule(&mut self, purr_design: &PurrDesign<T>) -> Result<()> {
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
        purr_design: &PurrDesign<T>,
        route_vec: &mut Vec<RouteBox<T>>,
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

#[derive(Debug, Clone)]
pub struct DesignBox<T: PurrStep> {
    pub rule: StandardRules,
    pub coupling: Option<Vec<T>>
}

impl<T> DesignBox<T> 
where 
    T: PurrStep
{
    pub fn new(rule: StandardRules, coupling: Option<Vec<T>>) -> Self {
        Self {
            rule,
            coupling
        }
    }
}

#[derive(Debug, Clone)]
pub struct RouteBox<T: PurrStep> {
    pub rule: StandardRules,
    pub carriage: T
}

impl<T> RouteBox<T> 
where 
    T: PurrStep
{
    pub fn new(rule: StandardRules, carriage: T) -> Self {
        Self {
            rule,
            carriage
        }
    }
}

#[derive(Debug, Clone)]
pub enum StandardRules {
    Instant(condition::InstantCondition),
    Flag(condition::PurrFlag),
    Timer(condition::PurrTimer),
    Proximity(condition::PurrProximity)
}

impl StandardRules {
    pub fn instant() -> Self {
        Self::Instant(condition::InstantCondition)
    }

    pub fn timer(duration: f32) -> Self {
        Self::Timer(condition::PurrTimer::new(duration))
    }

    pub fn flag() -> Self {
        Self::Flag(condition::PurrFlag::new())
    }

    pub fn proximity(pos: PurrVec, start_pos: PurrVec, target_pos: PurrVec) -> Self {
        Self::Proximity(condition::PurrProximity::new(pos, start_pos, target_pos))
    }

    pub fn is_finished(&mut self) -> bool {
        match self {
            Self::Instant(instant) => instant.is_finished(),
            Self::Timer(timer) => timer.is_finished(),
            Self::Flag(flag) => flag.is_finished(),
            Self::Proximity(proximity) => proximity.is_finished(),
        }
    }

    pub fn get_instant(&self) -> Option<&condition::InstantCondition> {
        if let Self::Instant(instant) = self {
            return Some(instant);
        };

        None
    }

    pub fn get_mut_instant(&mut self) -> Option<&mut condition::InstantCondition> {
        if let Self::Instant(instant) = self {
            return Some(instant);
        };

        None
    }

    pub fn get_timer(&self) -> Option<&condition::PurrTimer> {
        if let Self::Timer(timer) = self {
            return Some(timer);
        };

        None
    }

    pub fn get_mut_timer(&mut self) -> Option<&mut condition::PurrTimer> {
        if let Self::Timer(timer) = self {
            return Some(timer);
        };

        None
    }

    pub fn get_flag(&self) -> Option<&condition::PurrFlag> {
        if let Self::Flag(flag) = self {
            return Some(flag);
        };

        None
    }

    pub fn get_mut_flag(&mut self) -> Option<&mut condition::PurrFlag> {
        if let Self::Flag(flag) = self {
            return Some(flag);
        };

        None
    }

    pub fn get_proximity(&self) -> Option<&condition::PurrProximity> {
        if let Self::Proximity(proximity) = self {
            return Some(proximity);
        };

        None
    }

    pub fn get_mut_proximity(&self) -> Option<&condition::PurrProximity> {
        if let Self::Proximity(proximity) = self {
            return Some(proximity);
        };

        None
    }
}
use std::collections::HashMap;
use crate::cat_stage_manager::*;
use crate::cat_stage_manager::manager::StageManager;

use super::*;

// This file contains all the animator metadata, which serves to:
//  Synchronize stages
//  Stitch logic together
//  Update logic

// You can explore and dig into it for familiarization purposes!
// But in most projects, these structures will never be needed in their raw form!

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
pub struct PurrAnimator<T, U, C>
where 
    T: manager_types::PurrStep,
    U: manager_types::PurrStep
{
    animator_manager: manager::StageManager<T>,

    animated_stages: HashMap<U, (pandemonium_types::PurrFrameStage, PurrAnimateMetaData<C>)>,
}

impl<T, U, C> PurrAnimator<T, U, C> 
where 
    T: manager_types::PurrStep,
    U: manager_types::PurrStep
{
    pub fn new(
        animator_manager: manager::StageManager<T>,
        animated_stages: HashMap<U, (pandemonium_types::PurrFrameStage, PurrAnimateMetaData<C>)>,
    ) -> Self {
        PurrAnimator {
            animator_manager,
            animated_stages,
        }
    }
}

impl<T, U, C> PurrAnimator<T, U, C> 
where 
    T: manager_types::PurrStep,
    U: manager_types::PurrStep
{
    pub fn get_animator(&self) -> &manager::StageManager<T> {
        &self.animator_manager
    }

    pub fn get_animator_mut(&mut self) -> &mut manager::StageManager<T> {
        &mut self.animator_manager
    }
}

impl<T, U, C> PurrAnimator<T, U, C> 
where 
    T: manager_types::PurrStep,
    U: manager_types::PurrStep
{
    pub fn get_animated_stages(&self, stage: U) -> Option<&(pandemonium_types::PurrFrameStage, PurrAnimateMetaData<C>)>
    {
        self.animated_stages.get(&stage)
    }

    pub fn get_animated_stages_mut(&mut self, stage: U) -> Option<&mut(pandemonium_types::PurrFrameStage, PurrAnimateMetaData<C>)>
    {
        self.animated_stages.get_mut(&stage)
    }

    pub fn get_animated_stages_no_key(&mut self, stage: pandemonium_types::PurrFrameStage) -> Option<(&U, &(pandemonium_types::PurrFrameStage, PurrAnimateMetaData<C>))>
    {
        self.animated_stages
            .iter()
            .find(|(_, animated_stages)| animated_stages.0 == stage)
    }

    pub fn get_animated_stages_mut_no_key(&mut self, stage: pandemonium_types::PurrFrameStage) -> Option<(&U, &mut(pandemonium_types::PurrFrameStage, PurrAnimateMetaData<C>))>
    {
        self.animated_stages
            .iter_mut()
            .find(|(_, animated_stages)| animated_stages.0 == stage)
    }
}

#[derive(Debug, Clone)]
pub struct PurrAnimateMetaData<C> {
    flow_stages: Vec<(C, PurrFlowMetaData)>,
}

impl<C> PurrAnimateMetaData<C> {
    pub fn new(
        flow_stages: Vec<(C, PurrFlowMetaData)>
    ) -> Self {
        PurrAnimateMetaData {
            flow_stages,
        }
    }
}

impl <C> PurrAnimateMetaData<C> {
    pub fn get_flow_stages(&self) -> &[(C, PurrFlowMetaData)] {
        &self.flow_stages
    }

    pub fn get_flow_stages_mut(&mut self) -> &mut [(C, PurrFlowMetaData)] {
        &mut self.flow_stages
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PurrFlowMetaData {
    flow_stage_chain_index: pandemonium_types::PurrFrameStage,
    last_frame_index: pandemonium_types::PurrFrameStage,

    frame: f32,
    fps: f32,
    duration: f32,
}

impl PurrFlowMetaData {
    pub fn new(
        flow_stage_chain_index: pandemonium_types::PurrFrameStage,
        frame_index: usize,
        frame: f32,
        fps: f32,
        duration: f32,
    ) -> Self {
        PurrFlowMetaData {
            flow_stage_chain_index,
            last_frame_index: pandemonium_types::PurrFrameStage::Frame(frame_index),

            frame,
            fps,
            duration,
        }
    }
}

impl PurrFlowMetaData {
    pub fn get_frame(&self) -> f32 {
        self.frame
    }

    pub fn get_fps(&self) -> f32 {
        self.fps
    }

    pub fn get_duration(&self) -> f32 {
        self.duration
    }

    pub fn get_flow_stage_chain_index(&self) -> pandemonium_types::PurrFrameStage {
        self.flow_stage_chain_index
    }

    pub fn get_last_frame_index(&self) -> pandemonium_types::PurrFrameStage {
        self.last_frame_index
    }

    pub fn frame_duration(&self) -> f32 {
        1.0 / self.fps
    }

    pub fn frame_stage_count(&self) -> usize {
        (self.duration / (self.frame / self.fps)).round() as usize
    }
}

// A frame execution condition update function created in accordance with the animation manager instructions
pub fn flow_stage_chain(sub_stage_manager: &mut StageManager<pandemonium_types::PurrFrameStage>, delta: f32, last_frame: usize) {
    for i in 0..=last_frame {
        if let Some(timer) = sub_stage_manager.get_condition_mut::<condition::PurrTimer>(pandemonium_types::PurrFrameStage::Frame(i)) {
            timer.tick(delta);
        }
    }
}
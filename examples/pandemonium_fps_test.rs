use purrgress::cat_stage_manager::manager::{meowphosis, PurrStep};
use purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage;
use purrgress::types;
use std::collections::HashMap;
use std::time;

#[meowphosis]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
pub enum MyStage {
    Idle,
    Walk,
    Run,
    PurrChain(usize)
}

#[derive(Debug, Clone, Copy)]
pub enum MyFrameStage {
    Start,
    Run,
    End,
    Pause,
}

fn main() {
    let idle_ani_manager = purrgress_macros::purr_pandemonium!(
        !!MyStage::Idle : <
            MyFrameStage::Start, [3, 10] =>
            MyFrameStage::Run, [3, 12, pandemonium_types::AbyssalDuration::Seconds(1.0)] => 
            MyFrameStage::End, [3, 10]
        >
    );

    let walk_ani_manager = purrgress_macros::purr_pandemonium!(
        !!MyStage::Walk : <
            MyFrameStage::Pause, [3, 10, pandemonium_types::AbyssalDuration::Millis(100.0)] =>
            MyFrameStage::Start, [3, 10] =>
            MyFrameStage::Pause, [3, 10, pandemonium_types::AbyssalDuration::Millis(100.0)] =>
            MyFrameStage::Run, [10, 24, pandemonium_types::AbyssalDuration::Seconds(10.0)] => 
            MyFrameStage::End, [3, 10]
        >
    );

    let mut animator_meta_data = purrgress_macros::abyssal_grimoire!(
        !!MyStage : <
            idle_ani_manager,
            walk_ani_manager
        >
    );

    purrgress_macros::abyssal_march!(
        !!!animator_meta_data : <
            manager::PurrAction::Push : MyStage::Idle,
            !types::DuplicatePolicy::KeepAll;
        >
    );

    purrgress_macros::abyssal_march!(
        !!!animator_meta_data : <
            manager::PurrAction::Push : MyStage::Walk,
            !types::DuplicatePolicy::KeepAll;
        >
    );

    let mut last_time = time::Instant::now();

    loop {
        let delta = get_delta_time(&mut last_time);

        let updated_animator_meta_data = purrgress_macros::purr_rumble_brimstone!(
            !!!animator_meta_data
        );

        if let (Some(stage), Some(sub_satge), Some(index)) = updated_animator_meta_data {
            println!("ani stage: {:?}, ani sub stage: {:?}, stage index: {}", stage, sub_satge, index);
        };

        let animator = animator_meta_data.get_animator();

        if animator.query_is_empty() {
            break;
        };
    }
}

fn get_delta_time(last_time: &mut time::Instant) -> f32 {
    let current_time = time::Instant::now();
    let delta = current_time.duration_since(*last_time).as_secs_f32();
    *last_time = current_time;

    delta
}
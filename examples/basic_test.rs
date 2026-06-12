use purrgress::cat_stage_manager::*;

use purrgress_macros::{meowphosis, PurrStep};

use std::time;


#[meowphosis]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]

pub enum MyStage {
    Idle,
    Walk,
    Run,
    PurrChain(usize)
}

fn main() {
    
    let mut cat_manager = MyStage::meowphosis_manager();

    let sub_cat_manager_procces_1 = create_sub_manager_1(&mut cat_manager);
    add_sub_manager_1(&mut cat_manager, sub_cat_manager_procces_1);

    let sub_cat_manager_procces_2 = create_sub_manager_2(&mut cat_manager);
    add_sub_manager_2(&mut cat_manager, sub_cat_manager_procces_2);

    println!("{:?}", cat_manager.current_vec_query());

    let mut last_time = time::Instant::now();

    loop {
        let delta = get_delta_time(&mut last_time);

        update_sub_manager_1(&mut cat_manager, sub_cat_manager_procces_1, delta);
        
        update_sub_manager_2(&mut cat_manager, sub_cat_manager_procces_2, delta);

        match cat_manager.update() {
            manager_types::PurrEvent::Idle => {
                println!("The queue main manager is empty");
                break;
            },
            manager_types::PurrEvent::Running(_) => (),
            manager_types::PurrEvent::Transition { from, to } => {
                println!("The action {:?} is over", from);
                if let Some(to) = to {
                    println!("Go to {:?}", to);
                };
            },
        };
    }
}

fn create_sub_manager_1(cat_manager: &mut manager::StageManager<MyStage>) -> MyStage {

    let idle_condition = condition::PurrTimer::new(2.0);
    let walk_condition = condition::PurrTimer::new(2.0);
    let run_condition = condition::PurrTimer::new(4.0);
    
    let sub_cat_manager_procces_1 = purrgress_macros::new_purr_chain!(
        cat_manager,
        MyStage,
        MyStage::Idle : idle_condition =>
        MyStage::Walk : walk_condition =>
        MyStage::Run : run_condition
    );

    sub_cat_manager_procces_1
}

fn add_sub_manager_1(
    cat_manager: &mut manager::StageManager<MyStage>,
    sub_cat_manager_procces_1: MyStage,
) {

    purrgress_macros::purr_tentacle!(
        cat_manager : sub_cat_manager_procces_1,
        MyStage,
        manager::PurrAction::Push : MyStage::Run,
        !manager_types::DuplicatePolicy::RemoveMatch
    );

    purrgress_macros::purr_pounce!(
        cat_manager : sub_cat_manager_procces_1,
        MyStage,
        manager_types::PurrAction::Push,
        !manager_types::DuplicatePolicy::RemoveMatch
    );
}

fn update_sub_manager_1(
    cat_manager: &mut manager::StageManager<MyStage>,
    sub_cat_manager_procces_1: MyStage,
    delta: f32,
) {

    let sub_cat_manager_stage_1 = purrgress_macros::purr_rumble!(
        cat_manager : sub_cat_manager_procces_1,
        MyStage,
        sub_manager_procces_1_func : delta
    );

    if let Some(stage) = sub_cat_manager_stage_1 {
        match stage {
            manager_types::PurrEvent::Idle => println!("The queue sub manager 1 is empty"),
            manager_types::PurrEvent::Running(_) => (),
            manager_types::PurrEvent::Transition { from, to } => {
                   println!("The action {:?} is over", from);

                if let Some(to) = to {
                       println!("Go to {:?}", to);
                };
            },
        };
    };
}

fn sub_manager_procces_1_func(sub_manager_1: &mut manager::StageManager<MyStage>, delta: f32) {

    let timer_staege = [MyStage::Idle, MyStage::Walk, MyStage::Run];

    for stage in timer_staege {
        if let Some(timer) = sub_manager_1.get_condition_mut::<condition::PurrTimer>(stage) {
            timer.tick(delta);
        };
    };
}

fn get_delta_time(last_time: &mut time::Instant) -> f32 {
    
    let current_time = time::Instant::now();
    let delta = current_time.duration_since(*last_time).as_secs_f32();
    *last_time = current_time;

    delta
}

fn create_sub_manager_2(cat_manager: &mut manager::StageManager<MyStage>) -> MyStage {

    let idle_condition = condition::PurrTimer::new(2.0);
    let walk_condition = condition::PurrTimer::new(2.0);
    let run_condition = condition::PurrTimer::new(4.0);

    let sub_cat_manager_procces_2 = purrgress_macros::new_purr_chain!(
        cat_manager,
        MyStage,
        MyStage::Idle : idle_condition =>
        MyStage::Walk : walk_condition =>
        MyStage::Run : run_condition
    );

    sub_cat_manager_procces_2
}

fn add_sub_manager_2(
    cat_manager: &mut manager::StageManager<MyStage>,
    sub_cat_manager_procces_2: MyStage,
) {

    purrgress_macros::purr_tentacle!(
        cat_manager : sub_cat_manager_procces_2,
        MyStage,
        manager_types::PurrAction::Push : MyStage::Run,
        !manager_types::DuplicatePolicy::KeepAll
    );

    purrgress_macros::purr_pounce!(
        cat_manager : sub_cat_manager_procces_2,
        MyStage,
        manager_types::PurrAction::Push,
        !manager_types::DuplicatePolicy::KeepAll
    );
}

fn update_sub_manager_2(
    cat_manager: &mut manager::StageManager<MyStage>,
    sub_cat_manager_procces_2: MyStage,
    delta: f32,
) {

    let sub_cat_manager_stage_2 = purrgress_macros::purr_rumble!(
        cat_manager : sub_cat_manager_procces_2,
        MyStage,
        sub_manager_procces_2_func : delta
        !!manager_types::RumblePolicy::Parallel
    );

    if let Some(stage) = sub_cat_manager_stage_2 {
        match stage {
            manager_types::PurrEvent::Idle => println!("The queue sub manager 2 is empty"),
            manager_types::PurrEvent::Running(_) => (),
            manager_types::PurrEvent::Transition { from, to } => {
                   println!("The action {:?} is over", from);

                if let Some(to) = to {
                       println!("Go to {:?}", to);
                };
            },
        };
    };
}

fn sub_manager_procces_2_func(sub_manager_2: &mut manager::StageManager<MyStage>, delta: f32) {

    let timer_staege = [MyStage::Idle, MyStage::Walk, MyStage::Run];

    for stage in timer_staege {
        if let Some(timer) = sub_manager_2.get_condition_mut::<condition::PurrTimer>(stage) {
            timer.tick(delta);
        };
    }
}

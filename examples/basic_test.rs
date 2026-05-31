use std::time;
use purrgress::cat_stage_manager::*;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MyStage {
    Idle,
    Walk,
    Run
}

impl manager::PurrStep for MyStage {}


fn main() {
    let mut cat_manager = manager::StageManager::new();

    cat_manager.add_to_graph(MyStage::Idle);
    cat_manager.add_to_graph(MyStage::Walk);
    cat_manager.add_to_graph(MyStage::Run);

    cat_manager.add_dependency(MyStage::Idle, MyStage::Walk);
    cat_manager.add_dependency(MyStage::Walk, MyStage::Run);

    let idle_condition = condition::PurrTimer::new(5.0);
    cat_manager.set_condition(MyStage::Idle, Box::new(idle_condition));

    let walk_condition = condition::PurrTimer::new(1.0);
    cat_manager.set_condition(MyStage::Walk, Box::new(walk_condition));

    cat_manager.push(MyStage::Idle, manager::DuplicatePolicy::RemoveMatch);
    cat_manager.push(MyStage::Run, manager::DuplicatePolicy::RemoveMatch);
    cat_manager.insert(MyStage::Walk, manager::DuplicatePolicy::KeepAll, manager::InsertPosition::Index(1));
    println!("{:?}", cat_manager.current_vec_query());

    let mut last_time = time::Instant::now();

    loop {
        let current_time = time::Instant::now();
        let delta = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        if let Some(idle_time) = cat_manager.get_condition_mut::<condition::PurrTimer>(MyStage::Idle) {
            idle_time.tick(delta);
        };

        if let Some(walk_time) = cat_manager.get_condition_mut::<condition::PurrTimer>(MyStage::Walk) {
            walk_time.tick(delta);
        };

        match cat_manager.update() {
            manager::PurrEvent::Idle => (),
            manager::PurrEvent::Running(_) => (),
            manager::PurrEvent::Transition { from, to } => {
                println!("Действие закончено {:?}", from);
                if let Some(to) = to {
                    println!("Перехожу к {:?}", to);
                };
            },
        };

        if cat_manager.len_vec_query() == 0 {
            break;
        };
    }
}
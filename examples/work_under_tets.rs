use purrgress::cat_stage_manager::*;
use purrgress::condition;
use purrgress::types;
use std::time;


// Creating enum
// Создание энума
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MyStage {
    Idle,
    Walk,
    Run
}

impl types::PurrStep for MyStage {}


fn main() {
    let mut cat_manager = manager::StageManager::new();

    // 1. We register the stages in the graph
    // 1. Регистрируем стадии в графе

    cat_manager.add_to_graph(MyStage::Idle);
    cat_manager.add_to_graph(MyStage::Walk);
    cat_manager.add_to_graph(MyStage::Run);

    // 2. Building dependency relationships
    // 2. Строим связи зависимостей

    cat_manager.add_dependency(MyStage::Idle, MyStage::Walk);
    cat_manager.add_dependency(MyStage::Walk, MyStage::Run);

    // 3. We put a one-second timer on the Idle stage
    // 3. Вешаем секундный таймер на стадию Idle

    let idle_condition = condition::PurrTimer::new(1.0);
    cat_manager.set_condition(MyStage::Idle, idle_condition);

    // 4. Push the target stage (Idle and Walk will be added automatically)
    // 4. Пушим целевую стадию (автоматически добавятся Idle и Walk)

    cat_manager.push(MyStage::Idle, types::DuplicatePolicy::RemoveMatch);
    cat_manager.push(MyStage::Run, types::DuplicatePolicy::RemoveMatch);
    cat_manager.insert(MyStage::Walk, types::DuplicatePolicy::KeepAll, types::InsertPosition::Index(1));
    println!("{:?}", cat_manager.current_vec_query());

    let mut last_time = time::Instant::now();

    // 5. Game Loop (Immediate-Mode)
    // 5. Игровой цикл (Immediate-Mode)

    loop {
        let current_time = time::Instant::now();
        let delta = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        // External data pumping (Data Pushing) into the condition
        // Внешняя накачка данных (Data Pushing) в условие

        if let Some(idle_time) = cat_manager.get_condition_mut::<condition::PurrTimer>(MyStage::Idle) {
            idle_time.tick(delta);
        };

        // Update the manager every frame
        // Обновляем менеджер каждый кадр

        match cat_manager.update() {
            types::PurrEvent::Idle => {
                println!("The queue is empty");
                break;
            },
            types::PurrEvent::Running(_) => (),
            types::PurrEvent::Transition { from, to } => {
                println!("The action {:?} is over", from);
                if let Some(to) = to {
                    println!("Go to {:?}", to);
                };
            },
        };
    }
}
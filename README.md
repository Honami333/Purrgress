# purrgress

An immediate-mode stage manager and queue sequencer for Rust with compile-time dependency validation and lightweight dynamic dispatch.

Менеджер стадий и секвенсор очередей в стиле immediate-mode для Rust с валидацией зависимостей на этапе компиляции и легковесной динамической диспетчеризацией.

## Features

* **Immediate-Mode Design** — Every-frame updates (`.update()`), perfect for seamless integration with Bevy, egui, and custom game loops.
* **Graph-Based Dependencies** — Automated calculation and unrolling of task chains powered by a directed acyclic graph (`petgraph`).
* **Zero-Cost Abstractions** — Heavy validation logic is processed during initialization, keeping the runtime flat and ultra-fast.
* **Duplicate Control** — Flexible deduplication policies (`DuplicatePolicy`) and precise queue task positioning (`insert`).
* **Clean Data Pushing** — No heavy runtime contexts in the update method. Data is pushed directly into conditions via mutable downcasting (`get_condition_mut`).

* **Immediate-Mode Design** — Обновление каждый кадр (`.update()`), идеальная интеграция с Bevy, egui и любыми игровыми циклами.
* **Graph-Based Dependencies** — Автоматическое вычисление и разворачивание цепочек задач с помощью направленного графа (`petgraph`).
* **Zero-Cost Abstractions** — Вся тяжелая логика проверок подготавливается на этапе инициализации, рантайм остается плоским и сверхбыстрым.
* **Duplicate Control** — Гибкие политики дедупликации повторов (`DuplicatePolicy`) и ювелирное позиционирование задач в очереди (`insert`).
* **Clean Data Pushing** — Никаких тяжелых рантайм-контекстов в апдейте. Данные засылаются напрямую в условия через даункаст (`get_condition_mut`).

## Usage

Add the library to your `Cargo.toml`:
Добавьте библиотеку в ваш `Cargo.toml`:

```toml
[dependencies]
purrgress = "0.1.0"
```

### Quick Start

A basic example of modeling a cat behavior chain (`Idle -> Walk -> Run`):
Простейший пример моделирования цепочки поведений кота (`Idle -> Walk -> Run`):

```rust
# Work under the hood or the simplest queues

use purrgress::cat_stage_manager::*;
use std::time;


// Creating enum
// Создание энума
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MyStage {
    Idle,
    Walk,
    Run
}

impl manager_types::PurrStep for MyStage {}


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

    cat_manager.push(MyStage::Idle, manager_types::DuplicatePolicy::RemoveMatch);
    cat_manager.push(MyStage::Run, manager_types::DuplicatePolicy::RemoveMatch);
    cat_manager.insert(MyStage::Walk, manager_types::DuplicatePolicy::KeepAll, manager_types::InsertPosition::Index(1));
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
            manager_types::PurrEvent::Idle => {
                println!("The queue is empty");
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

# Convenient work with macros and heavy or nested queues
## MORE DETAILED INFORMATION IN THE MACROS DOCUMENTATION!

use purrgress::cat_stage_manager::*;
use purrgress_macros::{meowphosis, PurrStep};
use std::time;


// Creating enum
// Создание энума
#[meowphosis]
// We indicate derive and attribute
// Указываем derive и attribute
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)] 
pub enum MyStage {
    Idle,
    Walk,
    Run,

    // Add this stage to work with macros
    // Добавьте эту стадию для работы с макросами
    PurrChain(usize)
}

fn main() {
    // Create a manager with all selected stages
    // Создать менеджер со всеми выбранными стадиями
    let mut cat_manager = MyStage::meowphosis_manager();

    // Create all conditions and dependencies inside this function
    // Создание всех условий и зависимостей внтури этой функции
    let sub_cat_manager_procces_1 = create_sub_manager_1(&mut cat_manager);
    // Creating a queue within the SFB manager and adding it to the main queue
    // Создание очереди внутри сфб менеджера и добавление его в очередь основного
    add_sub_manager_1(&mut cat_manager, sub_cat_manager_procces_1);

    let sub_cat_manager_procces_2 = create_sub_manager_2(&mut cat_manager);
    add_sub_manager_2(&mut cat_manager, sub_cat_manager_procces_2);

    println!("{:?}", cat_manager.current_vec_query());

    let mut last_time = time::Instant::now();

    loop {
        let delta = get_delta_time(&mut last_time);

        // Working with stages within a sub-manager by adding a custom function to execute conditions
        // Работа со стадиями внутри саб менеджера путем добавления в него самописаной функции выполнения условий
        update_sub_manager_1(&mut cat_manager, sub_cat_manager_procces_1, delta);
        
        update_sub_manager_2(&mut cat_manager, sub_cat_manager_procces_2, delta);

        // Update the manager every frame
        // Обновляем менеджер каждый кадр
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
        manager::PurrAction::Push,
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
    if let Some(idle_time) = sub_manager_1.get_condition_mut::<condition::PurrTimer>(MyStage::Idle) {
        idle_time.tick(delta);
    };

    if let Some(walk_time) = sub_manager_1.get_condition_mut::<condition::PurrTimer>(MyStage::Walk) {
        walk_time.tick(delta);
    };

    if let Some(walk_time) = sub_manager_1.get_condition_mut::<condition::PurrTimer>(MyStage::Run) {
        walk_time.tick(delta);
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
        manager::PurrAction::Push : MyStage::Run,
        !manager_types::DuplicatePolicy::KeepAll
    );

    purrgress_macros::purr_pounce!(
        cat_manager : sub_cat_manager_procces_2,
        MyStage,
        manager::PurrAction::Push,
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
    if let Some(idle_time) = sub_manager_2.get_condition_mut::<condition::PurrTimer>(MyStage::Idle) {
        idle_time.tick(delta);
    };

    if let Some(walk_time) = sub_manager_2.get_condition_mut::<condition::PurrTimer>(MyStage::Walk) {
        walk_time.tick(delta);
    };

    if let Some(walk_time) = sub_manager_2.get_condition_mut::<condition::PurrTimer>(MyStage::Run) {
        walk_time.tick(delta);
    };
}
```

## Roadmap

- [ ] Version `0.3.0`: Official plugin for `bevy` engine integration.
- [ ] Version `0.4.0`: Three-step pattern for sprite and animation management.
- [ ] Version `0.5.0`: Feature flag for async conditions powered by `tokio`.

- [ ] Версия `0.3.0`: Официальный плагин для интеграции с движком `bevy`.
- [ ] Версия `0.4.0`: Трехступенчатый паттерн для работы со спрайтами и анимациями.
- [ ] Версия `0.5.0`: Feature-флаг для асинхронных условий на базе `tokio`.

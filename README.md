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
purrgress = "0.3.0"
```

### The library includes a built-in animator; to enable it, use features = "animator"
### В библиотеке есть встроенный аниматор, что бы подключить его используйте features = "animator"

### Quick Start

A basic example of modeling a cat behavior chain (`Idle -> Walk -> Run`):
Простейший пример моделирования цепочки поведений кота (`Idle -> Walk -> Run`):

```rust
# Work PurrTrain

use purrgress::{cat_malloc::purr_train::{self, StandardRules}, cat_stage_manager::manager_types::PurrEvent};
use purrgress_macros::PurrStep;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]

pub enum MyStage {
    Idle,
    Walk,
    Run,
    PurrChain(usize)
}

fn main() {
    let mut purr_train = purr_train::PurrTrain::new();

    let mut purr_design = purr_train::PurrDesign::new();

    let mut purr_route = purr_train::PurrRoute::new();

    let mut purr_siding = purr_train::PurrSiding::new();

    design_single(&mut purr_design);

    let purr_chain1box = purr_train::DesignBox::new(
        purr_train::StandardRules::instant(),
        Some(
            vec![
                MyStage::Idle,
                MyStage::Walk,
                MyStage::Run
            ]
        )
    );
    
    purr_design.chain(
        MyStage::PurrChain(1), 
        purr_chain1box
    );

    purr_route.construct_schedule(&purr_design).unwrap();

    purr_siding.launch(MyStage::PurrChain(1), &purr_route).unwrap();

    purr_siding.find_index(MyStage::Run);

    let index_vec = purr_siding.get_switches();

    purr_siding.change_rule(index_vec[0], purr_train::StandardRules::timer(2.0)).unwrap();

    purr_train.attach(&mut purr_siding);

    println!("{purr_train:?}");

    loop {
        rule_update(&mut purr_train);

        let purr_event = purr_train.advance_train();

        if let PurrEvent::Transition { .. } = purr_event {
            println!("{purr_event:?}");
        };

        if purr_event == PurrEvent::Idle { break; };
    };
}

fn rule_update(purr_train: &mut purr_train::PurrTrain<MyStage>) {
    let delta = 0.0006;

    if let Some(first) = purr_train.get_current_mut() {
        match first.rule {
            StandardRules::Timer(_) => first.rule.get_mut_timer().unwrap().tick(delta),
            StandardRules::Flag(_) => first.rule.get_mut_flag().unwrap().set_flag(true),
            _ => ()
        };
    };
}

fn design_single(purr_design: &mut purr_train::PurrDesign<MyStage>) {
    purr_design.single(
        MyStage::Idle, 
        purr_train::StandardRules::timer(2.0),
    );

    purr_design.single(
        MyStage::Walk, 
        purr_train::StandardRules::timer(1.0),
    );

    purr_design.single(
        MyStage::Run, 
        purr_train::StandardRules::timer(1.0),
    );
}

#Working with an animator

use purrgress_macros::{meowphosis, PurrStep};
use purrgress::cat_stage_manager::*;
use purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage;
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

    let animator_meta_data = purrgress_macros::abyssal_grimoire!(
        !!MyStage : <
            idle_ani_manager,
            walk_ani_manager
        >
    );

    let animator_meta_data = purrgress_macros::abyssal_march!(
        !!!animator_meta_data : <
            manager::PurrAction::Push : MyStage::Idle,
            !manager_types::DuplicatePolicy::KeepAll;
        >
    );

    let mut animator_meta_data = purrgress_macros::abyssal_march!(
        !!!animator_meta_data : <
            manager::PurrAction::Push : MyStage::Walk,
            !manager_types::DuplicatePolicy::KeepAll;
        >
    );

    let mut last_time = time::Instant::now();

    loop {
        let delta = get_delta_time(&mut last_time);

        let updated_animator_meta_data = purrgress_macros::purr_rumble_brimstone!(
            !!!animator_meta_data
        );

        if let (Some(stage), Some(sub_satge), Some(index)) = updated_animator_meta_data.0 {
            println!("ani stage: {:?}, ani sub stage: {:?}, stage index: {}", stage, sub_satge, index);
        };

        let animator = updated_animator_meta_data.1.get_animator();

        if animator.query_is_empty() {
            break;
        };

        animator_meta_data = updated_animator_meta_data.1;
    }
}

fn get_delta_time(last_time: &mut time::Instant) -> f32 {
    let current_time = time::Instant::now();
    let delta = current_time.duration_since(*last_time).as_secs_f32();
    *last_time = current_time;

    delta
}
```

## Roadmap

- [ ] Version `0.4.0`: Feature flag for async conditions powered by `tokio`.

- [ ] Версия `0.4.0`: Feature-флаг для асинхронных условий на базе `tokio`.

## ─── ВЫПОЛНЕНО / АРХИВ ───
### [v0.2.0]
* [x] Macros to make working with duplicates and nested stages easier.

* [x] Макросы для облечения работы с дубликатами и вложеными стадиями.

### [v0.3.0]
* [x] Official plugin for `bevy` engine integration. As a label component.
* [x] Three-step pattern for sprite and animation management. Even more! The phases are limited by the user's needs!

* [x] Официальный плагин для интеграции с движком `bevy`. Ввиде компонент метки.
* [x] Трехступенчатый паттерн для работы со спрайтами и анимациями. Даже больше! Фазы ограничены потребностями пользотеля!
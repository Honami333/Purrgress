# Purrgress

### English

* **An immediate-mode stage manager and queue sequencer for Rust with compile-time dependency validation,
static zero-cost architecture, and legacy dynamic dispatch support.

### Русский

* **Менеджер стадий и секвенсор очередей в стиле immediate-mode для Rust с валидацией зависимостей на этапе компиляции,
статической zero-cost архитектурой и поддержкой легаси динамической диспетчеризации.

## Lib Features

### English

* **A major architectural update 0.4.0 that completely rewrites the core internals of the stage and queue manager. The legacy system with recursive nesting remains for backward compatibility, while a parallel, ultra-fast engine has been built from scratch using Data-Oriented Design (DOD). It is tailored for real-time applications and game loops where frame budget and memory footprints are critical.
What was added and changed in simple terms:

* **Linear Trains Instead of Nesting: Nested stages no longer hold internal managers. All dependencies are automatically flattened into a straight line, and the nesting trigger is guaranteed to be pushed to the very end of the sequence.
* **Zero Box and dyn Overhead: All custom execution conditions are packed into a single, static enum of a fixed size. No more dynamic dispatch or heap-hopping inside the hot path of the game loop.
* **Graph Baking: The heavy recursive lookup and cyclic dependency checks run exactly once. Ready-to-go routes are cached, turning runtime initialization into a blazing-fast memory copy. You can still bake new scenarios dynamically mid-game.
* **Isolated Sidings for Preparation (PurrSiding): A dedicated builder buffer manages train assembly. It allows you to extract indices of specific stages without expensive .find() calls and rewrite their default rules or timers before switching to the main track.
* **Cursor-Based Queues: The primary controller (PurrTrain) now utilizes a shift-pointer vector instead of standard array shifting. Completed stages stay in place while the locomotive advances, making stage transitions cost exactly 0 nanoseconds.
* **Leak and Bloat Protection: Smart capacity trimming has been introduced. Once the track is clear and the queue drops to idle, the memory is safely reclaimed by the OS, preventing RAM accumulation over months of non-stop deployment.

### Русский

* **Крупное архитектурное обновление 0.4.0 полностью меняет внутреннее устройство менеджера очередей и стадий. Старая система с вложенными структурами и динамическими проверками типов осталась для совместимости, а рядом написан новый легковесный движок на принципах Data-Oriented Design. Он разработан специально для real-time систем и геймдева, где критически важна скорость кадра и отсутствие утечек ОЗУ.
Что именно добавлено и изменено простым языком:

* **Линейные «паровозики» вместо матрешек: Вложенные стадии больше не хранят в себе другие менеджеры. Теперь все зависимости автоматически разворачиваются в одну прямую линию, а триггер вложенности гарантированно улетает в самый хвост цепочки.
Полный отказ от Box и dyn: Все кастомные условия выполнения стадий упакованы в единый плоский энум фиксированного размера. Никакой динамической диспетчеризации и прыжков по куче (heap) в игровом цикле.
* **Запекание графов (Baking): Тяжелый рекурсивный обход связей и проверка на бесконечные циклы происходят всего один раз. Готовые маршруты кэшируются, а в рантайме они мгновенно копируются в память за один такт. Можно безопасно собирать новые сценарии прямо по ходу игры.
* **Запасные пути для сборки (PurrSiding): Появился специальный изолированный контекст-буфер. В него выгружается шаблон, после чего можно без тяжелого поиска (.find()) быстро собрать индексы нужных вагонов и точечно переписать им таймеры или флаги перед отправкой на главный путь.
* **Очередь без удаления элементов: Главный состав (PurrTrain) работает на базе вектора со сдвигом указателя. Пройденные стадии не сдвигают массив в памяти. Удаление и переход на следующий шаг теперь занимают ровно 0 наносекунд.
* **Защита от раздувания памяти: Внедрена автоматическая очистка емкости по порогу. Когда поезд доезжает до конечной, память физически возвращается операционной системе, что защищает либу от утечек при непрерывной работе в течение месяцев.

## Usage

Add the library to your / Добавьте библиотеку в ваш `Cargo.toml`:

```toml
[dependencies]
purrgress = { version = "0.4.22", features = ["train"] }
```

## Features / Поддерживаемые фичи

### English

By default, all features are disabled so you can compile only what your game loop requires.

* **`train` (Highly Recommended)** — Activates the high-performance Data-Oriented engine. It provides a flat, ultra-fast, zero-cost layout (`PurrTrain`, `PurrDesign`, `PurrRoute`, `PurrSiding`) with O(1) transitions and direct memory reclamation.
* **`animator`** — Enables the built-in stage animator system (automatically pulls the `scrap` core and internal macro generators).
* **`scrap`** — Retains the hierarchical stage manager and queue sequencer for cases requiring deep recursive nesting and runtime dynamics.
* **`bevy_ecs`** — Provides optional ECS integration blocks for Bevy.

### Русский

По умолчанию все фичи выключены. Вы сами выбираете, за какой функционал платить временем компиляции и байтами в бинарнике.

* **`train` (Рекомендуется)** — Включает высокопроизводительный Data-Oriented движок. Обеспечивает плоскую, сверхбыструю zero-cost архитектуру (`PurrTrain`, `PurrDesign`, `PurrRoute`, `PurrSiding`) со скоростью переходов O(1) и прямым возвратом памяти операционной системе.
* **`animator`** — Подключает встроенную систему анимации стадий (под капотом автоматически задействует архитектуру `scrap` и внутренние генераторы макросов).
* **`scrap`** — Оставляет иерархический менеджер стадий и секвенсор очередей для сценариев, требующих глубокой вложенности и высокой динамики в рантайме.
* **`bevy_ecs`** — Добавляет опциональную интеграцию с ECS-системой Bevy.

## Quick Start

### Work PurrTrain

```rust
use purrgress::cat_malloc::purr_train;
use purrgress::cat_malloc::train_design;
use purrgress::cat_malloc::train_route;
use purrgress::cat_malloc::train_siding;
use purrgress::cat_malloc::train_types;

use purrgress::cat_malloc::train_types::{PurrRule, BufferMode};
use purrgress::types::PurrEvent;
use purrgress::condition;

use purrgress_macros::PurrStep;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
pub enum MyStage {
    Idle,
    Walk,
    Run,
    IWRChain
}

fn main() {
    let mut purr_train = purr_train::PurrTrain::new();

    let mut purr_design = train_design::PurrDesign::new();

    let mut purr_route = train_route::PurrRoute::new();

    let mut purr_siding = train_siding::PurrSiding::new();

    design_single(&mut purr_design);

    let purr_iwr_chain = train_design::DesignBox::new(
        train_types::StandardRules::instant(),
        Some( vec![MyStage::Idle, MyStage::Walk, MyStage::Run] )
    );
    
    purr_design.chain(MyStage::IWRChain, purr_iwr_chain);

    purr_route.construct_schedule(&purr_design).unwrap();

    purr_siding.launch(MyStage::IWRChain, BufferMode::Clear, &purr_route).unwrap();

    purr_siding.find_index(MyStage::Run, BufferMode::Clear);

    let index_vec = purr_siding.get_switches();

    purr_siding.change_rule(index_vec[0], train_types::StandardRules::timer(2.0)).unwrap();

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

fn rule_update(purr_train: &mut purr_train::PurrTrain<MyStage, train_types::StandardRules>) {
    let delta = 0.00000006;

    if let Some(first) = purr_train.get_current_mut() {
        if let Some(timer) = first.rule.as_mut_rule::<condition::PurrTimer>() {
            timer.tick(delta);
        };

        if let Some(flag) = first.rule.as_mut_rule::<condition::PurrFlag>() {
            flag.set_flag(true);
        };
    };
}

fn design_single(purr_design: &mut train_design::PurrDesign<MyStage, train_types::StandardRules>) {
    purr_design.single(MyStage::Idle, train_types::StandardRules::timer(2.0));

    purr_design.single(MyStage::Walk, train_types::StandardRules::timer(1.0));

    purr_design.single(MyStage::Run, train_types::StandardRules::timer(1.0));
}
```

### Working with an animator

```rust
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

- [ ] Version `0.5.0`: Adaptation of the animator for the steam locomotive system, as well as other cool systems.

- [ ] Версия `0.5.0`: Адаптация аниматора под систему паровозов, а также другие прикольные системы.

## ─── ВЫПОЛНЕНО / АРХИВ ───
### [v0.2.0]
* [x] Macros to make working with duplicates and nested stages easier.

* [x] Макросы для облечения работы с дубликатами и вложеными стадиями.

### [v0.3.0]
* [x] Official plugin for `bevy` engine integration. As a label component.
* [x] Three-step pattern for sprite and animation management. Even more! The phases are limited by the user's needs!

* [x] Официальный плагин для интеграции с движком `bevy`. Ввиде компонент метки.
* [x] Трехступенчатый паттерн для работы со спрайтами и анимациями. Даже больше! Фазы ограничены потребностями пользотеля!

### [v0.4.0]
* [x] A complete redesign of the manager: everything is now linear.
* [x] Complete elimination of macro dependencies, making code easier to read and write.

* [x] Полная переработка менеджера: перевод все на линейные паровозики.
* [x] Полное избавление от макросо зависимости, облегчение читаемости и написания кода.
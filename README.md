# Purrgress

### English

**A smart DOD-style stage manager that squeezes out maximum speed while retaining 100% dynamism and convenience. It positions itself as a zero-cost architecture, yet supports legacy code written in the spirit of OOP patterns.**

Tailored for real-time applications and game loops where frame budget and memory footprints are critical. 
* **Linear Trains:** Nested stages are flattened into a straight line.
* **Zero Box & dyn Overhead:** Custom conditions are packed into a static enum. No dynamic dispatch in the hot path.
* **Graph Baking:** Heavy recursive lookups run once. Ready routes are cached and copied in O(1).
* **Isolated Sidings (`PurrSiding`):** A dedicated buffer for train assembly and modification before switching to the main track.
* **Cursor-Based Queues:** Completed stages stay in place while the locomotive advances (0 ns transitions).

### Русский

**Умный менеджер стадий в стиле DOD, выжимающий максимум скорости при сохранении 100% динамичности и удобства. Позиционирует себя как zero-cost архитектура, но поддерживает легаси-код, написанный в духе ООП-паттернов.**

Разработан для real-time систем и геймдева, где критически важна скорость кадра и отсутствие утечек ОЗУ.
* **Линейные «паровозики»:** Все зависимости разворачиваются в прямую линию, триггер вложенности уходит в конец.
* **Отказ от Box и dyn:** Условия упакованы в плоский энум. Никаких прыжков по куче в игровом цикле.
* **Запекание графов:** Тяжелый обход связей происходит один раз. В рантайме маршруты копируются за один такт.
* **Запасные пути (`PurrSiding`):** Изолированный буфер для точечной сборки и перезаписи таймеров/флагов вагонов.
* **Очередь на курсоре:** Пройденные стадии не сдвигают массив. Переход на следующий шаг занимает 0 наносекунд.

## A major update 

### English

* **A major update 0.5 has been released. You can check it out in the Releases tab.

### Русский

* **Было выпущено крупное обновление 0.5, ознакомиться можно во вкладке Releases.

## Feedback & Support

### English

* **The author is always open to questions and feedback in their Telegram channel: https://t.me/cat_garden_news

### Русский

* **Автор всегда открыт к ответам в своем Telegram-канале: https://t.me/cat_garden_news

## Usage

Add the library to your / Добавьте библиотеку в ваш `Cargo.toml`:

```toml
[dependencies]
purrgress = { version = "0.5.11", features = ["train"] }
```

## Features / Поддерживаемые фичи

### English

By default, all features are disabled. You choose yourself which functionality to pay for with compile time.
By default, all features are disabled.

* **train (Highly Recommended)** - Enables the high-performance Data-Oriented engine
(PurrTrain, PurrDesign, PurrRoute, PurrSiding) with O(1) transition speed.
* **dispatcher** - Asynchronous command handler, actor model, integration with tokio, rkyv and bytes.
* **animator** - Built-in stage frame processing system (automatically pulls scrap and macros).
* **scrap** - Hierarchical stage manager for scenarios requiring deep nesting and runtime dynamics.
* **bevy_ecs** - Optional integration with the Bevy ECS system.
* **serde** - Support for optional serialization of base structures.
* **rkyv** - Support for optional serialization of almost everything in the library via the crate of the same name.

### Русский

По умолчанию все фичи выключены. Вы сами выбираете, за какой функционал платить временем компиляции.
By default, all features are disabled.

* **train (Рекомендуется / Highly Recommended)** - Включает высокопроизводительный Data-Oriented движок
(PurrTrain, PurrDesign, PurrRoute, PurrSiding) со скоростью переходов O(1).
* **dispatcher** - Асинхронный обработчик команд, акторная модель, интеграция с tokio, rkyv и bytes.
* **animator** - Встроенная система обработки кадров стадий (автоматически тянет scrap и макросы).
* **scrap** - Иерархический менеджер стадий для сценариев, требующих глубокой вложенности и динамики в рантайме.
* **bevy_ecs** - Опциональная интеграция с ECS-системой Bevy.
* **serde** - Поддержка опциональной сериализации базовых структур.
* **rkyv** - Поддержка опциональной сериализации почти всего в библиотеки через одименный крейт

## Quick Start

### Work PurrTrain

```rust
use purrgress::cat_malloc::purr_train;
use purrgress::cat_malloc::train_design;
use purrgress::cat_malloc::train_route;
use purrgress::cat_malloc::train_siding;
use purrgress::cat_malloc::train_types;

use purrgress::cat_malloc::train_types::{PurrRule, BufferMode};
use purrgress::condition;
use purrgress::PurrStep;

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

    let mut purr_route = train_route::PurrRoute::new(8);

    let mut purr_siding = train_siding::PurrSiding::new(8);

    design_single(&mut purr_design);
    
    purr_design.new_chain(MyStage::IWRChain, train_types::StandardRules::instant(), vec![MyStage::Idle, MyStage::Walk, MyStage::Run]);

    purr_route.construct_schedule(&purr_design, BufferMode::Keep).unwrap();

    purr_siding.launch(MyStage::IWRChain, BufferMode::Clear, &purr_route).unwrap();

    purr_siding.find_index(MyStage::Run, BufferMode::Clear);

    let index_vec = purr_siding.get_switches();

    purr_siding.change_rule(index_vec[0], train_types::StandardRules::timer(2.0)).unwrap();


    println!("{purr_train:?}");

    loop {
        rule_update(&mut purr_train);

        let purr_event = purr_train.advance_train();

        if let train_types::PurrTrainEvent::Transition { .. } = purr_event {
            println!("{purr_event:?}");
        };

        if purr_event == train_types::PurrTrainEvent::Idle {
            purr_siding.launch(MyStage::IWRChain, BufferMode::Clear, &purr_route).unwrap();
            purr_train.attach(&mut purr_siding);
        };

        purr_train.shrink_line(10000);
    };
}

fn rule_update(purr_train: &mut purr_train::PurrTrain<MyStage, train_types::StandardRules>) {
    let delta = 0.00000006;

    if let Some(route_box) = purr_train.get_current_mut() {
        if let Some(timer) = route_box.rule.as_mut_rule::<condition::PurrTimer>() {
            timer.tick(delta);
        };

        if let Some(flag) = route_box.rule.as_mut_rule::<condition::PurrFlag>() {
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

### Work PurrDispatcher

``` rust
use purrgress::cat_malloc::train_design;
use purrgress::cat_malloc::train_route;
use purrgress::cat_malloc::train_siding;
use purrgress::cat_malloc::train_types::{self, PurrRule};
use purrgress::cat_telegraph::dispatcher;
use purrgress::cat_telegraph::dispatcher_types;
use purrgress::cat_telegraph::station_link;
use purrgress::cat_telegraph::dispatcher_condition::{TrackRule, RunTimer, WaitTimer};
use purrgress::PurrStep;

use rkyv::{Archive, Deserialize, Serialize};
use tokio::sync::mpsc::Sender;


// Все тот же абсолютно любой энум стадий
// The exact same arbitrary stage enum
#[derive(Archive, Deserialize, Serialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
pub enum MyStage {
    Connect,
    Waiting,
    Disconnect
}

// Мой юнум ключей, важно нужно иметь default. Трейт ключа реализуется автоматически
// Советую делать дефолтом пустую стадию
// My key enum, it is important to have default. Key trait is implemented automatically
// I recommend setting an empty stage as default
#[derive(Archive, Deserialize, Serialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MyKey {
    #[default]
    None,
    Running,
    Waiting
}


#[tokio::main]
async fn main() {
    // Обязательно замеряйте delta но тут тест
    // Be sure to measure delta, but this is a test
    let delta = 0.0001;
    // Создаем единый диспетчер работы поезда и линк для внешних поключений
    // Create a single train dispatcher and a link for external connections
    let (mut dispatcher, mut link) = dispatcher::PurrDispatcher::<MyStage, TrackRule<MyKey>, MyKey>::new(32);
    tokio::spawn(async move {
        // Асихронная задача токио с циклом опрашивания диспечера
        // Tokio async task with a dispatcher polling loop
        loop {
            // Диспечер постоянно ловит команды возращая ключ станции от которой она пришла
            // Dispatcher continuously catches commands, returning the station key it came from
            match dispatcher.dispatch(dispatcher_types::RouteAction::Default).await {
                Ok(Some(key)) => {
                    // Лучше всего обрабатывать условия именно тут но опять же важно замерять delta
                    // It's best to handle conditions right here, but again, measuring delta is important
                    if let Some(route_box) = dispatcher.purr_train.get_current_mut() {
                        // Условия которые изначально были кастомными но перешли внутрь либы (Почти взаимо заменяемы)
                        // Первое бежит и дает время на просмотр после просмотра или по истечению второго таймера уничтожается
                        // Conditions that were originally custom but moved inside the library (Almost interchangeable)
                        // The first runs and grants time for viewing; after viewing or upon expiration of the second timer, it is destroyed
                        if let Some(run) = route_box.rule.as_mut_rule::<RunTimer<MyKey>>() {
                            run.tick(delta);
                            run.is_key_fast(key);
                        };
                        // Второе именно ждет пока на него не откликнется именно нужный ключ
                        // На всякий случай был сделан таймер максимального ожидания, но если вам не нужно то просто поставте большое число
                        // The second waits specifically until the correct key responds to it
                        // A maximum wait timer was made just in case, but if you don't need it, just set a large number
                        if let Some(wait) = route_box.rule.as_mut_rule::<WaitTimer<MyKey>>() {
                            wait.tick(delta);
                            wait.set_flag(key, true);
                        };
                    };
                },
                Err(error) => {
                    println!("{:?}", error);
                    break;
                }
                _ => {}
            };
        };
    });

    // Обычное конструирование чертежа
    // Для бегущего клиента
    // Standard design construction
    // For running client
    let mut run_disign = train_design::PurrDesign::new();
    run_disign.single(MyStage::Connect, TrackRule::run(3.0, 1.0, MyKey::Running));
    run_disign.single(MyStage::Waiting, TrackRule::run(3.0, 1.0, MyKey::Running));
    run_disign.new_chain(
        MyStage::Disconnect,
        TrackRule::run(3.0, 2.0, MyKey::Running),
        vec![MyStage::Connect, MyStage::Waiting, MyStage::Waiting]
    );

    // Для ждущего клиента
    // For waiting client
    let mut wait_disign = train_design::PurrDesign::new();
    wait_disign.single(MyStage::Connect, TrackRule::wait(1.0, 10.0, MyKey::Waiting));
    wait_disign.single(MyStage::Waiting, TrackRule::wait(1.0, 10.0, MyKey::Waiting));
    wait_disign.new_chain(
        MyStage::Disconnect,
        TrackRule::wait(2.0, 10.0, MyKey::Waiting),
        vec![MyStage::Connect, MyStage::Waiting]
    );

    // Обычное запикание шаблонов
    // Standard baking of templates
    let mut run_route = train_route::PurrRoute::new(32);
    run_route.construct_schedule(&run_disign, train_types::BufferMode::Keep).unwrap();
    let mut wait_route = train_route::PurrRoute::new(32);
    wait_route.construct_schedule(&wait_disign, train_types::BufferMode::Keep).unwrap();

    // Через линк обращаемся к диспетчеру и просим зарегистровать ключи в ответ получая канал общения для команд
    // Via link, we contact dispatcher and request key registration, receiving a communication channel for commands in return
    let run_route_link = link.new_route(MyKey::Running, dispatcher_types::RouteAction::Default, 32).await.unwrap();
    let walk_route_link = link.new_route(MyKey::Waiting, dispatcher_types::RouteAction::Default, 32).await.unwrap();

    let client1 = tokio::spawn(run_client(walk_route_link, wait_route, link.new_line(), MyKey::Waiting));
    // Для видимости пауза
    // Pause for visibility
    tokio::time::sleep(std::time::Duration::from_secs_f32(0.1)).await;
    let client2 = tokio::spawn(run_client(run_route_link, run_route, link.new_line(), MyKey::Running));


    let _ = tokio::join!(client1, client2);
}

pub async fn run_client(
    route_link: Option<dispatcher_types::DispatcherReply<MyStage, TrackRule<MyKey>>>,
    purr_route: train_route::PurrRoute<MyStage, TrackRule<MyKey>>,
    line: Sender<dispatcher_types::DispatcherCommand<MyKey>>,
    key: MyKey
) -> Result<(), dispatcher_types::PurrError> {
    println!("Clien init: {:?}", key);
    let mut siding = train_siding::PurrSiding::new(32);

    // Выгружаем цепочку в буфер
    // Offloading the chain into buffer
    siding.launch(MyStage::Disconnect, train_types::BufferMode::Clear, &purr_route).unwrap();

    if let Some(dispatcher_types::DispatcherReply::PurrStation { rx_reply }) = route_link {
        // Создаем новую станцию line это просто копия из линка
        // Create a new station; line is just a copy from link
        let mut station = station_link::PurrStation::new(line, rx_reply, key);

        // Для максимальной скорости был выбран метод серелизации через rkyv в Bytes
        // Важно читайте предупреждение!
        // Serialization via rkyv into Bytes was chosen for maximum speed
        // Important: read the warning!
        let bytes = station.siding_to_byte(&mut siding)?;
        // Отправляем команду о прикреплении вагона к основному поезду
        // Send command to attach carriage to main train
        let _ = station.send_command(dispatcher_types::DispatcherCommand::Attach { siding_data: bytes, key: station.get_key() }).await?;

        loop {
            // Запрашиваю текущую стадию что бы сделать проверки
            // Requesting current stage to perform checks
            let reply = station.send_command(dispatcher_types::DispatcherCommand::GetCurrent { key }).await?;
            let mut advance_reply = None;

            // В ответ я получаю route box 
            // In response I get route box
            if let Some(dispatcher_types::DispatcherReply::Current { route_box }) = reply
                && let Some(route_box) = route_box {
                
                // Достаю нужное мне условие что бы сделать проверки на совпадание
                // Если это мое условие или время обработки чужего превышено отправлю команду на движение поезда
                // Важно если отправлять команды по соответвию начнется хаос
                // Подсказка для большого количества условий можно использовать match
                
                // Extracting needed condition to run matching checks
                // If this is my condition or someone else's processing time exceeded, I send train advance command
                // Important: sending commands on match will result in chaos
                // Hint: for a large number of conditions, you can use match
                if let Some(timer) = route_box.rule.as_ref_rule::<RunTimer<MyKey>>() && ((key == MyKey::Running && timer.is_key(key)) || timer.overflow() ) {
                    advance_reply = station.send_command(dispatcher_types::DispatcherCommand::AdvanceTrain { key }).await?;
                };
                if let Some(timer) = route_box.rule.as_ref_rule::<WaitTimer<MyKey>>() && ((key == MyKey::Waiting && timer.is_key(key)) || timer.overflow()) {
                    advance_reply = station.send_command(dispatcher_types::DispatcherCommand::AdvanceTrain { key }).await?;
                };
            };

            // Перерь поезд возращает route box выполненой стадии (move), обработка осталась стандарной
            // Now train returns route box of completed stage (move), processing remains standard
            if let Some(dispatcher_types::DispatcherReply::Advance { event }) = advance_reply {
                match event {
                    train_types::PurrTrainEvent::Idle => break,
                    train_types::PurrTrainEvent::Transition { from, to } => {
                        if let Some(to) = to {
                            println!("State: {:?} next: {:?} my key: {:?}", from.carriage, to.carriage, key);
                        } else {
                            println!("State: {:?} my key: {:?}", from.carriage, key);
                        };
                        
                        match (from.carriage, from.rule, key) {
                            (MyStage::Disconnect, TrackRule::RunTimer(timer), MyKey::Running) if timer.is_key(key) => break,
                            (MyStage::Disconnect, TrackRule::WaitTimer(timer), MyKey::Waiting) if timer.is_key(key) => break,
                             _ => {}
                        };
                    }
                    _ => {}
                };
            };
        };
    };
    Ok(())
}
```

## Roadmap

- [ ] Version `0.6.0`: Implementation of a custom circular vector to fix the main problem of CursorVector.
- [ ] Версия `0.6.0`: Further deserialization optimizations: conditions are now 100% Copy, enabling guaranteed extraction with near-safe zero-overhead unwrapping.

- [ ] Версия `0.6.0`: Реализация кастомного зацикленного вектора для фикса главной проблемы CursorVector.
- [ ] Версия `0.6.0`: Еще большая оптимизация дисереализации, так как условия стали 100% копированием + гарантированное извлечение хоть и почти бесопасного, но разворачивания. и это

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

### [v0.5.0]
* [x] Massive work in the form of bug fixing, laying the groundwork for new systems,
as well as large-scale integration with tokio in the form of an asynchronous dispatcher, all in the same DOD style,
trying to squeeze out maximum speed through the train architecture.
* [x] Мощная работа в виде багофикса, подготовки "почвы" под новые системы,
а также масштабная интеграция с tokio в виде асинхронного диспетчера — всё в том же DOD-стиле,
пытающемся выжать максимум скорости за счет архитектуры поездов.
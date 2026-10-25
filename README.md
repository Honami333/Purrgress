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
purrgress = { version = "0.6.0", features = ["train"] }
```

## Features / Поддерживаемые фичи

### English

By default, all features are disabled. You choose yourself which functionality to pay for with compile time.
By default, all features are disabled.

* **train (Highly Recommended)** - Enables the high-performance Data-Oriented engine
(PurrTrain, PurrDesign, PurrRoute, PurrSiding) with O(1) transition speed.
* **dispatcher** - Asynchronous command handler, actor model, integration with tokio.
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
* **dispatcher** - Асинхронный обработчик команд, акторная модель, интеграция с tokio.
* **animator** - Встроенная система обработки кадров стадий (автоматически тянет scrap и макросы).
* **scrap** - Иерархический менеджер стадий для сценариев, требующих глубокой вложенности и динамики в рантайме.
* **bevy_ecs** - Опциональная интеграция с ECS-системой Bevy.
* **serde** - Поддержка опциональной сериализации базовых структур.
* **rkyv** - Поддержка опциональной сериализации почти всего в библиотеки через одименный крейт

## Quick Start

### Work PurrTrain


## Roadmap

- [ ] Версия `0.7.0`: Реализация кастомного зацикленного вектора для фикса главной проблемы CursorVector.
- [ ] Version `0.7.0`: Implementation of a custom circular vector to fix the main problem of CursorVector.

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

### [v0.6.0]
* [x] Полный слом протокола команд диспетчера: `Attach` / `Replace` / `RerouteAt` больше не работают с байтами и `rkyv`, а принимают `carriage: T` напрямую — `bytes` и `rkyv` полностью удалены как стандарт для диспетчера, раздел `train_codec` вырезан из либы целиком.
* [x] `dispatch()` стал более глупым и универсальным: вместо `Option<C>` возвращает `DispatchData<T, C>` со всеми возможными данными команды, а `Attach` / `Replace` / `RerouteAt` теперь на полной ответственности пользователя.
* [x] `RouteAction` и `DispatcherCommand` теперь `Copy` — гарантированное дешёвое извлечение данных из команд без лишних клонирований.
* [x] Переработка `PurrSiding`: `main_train` переименован в `main_line`, поле `switches` и вся работа с ним (`find_index`, `find_index_few`, `find_index_many`, `get_switches`, `clear_switches`) полностью вырезаны, взамен — умные методы `change_rule` и `change_rule_few`.
* [x] `PurrDesign`: `chain` и `new_chain` поменялись местами (переименованы) под более ликвидную форму добавления.
* [x] Введены `TrainError<T>` / `TrainResult<V, T>` для всего train-раздела, `PurrError` переименован в `DispatcherError` с новыми вариантами `LineChannelClosed` и `RouteChannelClosed`.
* [x] `launch` переведён на `extend_from_slice` вместо `extend(branch.clone())` благодаря `Copy` на `RouteBox`.

* [x] Complete break of the dispatcher's command protocol: `Attach` / `Replace` / `RerouteAt` no longer work with bytes or `rkyv`, taking `carriage: T` directly — `bytes` and `rkyv` fully removed as a standard for the dispatcher, `train_codec` section cut from the lib entirely.
* [x] `dispatch()` got dumber and more generic: instead of `Option<C>` it now returns `DispatchData<T, C>` with all possible command data, while `Attach` / `Replace` / `RerouteAt` are now fully on the user's responsibility.
* [x] `RouteAction` and `DispatcherCommand` are now `Copy` — guaranteed cheap extraction from commands without extra cloning.
* [x] `PurrSiding` overhaul: `main_train` renamed to `main_line`, the `switches` field and all related work (`find_index`, `find_index_few`, `find_index_many`, `get_switches`, `clear_switches`) fully cut, replaced by smarter `change_rule` and `change_rule_few` methods.
* [x] `PurrDesign`: `chain` and `new_chain` swapped names for a more fitting form of addition.
* [x] Introduced `TrainError<T>` / `TrainResult<V, T>` across the whole train section, `PurrError` renamed to `DispatcherError` with new `LineChannelClosed` and `RouteChannelClosed` variants.
* [x] `launch` switched to `extend_from_slice` instead of `extend(branch.clone())` thanks to `Copy` on `RouteBox`.
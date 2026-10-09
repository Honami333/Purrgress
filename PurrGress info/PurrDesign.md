Компактный список изменений по `train_design.rs` (PurrDesign / DesignBox) за эту сессию — для CHANGELOG:

**API / поведение**

- Добавлен `try_single` — то же, что `single`, но с проверкой на дублирующийся `carriage`, возвращает `TrainResult<(), T>`.
- Добавлен `try_chain` — то же, что `chain`, но с проверкой дублирования carriage **и** валидацией coupling через `validated_set_coupling` (раньше `try_chain` мог принять пустой/невалидный coupling в обход проверки — исправлено).
- Добавлен `validated_set_coupling` на `DesignBox` — проверяет: coupling уже задан → `CouplingAlreadyDefined`; coupling пустой → `EmptyCoupling`. Порядок проверок фиксирован: `CouplingAlreadyDefined` имеет приоритет над `EmptyCoupling`.
- Добавлены `contains_parameter`, `coupling_is_some`, `coupling_is_none` — публичные predicate-методы.
- `add_blueprints` — явный публичный raw-метод вставки `DesignBox` (вынесен как переиспользуемый примитив для `single`/`chain`/`try_single`/`try_chain`).
- Переименована ошибка: `TrainError::CoupledCarriage` → `TrainError::DuplicateCarriage { carriage: T }`.
- Добавлена ошибка `TrainError::CouplingAlreadyDefined`.
- Добавлена ошибка `TrainError::EmptyCoupling`.
- Зафиксировано и задокументировано (не исправлено, оставлено намеренно): `single`, `chain`, `add_blueprints`, `set_coupling` — raw-методы, молча перезаписывают существующее значение без ошибки.
- Зафиксировано: `set_coupling` принимает пустой `Vec` и пишет его как `Some(Vec::new())` — отдельное состояние от `None`.

**Документация**

- Добавлены `///` doc-комментарии на оба публичных struct'а (`PurrDesign`, `DesignBox`) и на все публичные методы, включая `# Warning`/`# Errors` секции с описанием поведения при дублировании/перезаписи/приоритете ошибок.

**Тесты**

- Добавлено 15 unit-тестов, покрывающих: успешную вставку через `try_single`/`try_chain`; ошибку на дублирующийся carriage в обоих; ошибку на пустой coupling в `try_chain` (+ подтверждение отсутствия частичной вставки); тихую перезапись в `single`/`chain` (задокументированное поведение, не баг); все сценарии `contains_parameter` (пусто / есть другие ключи / есть искомый); успешную и конфликтную установку coupling через `validated_set_coupling` (+ подтверждение, что старое значение не тронуто при конфликте); приоритет `CouplingAlreadyDefined` над `EmptyCoupling`; unchecked-поведение `set_coupling` с пустым вектором.

**Решение по архитектуре (зафиксировано, не выполнено в этом файле)**

- Принято решение полностью убрать v1 (`cat_stage_manager`, dyn-based) из следующего релиза; аниматор, построенный поверх v1, потребует портирования на v2 или будет выведен из крейта вместе с v1 — детали ещё не определены.
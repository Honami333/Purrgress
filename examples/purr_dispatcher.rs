use purrgress::cat_malloc::train_design;
use purrgress::cat_malloc::train_route;
use purrgress::cat_malloc::train_siding;
use purrgress::cat_malloc::train_types::{self, PurrRule};
use purrgress::cat_telegraph::dispatcher;
use purrgress::cat_telegraph::dispatcher_types;
use purrgress::cat_telegraph::station_link;
use purrgress::cat_telegraph::dispatcher_condition::{TrackRule, RunTimer, WaitTimer};
use purrgress_macros::PurrStep;

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
    let run_box = train_design::DesignBox::new(
        TrackRule::run(3.0, 2.0, MyKey::Running),
        Some(vec![MyStage::Connect, MyStage::Waiting, MyStage::Waiting])
    );
    run_disign.chain(MyStage::Disconnect, run_box);

    // Для ждущего клиента
    // For waiting client
    let mut wait_disign = train_design::PurrDesign::new();
    wait_disign.single(MyStage::Connect, TrackRule::wait(1.0, 10.0, MyKey::Waiting));
    wait_disign.single(MyStage::Waiting, TrackRule::wait(1.0, 10.0, MyKey::Waiting));
    let wait_box = train_design::DesignBox::new(
        TrackRule::wait(2.0, 10.0, MyKey::Waiting),
        Some(vec![MyStage::Connect, MyStage::Waiting])
    );
    wait_disign.chain(MyStage::Disconnect, wait_box);

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
    // Для максимальной скорости был выбран метод серелизации через rkyv в Bytes
    // Важно читайте предупреждение!
    // Serialization via rkyv into Bytes was chosen for maximum speed
    // Important: read the warning!
    let bytes = station_link::PurrStation::<MyStage, TrackRule<MyKey>, MyKey>::siding_to_byte(&mut siding)?;

    if let Some(dispatcher_types::DispatcherReply::PurrStation { rx_reply }) = route_link {
        // Создаем новую станцию line это просто копия из линка
        // Create a new station; line is just a copy from link
        let mut station = station_link::PurrStation::new(line, rx_reply, key);
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
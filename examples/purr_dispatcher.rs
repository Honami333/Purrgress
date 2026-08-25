use purrgress::cat_malloc::train_design;
use purrgress::cat_malloc::train_route;
use purrgress::cat_malloc::train_siding;
use purrgress::cat_malloc::train_types::{self, PurrRule};
use purrgress::cat_telegraph::dispatcher;
use purrgress::cat_telegraph::dispatcher_types;
use purrgress::cat_telegraph::dispatcher_types::DispatcherCommand;
use purrgress::cat_telegraph::station_link;
use purrgress::cat_telegraph::dispatcher_condition::{TrackRule, RunTimer, WaitTimer};
use purrgress::PurrStep;

use tokio::sync::mpsc::Sender;

use wibr::get;
use wibr::iff;


// Все тот же абсолютно любой энум стадий
// The exact same arbitrary stage enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
pub enum MyStage {
    RunConnect,
    RunWaiting,
    RunDisconnect,
    WaitConnect,
    WaitWaiting,
    WaitDisconnect
}

// Мой юнум ключей, важно нужно иметь default. Трейт ключа реализуется автоматически
// Советую делать дефолтом пустую стадию
// My key enum, it is important to have default. Key trait is implemented automatically
// I recommend setting an empty stage as default
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
        // Обычное конструирование чертежа
        // Standard design construction
        let mut disign = train_design::PurrDesign::new();
        // Для бегущего клиента

        // For running client
        disign.single(MyStage::RunConnect, TrackRule::run(3.0, 1.0, MyKey::Running));
        disign.single(MyStage::RunWaiting, TrackRule::run(3.0, 1.0, MyKey::Running));
        disign.chain(
        MyStage::RunDisconnect,
        TrackRule::run(3.0, 2.0, MyKey::Running),
        vec![MyStage::RunConnect, MyStage::RunWaiting, MyStage::RunWaiting]
        );

        // Для ждущего клиента
        // For waiting client
        disign.single(MyStage::WaitConnect, TrackRule::wait(1.0, 10.0, MyKey::Waiting));
        disign.single(MyStage::WaitWaiting, TrackRule::wait(1.0, 10.0, MyKey::Waiting));
        disign.chain(
            MyStage::WaitDisconnect,
            TrackRule::wait(2.0, 10.0, MyKey::Waiting),
            vec![MyStage::WaitConnect, MyStage::WaitWaiting]
        );

         // Обычное запикание шаблонов
        // Standard baking of templates
        let mut route = train_route::PurrRoute::new(32);
        route.construct_schedule(&disign, train_types::BufferMode::Keep).unwrap();

        let mut siging = train_siding::PurrSiding::new(32);

        // Асихронная задача токио с циклом опрашивания диспечера
        // Tokio async task with a dispatcher polling loop
        loop {
            // Диспечер постоянно ловит команды возращая ключ станции от которой она пришла
            // Dispatcher continuously catches commands, returning the station key it came from
            match dispatcher.dispatch(dispatcher_types::RouteAction::Default).await {
                Ok(dispatcher_data) => {
                    let key = get!{dispatcher_data.key ; continue };
                    // Лучше всего обрабатывать условия именно тут но опять же важно замерять delta
                    // It's best to handle conditions right here, but again, measuring delta is important
                    if let Some(route_box) = dispatcher.purr_train.get_current_mut() {
                        match &mut route_box.rule {
                            // Условия которые изначально были кастомными но перешли внутрь либы (Почти взаимо заменяемы)
                            // Первое бежит и дает время на просмотр после просмотра или по истечению второго таймера уничтожается
                            // Conditions that were originally custom but moved inside the library (Almost interchangeable)
                            // The first runs and grants time for viewing; after viewing or upon expiration of the second timer, it is destroyed
                            TrackRule::RunTimer(run_timer) => {
                                run_timer.tick(delta);
                                run_timer.is_key_fast(key);
                            },
                            // Второе именно ждет пока на него не откликнется именно нужный ключ
                            // На всякий случай был сделан таймер максимального ожидания, но если вам не нужно то просто поставте большое число
                            // The second waits specifically until the correct key responds to it
                            // A maximum wait timer was made just in case, but if you don't need it, just set a large number
                            TrackRule::WaitTimer(wait_timer) => {
                                wait_timer.tick(delta);
                                wait_timer.set_flag(key, true);
                            },
                            _ => {}
                        };
                    };
                    if let Some(carriage) = dispatcher_data.carriage {
                        siging.launch(carriage, train_types::BufferMode::Clear, &route).unwrap();
                    };
                    if let Some(command) = dispatcher_data.command {
                        match command {
                            DispatcherCommand::Attach { .. } => dispatcher.purr_train.attach(&mut siging),
                            DispatcherCommand::Replace { .. } => dispatcher.purr_train.replace(&mut siging),
                            DispatcherCommand::RerouteAt { position, .. } => dispatcher.purr_train.reroute_at(&mut siging, position),
                            _ => {}
                        };
                    };
                },
                Err(error) => {
                    println!("{:?}", error);
                    break;
                }
            };
        };
    });

    // Через линк обращаемся к диспетчеру и просим зарегистровать ключи в ответ получая канал общения для команд
    // Via link, we contact dispatcher and request key registration, receiving a communication channel for commands in return
    let run_route_link = link.new_route(MyKey::Running, dispatcher_types::RouteAction::Default, 32).await.unwrap();
    let walk_route_link = link.new_route(MyKey::Waiting, dispatcher_types::RouteAction::Default, 32).await.unwrap();

    let client1 = tokio::spawn(run_client(walk_route_link, link.new_line(), MyKey::Waiting));
    // Для видимости пауза
    // Pause for visibility
    tokio::time::sleep(std::time::Duration::from_secs_f32(0.1)).await;
    let client2 = tokio::spawn(run_client(run_route_link, link.new_line(), MyKey::Running));


    let _ = tokio::join!(client1, client2);
}

pub async fn run_client(
    route_link: Option<dispatcher_types::DispatcherReply<MyStage, TrackRule<MyKey>>>,
    line: Sender<dispatcher_types::DispatcherCommand<MyStage, MyKey>>,
    key: MyKey
) -> Result<(), dispatcher_types::DispatcherError> {
    println!("Clien init: {:?}", key);

    // Выбираем стадию которую хотим 
    // We select the stage we want
    let stage = iff!(key == MyKey::Running ; MyStage::RunDisconnect ; MyStage::WaitDisconnect);

    if let Some(dispatcher_types::DispatcherReply::PurrStation { rx_reply }) = route_link {
        // Создаем новую станцию line это просто копия из линка
        // Create a new station; line is just a copy from link
        let mut station = station_link::PurrStation::new(line, rx_reply, key);
        // Отправляем команду о прикреплении вагона к основному поезду
        // Send command to attach carriage to main train
        let _ = station.send_command(dispatcher_types::DispatcherCommand::Attach { carriage: stage, key: station.get_key() }).await?;

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
                            (MyStage::RunDisconnect, TrackRule::RunTimer(timer), MyKey::Running) if timer.is_key(key) => break,
                            (MyStage::WaitDisconnect, TrackRule::WaitTimer(timer), MyKey::Waiting) if timer.is_key(key) => break,
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
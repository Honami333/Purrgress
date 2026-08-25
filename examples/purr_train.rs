use purrgress::cat_malloc::purr_train;
use purrgress::cat_malloc::train_design;
use purrgress::cat_malloc::train_route;
use purrgress::cat_malloc::train_siding;
use purrgress::cat_malloc::train_types;

use purrgress::cat_malloc::train_types::StandardRules;
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
    purr_design.chain(MyStage::IWRChain, train_types::StandardRules::instant(), vec![MyStage::Idle, MyStage::Walk, MyStage::Run]);
    purr_route.construct_schedule(&purr_design, BufferMode::Keep).unwrap();

    purr_siding.launch(MyStage::IWRChain, BufferMode::Clear, &purr_route).unwrap();
    purr_siding.change_rule(MyStage::Run, train_types::StandardRules::timer(2.0));

    println!("{purr_train:?}");

    loop {
        rule_update(&mut purr_train);

        let purr_event = purr_train.advance_train();

        if let train_types::PurrTrainEvent::Transition { .. } = purr_event { println!("{purr_event:?}"); };

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
        match &mut route_box.rule {
            StandardRules::Timer(timer) => timer.tick(delta),
            StandardRules::Flag(flag) => flag.set_flag(true),
            _ => {}
        };
    };
}

fn design_single(purr_design: &mut train_design::PurrDesign<MyStage, train_types::StandardRules>) {
    purr_design.single(MyStage::Idle, train_types::StandardRules::timer(2.0));
    purr_design.single(MyStage::Walk, train_types::StandardRules::timer(1.0));
    purr_design.single(MyStage::Run, train_types::StandardRules::timer(1.0));
}
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
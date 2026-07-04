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
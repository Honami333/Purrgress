use purrgress::cat_depot::train_track::PurrTrack;
use purrgress::cat_malloc::train;
use purrgress::cat_malloc::train_design;
use purrgress::cat_malloc::train_route;
use purrgress::cat_malloc::train_siding;
use purrgress::cat_depot::train_core;
use purrgress::cat_depot::train_rule::PurrRuleLite;
use purrgress::PurrStep;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
pub enum MyStage {
    Idle,
    Walk,
    Run,
    IWRChain
}


fn main() {
    let mut purr_train = train::PurrTrain::with_cursor();
    let mut purr_design = train_design::PurrDesign::new();
    let mut purr_route = train_route::PurrRoute::new();
    let mut purr_siding = train_siding::PurrSiding::new();

    design_single(&mut purr_design);
    purr_design.chain(MyStage::IWRChain, PurrRuleLite::instant(), vec![MyStage::Idle, MyStage::Walk, MyStage::Run]);
    purr_route.construct_schedule(&purr_design, train_core::SaveMode::Keep, train_core::DuplicatePolicy::Overwrite).unwrap();

    purr_siding.launch(MyStage::IWRChain, train_core::SaveMode::Clear, &purr_route).unwrap();
    purr_siding.change_rule(MyStage::Run, PurrRuleLite::timer(2.0), train_core::ChangeRuleCount::First);

    println!("{purr_train:?}");

    loop {
        rule_update(&mut purr_train);

        let purr_event = purr_train.advance_train();

        if let train_core::PurrTrainEvent::Transition { .. } = purr_event { println!("{purr_event:?}"); };

        if purr_event == train_core::PurrTrainEvent::Idle {
            purr_siding.launch(MyStage::IWRChain, train_core::SaveMode::Clear, &purr_route).unwrap();
            purr_train.attach(&mut purr_siding).unwrap();
            purr_train.clear_line();
        };

        purr_train.shrink_line(10000);
    };
}

fn rule_update<S>(purr_train: &mut train::PurrTrain<S>) where S: PurrTrack<Rule = PurrRuleLite> {
    let delta = 0.00000006;

    if let Some(route_box) = purr_train.get_current_mut() {
        match &mut route_box.rule {
            PurrRuleLite::Timer(timer) => timer.tick(delta),
            PurrRuleLite::Flag(flag) => flag.set(true),
            _ => {}
        };
    };
}

fn design_single(purr_design: &mut train_design::PurrDesign<MyStage, PurrRuleLite>) {
    purr_design.single(MyStage::Idle, PurrRuleLite::timer(2.0));
    purr_design.single(MyStage::Walk, PurrRuleLite::timer(1.0));
    purr_design.single(MyStage::Run, PurrRuleLite::timer(1.0));
}
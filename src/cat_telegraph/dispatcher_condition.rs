use purrgress_macros::PurrRule;

use purrgress::cat_telegraph::dispatcher_types::PurrKey;
use purrgress::cat_depot::train_conditions::{self, PurrCondition};

use wibr::MakeFull;


#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PurrRule)]
pub enum TrackRule<Y: PurrKey> {
    Instant(train_conditions::PurrInstant),
    Timer(train_conditions::PurrTimer),
    Flag(train_conditions::PurrFlag),
    RunTimer(RunTimer<Y>),
    WaitTimer(WaitTimer<Y>)
}

impl<Y: PurrKey> TrackRule<Y> {
    pub fn instant() -> Self { Self::Instant(train_conditions::PurrInstant) }
    pub fn timer(duration: f32) -> Self { Self::Timer(train_conditions::PurrTimer::new(duration)) }
    pub fn flag() -> Self { Self::Flag(train_conditions::PurrFlag::new()) }
    pub fn run(main_timer: f32, waiting_timer: f32, key: Y) -> Self { Self::RunTimer( RunTimer::new(key, main_timer, waiting_timer, ) ) }
    pub fn wait(main_timer: f32, max_time: f32, key: Y) -> Self { Self::WaitTimer( WaitTimer::new(max_time, key, main_timer)) }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, MakeFull)]
#[Extern(main_timer: f32)]
#[Extern( waiting_timer: f32)]
pub struct RunTimer<Y: PurrKey> {
    #[Functional({ train_conditions::PurrTimer::new(main_timer) })] pub main_timer: train_conditions::PurrTimer,
    #[Functional({ train_conditions::PurrTimer::new(waiting_timer) })] pub waiting_timer: train_conditions::PurrTimer,
    #[Some(train_conditions::PurrFlag::new())] pub fast_flag: train_conditions::PurrFlag,
    pub key: Y
}

impl<Y: PurrKey> RunTimer<Y> {
    pub fn tick(&mut self, delta: f32) {
        if !self.main_timer.is_finished() {
            self.main_timer.tick(delta);
            return;
        };
        self.waiting_timer.tick(delta);
    }

    pub fn is_key(&self, key: Y) -> bool { self.key == key }
    pub fn is_key_fast(&mut self, key: Y) { if self.is_key(key) { self.fast_flag.set(true);}; }
    pub fn overflow(&self) -> bool { self.waiting_timer.is_finished() }
}

impl<Y: PurrKey> PurrCondition for RunTimer<Y> {
    fn is_finished(&self) -> bool { self.waiting_timer.is_finished() || self.fast_flag.is_finished() }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, MakeFull)]
#[Extern(main_timer: f32)]
pub struct WaitTimer<Y: PurrKey> {
    #[Functional({ train_conditions::PurrTimer::new(main_timer) })] pub main_timer: train_conditions::PurrTimer,
    #[Some(train_conditions::PurrFlag::new())] pub finish_flag: train_conditions::PurrFlag,
    pub max_time: f32,
    pub key: Y
}

impl<Y> WaitTimer<Y>
where
    Y: PurrKey
{
    pub fn tick(&mut self, delta: f32) { self.main_timer.tick(delta) }

    pub fn set_flag(&mut self, key: Y, new_state: bool) {
        if self.main_timer.is_finished() && self.key == key { self.finish_flag.set(new_state); };
    }

    pub fn reverse_flag(&mut self, key: Y) {
        if self.main_timer.is_finished() && self.key == key { self.finish_flag.reverse(); };
    }

    pub fn is_key(&self, key: Y) -> bool { self.key == key }
    pub fn overflow(&self) -> bool { self.main_timer.time_elapsed >= self.max_time }
}

impl<Y> PurrCondition for WaitTimer<Y>
where
    Y: PurrKey
{
    fn is_finished(&self) -> bool { self.finish_flag.is_finished() || self.overflow() }
}
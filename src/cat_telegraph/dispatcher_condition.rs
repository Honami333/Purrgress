use purrgress::cat_telegraph::dispatcher_types::PurrKey;
use purrgress_macros::PurrRule;
use purrgress::condition::{self, PurrCondition};

use wibr::MakeFull;


#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PurrRule)]
pub enum TrackRule<Y: PurrKey> {
    Instant(condition::InstantCondition),
    Timer(condition::PurrTimer),
    Flag(condition::PurrFlag),
    RunTimer(RunTimer<Y>),
    WaitTimer(WaitTimer<Y>)
}

impl<Y: PurrKey> TrackRule<Y> {
    pub fn instant() -> Self { Self::Instant(condition::InstantCondition) }
    pub fn timer(duration: f32) -> Self { Self::Timer(condition::PurrTimer::new(duration)) }
    pub fn flag() -> Self { Self::Flag(condition::PurrFlag::new()) }
    pub fn run(main_timer: f32, waiting_timer: f32, key: Y) -> Self { Self::RunTimer( RunTimer::new(key, main_timer, waiting_timer, ) ) }
    pub fn wait(main_timer: f32, max_time: f32, key: Y) -> Self { Self::WaitTimer( WaitTimer::new(max_time, key, main_timer)) }
}

#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Default, MakeFull)]
#[Extern(main_timer: f32)]
#[Extern( waiting_timer: f32)]
pub struct RunTimer<Y: PurrKey> {
    #[Functional({ condition::PurrTimer::new(main_timer) })] pub main_timer: condition::PurrTimer,
    #[Functional({ condition::PurrTimer::new(waiting_timer) })] pub waiting_timer: condition::PurrTimer,
    #[Some(condition::PurrFlag::new())] pub fast_flag: condition::PurrFlag,
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
    pub fn is_key_fast(&mut self, key: Y) { if self.is_key(key) { self.fast_flag.set_flag(true);}; }
    pub fn overflow(&self) -> bool { self.waiting_timer.is_finished() }
}

impl<Y: PurrKey> PurrCondition for RunTimer<Y> {
    fn is_finished(&self) -> bool { self.waiting_timer.is_finished() || self.fast_flag.is_finished() }

    fn reset(&mut self) {
        self.main_timer.reset();
        self.waiting_timer.reset();
    }
}

#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Default, MakeFull)]
#[Extern(main_timer: f32)]
pub struct WaitTimer<Y: PurrKey> {
    #[Functional({ condition::PurrTimer::new(main_timer) })] pub main_timer: condition::PurrTimer,
    #[Some(condition::PurrFlag::new())] pub finish_flag: condition::PurrFlag,
    pub max_time: f32,
    pub key: Y
}

impl<Y> WaitTimer<Y>
where
    Y: PurrKey
{
    pub fn tick(&mut self, delta: f32) { self.main_timer.tick(delta) }

    pub fn set_flag(&mut self, key: Y, flag: bool) {
        if self.main_timer.is_finished() && self.key == key { self.finish_flag.set_flag(flag); };
    }

    pub fn reverse_flag(&mut self, key: Y) {
        if self.main_timer.is_finished() && self.key == key { self.finish_flag.reverse_flag(); };
    }

    pub fn is_key(&self, key: Y) -> bool { self.key == key }
    pub fn overflow(&self) -> bool { self.main_timer.get_time_left() >= self.max_time }
}

impl<Y> PurrCondition for WaitTimer<Y>
where
    Y: PurrKey
{
    fn is_finished(&self) -> bool { self.finish_flag.is_finished() || self.overflow() }

    fn reset(&mut self) {
        self.main_timer.reset();
        self.finish_flag.reset();
    }
}
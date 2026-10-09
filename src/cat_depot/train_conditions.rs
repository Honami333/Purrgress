use std::fmt::Debug;

use wibr::Make;


pub trait PurrCondition: Copy {
    fn is_finished(&self) -> bool;
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurrInstant;

impl PurrCondition for PurrInstant {
    fn is_finished(&self) -> bool { true }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Make)]
pub struct PurrTimer {
    pub duration: f32,
    #[Some(0.0)] pub time_elapsed: f32,
}

impl PurrTimer {
    pub fn tick(&mut self, delta: f32) { self.time_elapsed += delta; }
}

impl PurrCondition for PurrTimer {
    fn is_finished(&self) -> bool { self.time_elapsed >= self.duration }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Make, Default)]
pub struct PurrFlag {
    #[Some(false)] pub state: bool,
}

impl PurrFlag {
    pub fn from_flag(state: bool) -> Self { Self { state } }
    pub fn set(&mut self, new_state: bool) { self.state = new_state; }
    pub fn reverse(&mut self) { self.state = !self.state; }
}

impl PurrCondition for PurrFlag {
    fn is_finished(&self) -> bool { self.state }
}

#[cfg(test)]
mod condition_test {
    use super::*;

    // Verification of whether PurrInstant is finished on every call
    #[test]
    fn instant_is_always_finished() {
        let instant = PurrInstant;
        assert!(instant.is_finished());
        assert!(instant.is_finished());
    }

    // Verification of whether PurrTimer finishes when time_elapsed overshoots duration
    #[test]
    fn timer_finishes_after_time_elapsed_exceeds_duration() {
        let mut timer = PurrTimer::new(1.0);
        let delta = 0.4;

        assert!(!timer.is_finished());
        timer.tick(delta);
        assert!(!timer.is_finished());
        timer.tick(delta);
        assert!(!timer.is_finished());
        timer.tick(delta);
        assert!(timer.is_finished());
    }

    // Verification of whether PurrTimer finishes exactly at duration and stays finished after extra ticks
    #[test]
    fn timer_finishes_at_duration_and_stays_finished() {
        let mut timer = PurrTimer::new(1.0);
        let delta = 0.5;

        assert!(!timer.is_finished());
        timer.tick(delta);
        assert!(!timer.is_finished());
        timer.tick(delta);

        assert!(timer.is_finished());
        timer.tick(delta);
        assert!(timer.is_finished());
    }

    // Verification of whether PurrTimer with zero duration is finished without any tick
    #[test]
    fn zero_duration_timer_is_finished_immediately() {
        let timer = PurrTimer::new(0.0);
        assert!(timer.is_finished());
    }

    // Verification of whether PurrTimer with negative duration is finished without any tick
    #[test]
    fn negative_duration_timer_is_finished_immediately() {
        let timer = PurrTimer::new(-0.5);
        assert!(timer.is_finished());
    }

    // Documenting current behavior: negative delta decreases time_elapsed without a lower bound
    #[test]
    fn timer_with_negative_delta_stays_unfinished_and_goes_negative() {
        let mut timer = PurrTimer::new(1.0);
        let delta = -0.5;

        assert!(!timer.is_finished());
        timer.tick(delta);
        assert!(!timer.is_finished());
        timer.tick(delta);

        assert!(!timer.is_finished());
        assert!(timer.time_elapsed < 0.0);
    }

    // Documenting current behavior: NaN delta makes time_elapsed NaN irreversibly, so the timer never finishes
    #[test]
    fn nan_delta_poisons_elapsed_and_never_finishes() {
        let mut timer = PurrTimer::new(1.0);
        let delta = f32::NAN;

        assert!(!timer.time_elapsed.is_nan());
        assert!(!timer.is_finished());
        timer.tick(delta);

        assert!(timer.time_elapsed.is_nan());
        assert!(!timer.is_finished());
        timer.tick(delta);

        assert!(timer.time_elapsed.is_nan());
        assert!(!timer.is_finished());
    }

    // Documenting current behavior: NaN duration makes the comparison always false, so the timer never finishes
    #[test]
    fn nan_duration_timer_never_finishes() {
        let mut timer = PurrTimer::new(f32::NAN);
        let delta = 0.5;

        assert!(timer.duration.is_nan());
        assert!(!timer.is_finished());
        timer.tick(delta);

        assert!(timer.duration.is_nan());
        assert!(!timer.is_finished());
        timer.tick(delta);

        assert!(timer.duration.is_nan());
        assert!(!timer.is_finished());
    }

    //INFINITY duration is never reached by finite deltas, so the timer never finishes
    #[test]
    fn infinity_duration_timer_never_finishes() {
        let mut timer = PurrTimer::new(f32::INFINITY);
        let delta = 0.5;

        assert_eq!(timer.duration, f32::INFINITY);
        assert!(!timer.is_finished());
        timer.tick(delta);

        assert_eq!(timer.duration, f32::INFINITY);
        assert!(!timer.is_finished());
        timer.tick(delta);

        assert_eq!(timer.duration, f32::INFINITY);
        assert!(!timer.is_finished());
    }

    // Documenting current behavior: f32 accumulation error leaves the timer unfinished after 100 ticks of 0.01
    #[test]
    fn float_accumulation_error_keeps_timer_unfinished() {
        let mut timer = PurrTimer::new(1.0);
        let delta = 0.01; // 0.01 * 100.0 = 1.0

        for _ in 0..100 { timer.tick(delta); }

        // timer.time_elapsed < 1.0
        assert!(!timer.is_finished());
    }

    // Verification of whether PurrFlag::set overrides the previous value and is idempotent
    #[test]
    fn flag_set_overrides_previous_value() {
        let mut flag = PurrFlag::new();

        assert!(!flag.is_finished());
        flag.set(true);
        assert!(flag.is_finished());
        flag.set(true);
        assert!(flag.is_finished());
        flag.set(false);
        assert!(!flag.is_finished());
        flag.set(false);
        assert!(!flag.is_finished());
    }

    // Verification of whether PurrFlag::reverse inverts the value in both directions
    #[test]
    fn flag_reverse_inverts_current_value() {
        let mut flag = PurrFlag::new();

        assert!(!flag.is_finished());
        flag.reverse();
        assert!(flag.is_finished());
        flag.reverse();
        assert!(!flag.is_finished());
    }

    // Verification of whether new, from_flag(false) and default produce the same PurrFlag
    #[test]
    fn new_flag_equals_false_flag_and_default() {
        assert_eq!(PurrFlag::new(), PurrFlag::from_flag(false));
        assert_eq!(PurrFlag::new(), PurrFlag::default());
    }

    // Verification of whether PurrFlag created with true is finished without any call
    #[test]
    fn flag_created_with_true_is_finished_immediately() {
        let true_flag = PurrFlag::from_flag(true);
        assert!(true_flag.is_finished());
    }
}
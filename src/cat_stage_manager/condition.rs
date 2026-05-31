pub trait PurrCondition  {
    fn is_finished(&mut self) -> bool;
    fn reset(&mut self);

    fn as_any(&self) -> &(dyn std::any::Any + 'static);
    fn as_any_mut(&mut self) -> &mut (dyn std::any::Any + 'static);
}

pub struct InstantCondition;

impl PurrCondition for InstantCondition {
    fn is_finished(&mut self) -> bool { true }

    fn reset(&mut self) {}

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

pub struct PurrTimer {
    pub duration: f32,
    pub time_left: f32,
}

impl PurrTimer {
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            time_left: 0.0
        }
    }

    pub fn tick(&mut self, delta: f32) {
        self.time_left += delta;
    }
}

impl PurrCondition for PurrTimer {
    fn is_finished(&mut self) -> bool {
        if self.time_left >= self.duration {
            self.reset();
            true
        } else {
            false
        }
    }

    fn reset(&mut self) {
        self.time_left = 0.0;
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

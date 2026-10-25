use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};

use wibr::{New, Make, MakeFull};

use super::train_conditions::PurrCondition;

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Make, Default)]
pub struct PurrProximity {
    pub current: PurrVec,
    pub target: PurrVec,
    #[Some(PurrDirection::from_purr_vec(current, target))] pub direction: PurrDirection
}

impl PurrProximity {
    // Adding another vector to the current position vector

    #[inline] pub fn forward(&mut self, add: PurrVec) { self.current += add; } 

    // Subtracting another vector from the current position vector
    #[inline] pub fn backward(&mut self, sub: PurrVec) { self.current -= sub; }

    // Multiplying the current position vector by another vector
    #[inline] pub fn expand(&mut self, mul: PurrVec) { self.current *= mul; }

    // Dividing the current position vector by another vector
    #[inline] pub fn shrink(&mut self, div: PurrVec) { if div.x != 0.0 && div.y != 0.0 { self.current /= div; } }

    #[inline] pub fn update_direction(&mut self) { self.direction.update(self.current, self.target); }

    // Distance from the current position to the target position
    #[inline] pub fn get_distance(&self) -> f32 { self.current.distance(self.target) }

    #[inline] pub fn get_distance_not_sqrt(&self) -> f32 { self.current.distance_squared(self.target) }
}

impl PurrCondition for PurrProximity {
    #[inline]
    fn is_finished(&self) -> bool {
        let tx = self.target.x;
        let ty = self.target.y;
        let px = self.current.x;
        let py = self.current.y;

        match (self.direction.x, self.direction.y) {
            (true, true) => px >= tx && py >= ty,
            (true, false) => px >= tx && py < ty,
            (false, true) => px < tx && py >= ty,
            (false, false) => px < tx && py < ty,
        }
    }
}

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, New, MakeFull, Default)]
#[Fn(from_purr_vec)]
#[Extern(current: PurrVec)]
#[Extern(target: PurrVec)]
pub struct PurrDirection {
    #[Some(target.x >= current.x)] pub x: bool,
    #[Some(target.y >= current.y)] pub y: bool
}

impl PurrDirection {
    #[inline] pub fn update(&mut self, current: PurrVec, target: PurrVec) { *self = Self::from_purr_vec(current, target); }
}

/// #PurrVec
/// // Vector2 with values ​​in the form of x and y coordinates
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, New, Default)]
pub struct PurrVec {
    pub x: f32,
    pub y: f32,
}

impl Add for PurrVec {
    type Output = Self;
 
    #[inline] fn add(mut self, rhs: Self) -> Self::Output { self += rhs; self }
}
 
impl Sub for PurrVec {
    type Output = Self;
 
    #[inline] fn sub(mut self, rhs: Self) -> Self::Output { self -= rhs; self }
}
 
impl Mul for PurrVec {
    type Output = Self;
 
    #[inline] fn mul(mut self, rhs: Self) -> Self::Output { self *= rhs; self }
}
 
impl Div for PurrVec {
    type Output = Self;
 
    #[inline] fn div(mut self, rhs: Self) -> Self::Output { self /= rhs; self }
}
 
impl Mul<f32> for PurrVec {
    type Output = Self;
 
    #[inline] fn mul(mut self, rhs: f32) -> Self::Output { self *= rhs; self }
}
 
impl Div<f32> for PurrVec {
    type Output = Self;
 
    #[inline] fn div(mut self, rhs: f32) -> Self::Output { self /= rhs; self }
}
 
impl AddAssign for PurrVec {
    #[inline] fn add_assign(&mut self, rhs: Self) { self.x += rhs.x; self.y += rhs.y; }
}
 
impl SubAssign for PurrVec {
    #[inline] fn sub_assign(&mut self, rhs: Self) { self.x -= rhs.x; self.y -= rhs.y; }
}
 
impl MulAssign for PurrVec {
    #[inline] fn mul_assign(&mut self, rhs: Self) { self.x *= rhs.x; self.y *= rhs.y; }
}
 
impl DivAssign for PurrVec {
    #[inline] fn div_assign(&mut self, rhs: Self) { self.x /= rhs.x; self.y /= rhs.y; }
}
 
impl MulAssign<f32> for PurrVec {
    #[inline] fn mul_assign(&mut self, rhs: f32) { self.x *= rhs; self.y *= rhs; }
}
 
impl DivAssign<f32> for PurrVec {
    #[inline] fn div_assign(&mut self, rhs: f32) { self.x /= rhs; self.y /= rhs; }
}

impl PurrVec {
    #[inline] pub fn length_squared(&self) -> f32 { self.x * self.x + self.y * self.y }

    #[inline] pub fn length(&self) -> f32 { self.length_squared().sqrt() }

    #[inline] pub fn distance_squared(&self, other: Self) -> f32 { (self.x - other.x).powi(2) + (self.y - other.y).powi(2) }

    #[inline] pub fn distance(&self, other: Self) -> f32 { self.distance_squared(other).sqrt() }

    #[inline]
    pub fn normalize(&mut self) {
        let len = self.length();
        match len > 0.0 {
            true => *self /= len,
            false => *self = Self::default()
        };
    }
}

impl PurrVec {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
    pub const LEFT: Self = Self { x: -1.0, y: 0.0 };
    pub const RIGHT: Self = Self { x: 1.0, y: 0.0 };
    pub const UP: Self = Self { x: 0.0, y: 1.0 };
    pub const DOWN: Self = Self { x: 0.0, y: -1.0 };
}
//! Numeric helpers
use std::ops::Mul;
use moonlander_gp::Number;
use std::f32::consts::PI;

pub const TAU: Number = 2.0 * PI;

pub fn square<T: Mul+Copy>(x: T) -> T::Output {
    x * x
}

pub fn angle_dist(o: Number) -> Number {
    let r = o % TAU;
    if r > PI { TAU - r } else { r }
}

pub fn abs(x: Number) -> Number {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

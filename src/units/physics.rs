use std::ops::{Add, Deref, Div, Mul, Neg, Sub};

use super::drawing::Game;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Millis(pub i64);

impl Add<Millis> for Millis {
    type Output = Millis;

    #[inline]
    fn add(self, rhs: Millis) -> Self::Output {
        let (Millis(a), Millis(b)) = (self, rhs);
        Millis(a + b)
    }
}

impl Sub<Millis> for Millis {
    type Output = Millis;

    #[inline]
    fn sub(self, rhs: Millis) -> Self::Output {
        let (Millis(a), Millis(b)) = (self, rhs);
        Millis(a - b)
    }
}

pub struct Velocity(pub f64);

impl Neg for Velocity {
    type Output = Velocity;

    #[inline]
    fn neg(self) -> Self::Output {
        let Velocity(v0) = self;
        Velocity(-v0)
    }
}

impl Add<Velocity> for Velocity {
    type Output = Velocity;

    #[inline]
    fn add(self, rhs: Velocity) -> Self::Output {
        let (Velocity(v0), Velocity(v1)) = (self, rhs);
        Velocity(v0 + v1)
    }
}

impl Sub<Velocity> for Velocity {
    type Output = Velocity;

    #[inline]
    fn sub(self, rhs: Velocity) -> Self::Output {
        let (Velocity(v0), Velocity(v1)) = (self, rhs);
        Velocity(v0 - v1)
    }
}

impl Mul<Millis> for Velocity {
    type Output = Game;

    #[inline]
    fn mul(self, rhs: Millis) -> Self::Output {
        let (Velocity(v0), Millis(t)) = (self, rhs);
        Game(v0 * t as f64)
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Acceleration(pub f64);

impl Mul<Millis> for Acceleration {
    type Output = Velocity;

    #[inline]
    fn mul(self, rhs: Millis) -> Self::Output {
        let (Acceleration(a), Millis(t)) = (self, rhs);
        Velocity(a * t as f64)
    }
}

impl Neg for Acceleration {
    type Output = Acceleration;

    #[inline]
    fn neg(self) -> Self::Output {
        let Acceleration(a) = self;
        Acceleration(-a)
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Degrees(pub f64);

impl Deref for Degrees {
    type Target = f64;

    #[inline]
    fn deref<'a>(&'a self) -> &'a Self::Target {
        let Degrees(ref inner) = *self;
        inner
    }
}

impl Add<Degrees> for Degrees {
    type Output = Degrees;

    #[inline]
    fn add(self, rhs: Degrees) -> Self::Output {
        let (Degrees(d0), Degrees(d1)) = (self, rhs);
        Degrees(d0 + d1)
    }
}

impl Div<Millis> for Degrees {
    type Output = AngularVelocity;

    #[inline]
    fn div(self, rhs: Millis) -> Self::Output {
        let (Degrees(d), Millis(t)) = (self, rhs);
        AngularVelocity(d / t as f64)
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct AngularVelocity(pub f64);

impl Mul<Millis> for AngularVelocity {
    type Output = Degrees;

    fn mul(self, rhs: Millis) -> Self::Output {
        let (AngularVelocity(av), Millis(t)) = (self, rhs);
        Degrees(av * t as f64)
    }
}

pub type Frame = u64;
pub type Fps = u64;

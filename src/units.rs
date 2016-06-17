//! Data types and utility function for units:
//! Implemet a poor man's unit type system for now...
//!
//! Maybe we can have s.th. like this (F#) for Rust: https://en.wikibooks.org/wiki/F_Sharp_Programming/Units_of_Measure
//! Alternatively, see here:
//! https://www.reddit.com/r/rust/comments/37qut9/typesafe_userdefined_units_of_measure_for_rust/
//! https://blog.mozilla.org/research/2014/06/23/static-checking-of-units-in-servo/

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Meter(pub f64);

impl PartialEq for Meter {
    fn eq(self: &Meter, &Meter(rhs): &Meter) -> bool {
        let Meter(lhs) = *self;
        lhs == rhs
    }
}

impl Add for Meter {
    type Output = Meter;

    fn add(self: Meter, Meter(rhs): Meter) -> Meter {
        let Meter(lhs) = self;
        Meter(lhs + rhs)
    }
}

impl Sub for Meter {
    type Output = Meter;

    fn sub(self: Meter, Meter(rhs): Meter) -> Meter {
        let Meter(lhs) = self;
        Meter(lhs - rhs)
    }
}

impl Mul<Meter> for Meter {
    type Output = Meter2;

    fn mul(self: Meter, Meter(rhs): Meter) -> Meter2 {
        let Meter(lhs) = self;
        Meter2(lhs * rhs)
    }
}

impl Mul<f64> for Meter {
    type Output = Meter;

    fn mul(self: Meter, rhs: f64) -> Meter {
        let Meter(lhs) = self;
        Meter(lhs * rhs)
    }
}

impl Div<Meter> for Meter {
    type Output = f64;

    fn div(self: Meter, Meter(rhs): Meter) -> f64 {
        let Meter(lhs) = self;
        lhs / rhs
    }
}

impl Div<Second> for Meter {
    type Output = MeterPerSecond;

    fn div(self: Meter, Second(rhs): Second) -> MeterPerSecond {
        let Meter(lhs) = self;
        MeterPerSecond(lhs / rhs)
    }
}


#[derive(Debug)]
pub struct Meter2(pub f64);

impl PartialEq for Meter2 {
    fn eq(self: &Meter2, &Meter2(rhs): &Meter2) -> bool {
        let Meter2(lhs) = *self;
        lhs == rhs
    }
}


#[derive(Debug)]
pub struct Second(pub f64);

impl Add for Second {
    type Output = Second;

    fn add(self: Second, Second(rhs): Second) -> Second {
        let Second(lhs) = self;
        Second(lhs + rhs)
    }
}

impl Sub for Second {
    type Output = Second;

    fn sub(self: Second, Second(rhs): Second) -> Second {
        let Second(lhs) = self;
        Second(lhs - rhs)
    }
}

impl Mul<Second> for Second {
    type Output = Second2;

    fn mul(self: Second, Second(rhs): Second) -> Second2 {
        let Second(lhs) = self;
        Second2(lhs * rhs)
    }
}

impl Mul<f64> for Second {
    type Output = Second;

    fn mul(self: Second, rhs: f64) -> Second {
        let Second(lhs) = self;
        Second(lhs * rhs)
    }
}

impl Div<Second> for Second {
    type Output = f64;

    fn div(self: Second, Second(rhs): Second) -> f64 {
        let Second(lhs) = self;
        lhs / rhs
    }
}

#[derive(Debug)]
pub struct Second2(pub f64);

#[derive(Debug)]
pub struct MeterPerSecond(pub f64);

impl PartialEq for MeterPerSecond {
    fn eq(self: &MeterPerSecond, &MeterPerSecond(rhs): &MeterPerSecond) -> bool {
        let MeterPerSecond(lhs) = *self;
        lhs == rhs
    }
}

impl Add for MeterPerSecond {
    type Output = MeterPerSecond;

    fn add(self: MeterPerSecond, MeterPerSecond(rhs): MeterPerSecond) -> MeterPerSecond {
        let MeterPerSecond(lhs) = self;
        MeterPerSecond(lhs + rhs)
    }
}

impl Sub for MeterPerSecond {
    type Output = MeterPerSecond;

    fn sub(self: MeterPerSecond, MeterPerSecond(rhs): MeterPerSecond) -> MeterPerSecond {
        let MeterPerSecond(lhs) = self;
        MeterPerSecond(lhs - rhs)
    }
}

impl Mul<Second> for MeterPerSecond {
    type Output = Meter;

    fn mul(self: MeterPerSecond, Second(rhs): Second) -> Meter {
        let MeterPerSecond(lhs) = self;
        Meter(lhs * rhs)
    }
}

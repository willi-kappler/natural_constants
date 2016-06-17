extern crate natural_constants;

use natural_constants::units::*;

#[test]
fn test_meter_add() {
    assert_eq!(Meter(2.3) + Meter(7.5), Meter(9.8));
}

#[test]
fn test_meter_sub() {
    assert_eq!(Meter(8.1) - Meter(5.6), Meter(2.5));
}

#[test]
fn test_meter_mul_meter() {
    assert_eq!(Meter(2.5) * Meter(4.0), Meter2(10.0));
}

#[test]
fn test_meter_div_meter() {
    assert_eq!(Meter(10.0) / Meter(5.0), 2.0);
}

#[test]
fn test_meter_div_second() {
    assert_eq!(Meter(15.0) / Second(3.0), MeterPerSecond(5.0));
}

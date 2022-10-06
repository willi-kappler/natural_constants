//! Data types and utility function for conversion

use crate::chemistry::absolute_zero;

const scale_factor: f64 = 1000.0;

pub fn to_kilo(value: f64) -> f64 {
    value / scale_factor
}

pub fn from_kilo(value: f64) -> f64 {
    value * scale_factor
}

pub fn to_mega(value: f64) -> f64 {
    value / (scale_factor * scale_factor)
}

pub fn from_mega(value: f64) -> f64 {
    value * (scale_factor * scale_factor)
}

pub fn to_giga(value: f64) -> f64 {
    value / (scale_factor * scale_factor * scale_factor)
}

pub fn from_giga(value: f64) -> f64 {
    value * (scale_factor * scale_factor * scale_factor)
}

pub fn to_tera(value: f64) -> f64 {
    value / (scale_factor * scale_factor * scale_factor * scale_factor)
}

pub fn from_tera(value: f64) -> f64 {
    value * (scale_factor * scale_factor * scale_factor * scale_factor)
}

pub fn kilo_to_mega(value: f64) -> f64 {
    value / scale_factor
}

pub fn mega_to_kilo(value: f64) -> f64 {
    value * scale_factor
}

pub fn to_mili(value: f64) -> f64 {
    value * scale_factor
}

pub fn from_mili(value: f64) -> f64 {
    value / scale_factor
}

pub fn to_micro(value: f64) -> f64 {
    value * (scale_factor * scale_factor)
}

pub fn from_micro(value: f64) -> f64 {
    value / (scale_factor * scale_factor)
}

pub fn kelvin_to_deg_cel(value: f64) -> f64 {
    value + absolute_zero
}

pub fn deg_cel_to_kelvin(value: f64) -> f64 {
    value - absolute_zero
}

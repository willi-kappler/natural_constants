//! Data types and utility function for conversion



pub fn to_kilo(value: f64) -> f64 {
    value / 1000.0
}

pub fn from_kilo(value: f64) -> f64 {
    value * 1000.0
}

pub fn to_mega(value: f64) -> f64 {
    value / (1000.0 * 1000.0)
}

pub fn from_mega(value: f64) -> f64 {
    value * (1000.0 * 1000.0)
}

pub fn to_giga(value: f64) -> f64 {
    value / (1000.0 * 1000.0 * 1000.0)
}

pub fn from_giga(value: f64) -> f64 {
    value * (1000.0 * 1000.0 * 1000.0)
}

pub fn to_terra(value: f64) -> f64 {
    value / (1000.0 * 1000.0 * 1000.0 * 1000.0)
}

pub fn from_terra(value: f64) -> f64 {
    value * (1000.0 * 1000.0 * 1000.0 * 1000.0)
}

pub fn kilo_to_mega(value: f64) -> f64 {
    value / 1000.0
}

pub fn mega_to_kilo(value: f64) -> f64 {
    value * 1000.0
}

//! natural_constants: a collection of constants and helper functions
//!
//! Written by Willi Kappler, Version 0.1 (2017.02.20)
//!
//! Repository: https://github.com/willi-kappler/natural_constants
//!
//! License: MIT
//!
//!
//! # Example:
//!
//! ```
//! use natural_constants::physics::*;
//!
//!
//! fn main() {
//!     let c = speed_of_light_vac;
//!     let m0 = 100.0;
//!
//!     // Use c in your code:
//!     let E = m0 * c * c;
//! }
//! ```


// For clippy
// #![feature(plugin)]
//
// #![plugin(clippy)]

#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub mod math;
pub mod physics;
pub mod chemistry;
pub mod biology;
pub mod engineering;
pub mod conversion;
pub mod misc;
pub mod geosciences;

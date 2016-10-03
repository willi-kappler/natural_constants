# natural_constants

[![Build Status](https://travis-ci.org/willi-kappler/natural_constants.svg?branch=master)](https://travis-ci.org/willi-kappler/natural_constants)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Pre-defined constants from all disciplines (math, physics, ...) as a Rust library.

## Currently the following modules exist:

- math
- physics
- chemistry
- biology
- engineering
- conversion

It's far from complete. So if your favorite constant is missing, just let me know.

What should go in ? Everything that you think is useful. Some constants may fit into multiple modules - we then have to decide which one.

- Why are you using long names for the constants ? Why not a simple character like 'c' ?

Well simple characters are often used ("overloaded") by various constants, so to avoid ambiguity the full name is used.

- But isn't that too complicated to type ? Now my code looks too ugly!

Well no one stops you from doing s.th. like this:

```rust
extern crate natural_constants;
use natural_constants::physics::*;

fn main() {
    let c = speed_of_light_vac;
	let m0 = 100.0;

    // Use c in your code:
    let E = m0 * c * c;
}
```

This is an ongoing effort to make Rust more suitable for scientific / numeric computing, you can join the discussion [here](https://users.rust-lang.org/t/numerics-math-foundation/7247).

## Other usefull numeric / scientific crates:

- [rand](https://github.com/rust-lang-nursery/rand)
- [num](https://github.com/rust-num/num)
- [fast_inv_sqrt](https://github.com/emkw/rust-fast_inv_sqrt)
- [vecmath](https://github.com/pistondevelopers/vecmath)
- [cgmath-rs](https://github.com/bjz/cgmath)
- [matrixmultiply](https://github.com/bluss/matrixmultiply/)
- [nalgebra](https://github.com/sebcrozet/nalgebra)
- [scirust](https://github.com/indigits/scirust)
- [beagle-rs](https://github.com/Popog/beagle-rs)
- [ndarray](https://github.com/bluss/rust-ndarray)
- [statrs](https://github.com/boxtown/statrs)
- [astro-rust](https://github.com/saurvs/astro-rust)
- [rust-bio](https://github.com/rust-bio/rust-bio)
- [lapack](https://github.com/stainless-steel/lapack)
- [rust-blas](https://github.com/mikkyang/rust-blas)
- [rust-gmp](https://github.com/thestinger/rust-gmp)
- [rust-gsl](https://github.com/GuillaumeGomez/rust-GSL)
- [georust](https://github.com/georust)
- [imageproc](https://github.com/chyh1990/imageproc)
- [units](https://github.com/Boddlnagg/units)
- [simple_units](https://github.com/willi-kappler/simple_units)

### More here:
- [awsome-rust](https://github.com/kud1ing/awesome-rust)

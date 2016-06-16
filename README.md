# natural_constants

[![Build Status](https://travis-ci.org/willi-kappler/natural_constants.svg?branch=master)](https://travis-ci.org/willi-kappler/natural_constants)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Pre-defined constants from all disciplines (math, physics, ...) as a Rust library.

Currently the following modules exist:

- math
- physics
- chemistry
- biology
- engineering

It's far from complete. So if your favorite constant is missing, just let me know.

What should go in ? Everything that you think is useful. Some constants may fit into multiple modules - we then have to decide which one.

- Why are you using long names for the constants ? Why not a simple character like 'c' ?

Well simple characters are often used ("overloaded") by various constants, so to avoid ambiguity the full name is used.

- But isn't that too complicated to type ? Now my code looks too ugly!

Well no one stops you from doing s.th. like this:

```rust
let c = speed_of_light_vac;

// Use c in your code:
let E = m0 * c * c;
```

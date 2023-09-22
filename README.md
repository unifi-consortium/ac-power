![continuous integration](https://github.com/unifi-consortium/ac-power/actions/workflows/rust.yml/badge.svg)

# ac-power
Rust crate for implementing ac power calculations such as park, clark, and fortesque transformations.

# Reference frames
The crate has 4 data types to represent 4 different reference frames for AC power systems:

1.  Abc - Raw three-phase signals
2.  Polar - Representing a balanced system specified with an amplitude and a phase.
3.  AlphaBeta - Stationary reference frame
4.  Dq0 - Rotating reference frame

All data types are based on integer data types using the [fixed](https://crates.io/crates/fixed) crate.  This was done for a few reasons.

1.  Speed.  Calculations are fast even on microcontrollers that do not have a dedicated FPU.
2.  Consistency.  Calculations will yield the exact same results regardless of platform.
3.  Logical full scale.  Trig functions like sin/cos benefit from data types which are naturally constrainted to +-1
4.  Natural wrap around (modulo) at the integer overflow: critical for phase/frequency applications used in ac power analysus.

Use of the [fixed](https://crates.io/crates/fixed) crate provides zero-cost abstractions which removes a lot of the pain points typically associated with using fixed-point arithmetic such as keeping track of the fractional bits and converting to/from floating point.

# Example Usage

```rust
// Create a balanced three-phase system in polar reference frame
let theta = I1F31::from_num(20. / 360.);
let amplitude = I11F21::from_num(480.0);
let polar = Polar { theta, amplitude };

// Convert to abc from polar
let abc = Abc::from(polar);

// convert to alpha/beta from abc
let alpha_beta = AlphaBeta::from(abc);

// convert to Dq0 from abc
let (sin, cos) = sin_cos(theta);
let dq0 = abc.to_dq0(sin, cos);
```

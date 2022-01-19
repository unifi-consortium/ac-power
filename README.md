![continuous integration](https://github.com/unifi-consortium/ac-power/actions/workflows/rust.yml/badge.svg)

# ac-power
Rust crate for implementing ac power calculations such as park, clark, and fortesque transformations.

# Reference frames
The crate has 4 data types to represent 4 different reference frames for AC power systems:

1.  Abc - Raw three-phase signals
2.  Polar - Representing a balanced system specified with an amplitude and a phase.
3.  AlphaBeta - Stationary reference frame
4.  Dq0 - Rotating reference frame

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

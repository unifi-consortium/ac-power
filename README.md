![continuous integration](https://github.com/unifi-consortium/ac-power/actions/workflows/rust.yml/badge.svg)

# ac-power
Rust crate for [ac-power](https://en.wikipedia.org/wiki/AC_power) analysis on embedded systems (i.e. microcontrollers).

# Fixed-Point
The core arithmetic is all [fixed-point](https://en.wikipedia.org/wiki/Fixed-point_arithmetic) using the [fixed](https://crates.io/crates/fixed) crate.  This was done for a few reasons:

1.  ADC readings are naturally evenly spaced, which has a more efficient translation to a fixed-point format.
2.  Speed.  Calculations are fast even on microcontrollers that do not have a dedicated FPU.
3.  Consistency.  Calculations will yield the exact same results regardless of platform.
4.  Logical full scale.  Trig functions like sin/cos benefit from data types which are naturally constrainted to +-1
5.  Natural wrap around (modulo) at the integer overflow: critical for phase/frequency applications used in ac power analysis.

Use of the [fixed](https://crates.io/crates/fixed) crate provides zero-cost abstractions which removes a lot of the pain points typically associated with using fixed-point arithmetic such as keeping track of the fractional bits and converting to/from floating point.

# Reference frames
The crate has 6 data types to represent the different reference frames for AC power systems:

Balanced reference frames:
1.  Polar - Representing a balanced system specified with an amplitude and a phase.
2.  AlphaBeta - Stationary referene frame
3.  Dq - Rotating reference frame

Unbalanced reference frames:
1.  Abc - Raw three-phase signals
2.  AlphaBeta0 - Stationary referene frame
3.  Dq0 - Rotating reference frame


# Plls
The crate includes single phase and three phase Phase-Locked-Loop (PLL) traits, as well as a reference implementation for a [Double Synchronous Reference Frame PLL](https://www.researchgate.net/publication/224626683_Double_Synchronous_Reference_Frame_PLL_for_Power_Converters_Control).

# Example Usage

```rust
use fixed::FixedI32;
type Fix = FixedI32<20>;  // 11 integer and 20 fractional bits

// create a rotating reference frame representation of an unbalanced waveform
let dq_pos = Dq {
    d: Fix::from_num(325.0),
    q: Fix::ZERO,
};
let dq_neg = Dq {
    d: Fix::from_num(100.0),
    q: Fix::from_num(-35.0),
};

// convert to abc reference frame
let (sin, cos) = sin_cos(theta);
let abc_pos = dq_pos.to_abc(sin, cos);
let abc_neg = dq_neg.to_abc(-sin, cos); // rotates in opposite direction

// a zero sequence is just a floating point numnber
let zero = Fix::from_num(20.0);

// arithmetic operations supported
let abc = abc_pos + abc_neg + zero;

```

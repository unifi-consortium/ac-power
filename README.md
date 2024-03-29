![continuous integration](https://github.com/unifi-consortium/ac-power/actions/workflows/rust.yml/badge.svg)

# ac-power
Reference frames, transforms, and trig tools for digital signal processing of AC power signals.

`ac-power` is a crate for creating and manipulating [ac power](https://en.wikipedia.org/wiki/AC_power) signals in commonly used reference frames for conducting ac power analysis and signal processing.  The crate is designed for `#![no_std]` so can be used in microcontrollers for devices such as inverters and power meters which need to process ac power signals.  While the library is floating-point, it uses the heavily optimized fixed-point trig from [idsp](https://crates.io/crates/idsp) for efficient execution in constrained environments such as microcontrollers.

# How to use

A very simple use case for this crate would be to calculate real and reactive powers from three-phase voltage and current data

```rust

use ac_power::reference_frames::{Abc, Dq0, AlphaBeta0, Polar};
use ac_power::trig::{Theta, cos_sin};
use approx::assert_abs_diff_eq;

// create a voltage and a current that is lagging 45 degrees
let v_polar = Polar{amplitude: 340.0, theta: Theta::from_degrees(0.0)};
let i_polar = Polar{amplitude: 8.2, theta: Theta::from_degrees(45.0)};

// convert to Abc reference
let v = Abc::from(v_polar);
let i = Abc::from(i_polar);

// calculate P and Q
let pq = v * i;

// calculate the power factor
let pf = pq.power_factor();

// check the power factor
assert_abs_diff_eq!(f32::from(pf), 0.707, epsilon = 0.0001);
```

Many inverter control systems that implement advanced grid controls or grid forming controls also rely on the transforms implemented in this crate.  Use of this crate can not only make the application code much more readible, it can improve performance and elinate bugs due to the extensive optimization and verification of this crate.

Bellow is a example of a simple three-phase Phased Locked Loop implementation, a common DSP block in inverter controls and advanced power meters, to illustrate how the crate can be used to facillitate such applications.

```rust
use ac_power::reference_frames::{Abc, AlphaBeta, Dq};
use ac_power::trig::{chebyshev, cos_sin, Cos, Sin, Theta};
use idsp::iir::{Action, Biquad, Pid};

pub struct Pll {
    fref: f32,

    // loop filter
    filter: Biquad<f32>,
    filter_state: [f32; 2], // uses direct form 2 transposed, which is optimized for floating point

    // frequency/angle
    pub theta: Theta,
    pub sin: Sin,
    pub cos: Cos,
    pub f: f32,

    // rotating reference frames
    pub dq_pos: Dq,
    pub dq_neg: Dq,

    // theta integration contant
    k_theta: f32,
}

impl Pll {
    pub fn new(fref: f32, kp: f32, ki: f32, max_integral: f32, ts: f32) -> Self {
        // calculate the theta integration constant
        let k_theta = ts * (u32::MAX as f32);

        // create the Pi frequency lock filter
        let mut filter: Biquad<f32> = Pid::default()
            .period(ts)
            .gain(Action::Kp, kp)
            .gain(Action::Ki, ki)
            .build()
            .unwrap()
            .into();
        filter.set_max(max_integral);
        filter.set_min(-max_integral);

        Self {
            fref,
            filter,
            filter_state: [0.0, 0.0],
            theta: 0.into(),
            sin: 0.0.into(),
            cos: 1.0.into(),
            f: fref,
            dq_pos: Dq::ZERO,
            dq_neg: Dq::ZERO,
            k_theta,
        }
    }

    pub fn update(&mut self, abc: Abc) {
        // clarke transform
        let alpha_beta = AlphaBeta::from(abc);

        // park transforms
        self.dq_pos = alpha_beta.to_dq(self.cos, self.sin);
        self.dq_neg = alpha_beta.to_dq(self.cos, -self.sin);

        // PI loop filter
        self.f = self.fref + self.filter.update(&mut self.filter_state, self.dq_pos.q);

        // update the phase info
        self.theta += (self.f * self.k_theta) as i32;
        (self.cos, self.sin) = cos_sin(self.theta);
    }
}
```

# Reference frames

The crate supports 5 difference reference frames.  These include 3 balanced reference frames:

1.  [Polar](crate::reference_frames::Polar) - Polar representation (aplitude and angle)
2.  [AlphaBeta](crate::reference_frames::AlphaBeta) - Two axis (alpha and beta) stationary reference frame representation
3.  [Dq](crate::reference_frames::Dq) - Two axis (d and q) rotating reference frame representation

And three reference frames for unbalanced representations (supports a zero sequence component):

1.  [Abc](crate::reference_frames::Abc) - Instantaneous signals
2.  [AlphaBeta0](crate::reference_frames::AlphaBeta0) - Two axis (alpha and beta) stationary reference frame representation with zero
3.  [Dq0](crate::reference_frames::Dq0) - Two axis (d and q) rotating reference frame representation with zero

# Transforms

The crate supports for following lossless conversions (zero sequence retained):


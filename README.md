![continuous integration](https://github.com/unifi-consortium/ac-power/actions/workflows/rust.yml/badge.svg)

# `ac-power`

Reference frames, transforms, and trig for embedded processing of AC power signals.

# How to use

At the core of the library are data structs which represent three-phase AC phasors in different reference frames.  The struct have transforms to support conversions between different reference frames.

```rust
use ac_power::reference_frames::{Abc, AlphaBeta0};

// create a phasor in Abc reference frame
let abc = Abc {a: 100.0, b: 200.0, c: 50.0};

// convert to alpha-beta
let alpha_beta_zero = AlphaBeta0::from(abc);
```

The library also include [trigometric functions](crate::trig), which are useful when converter between stationary and roatating reference frames.

```rust
use ac_power::reference_frames::{Abc, Dq0};
use ac_power::trig::{Theta, cos_sin};

// create a phasor in Abc reference frame
let abc = Abc {a: 100.0, b: 200.0, c: 50.0};

// convert to Dq0
let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
let dq0 = abc.to_dq0(cos, sin);
```

The reference frames are implemented with generics, so you can use any data-type that implements the necessary numeric traits.  The crate comes with three built-in: `Voltage(f32)`, `Current(f32)`, and `Power(f32)`.

```rust
use ac_power::newtypes::{Voltage, Current, Power};

let v = Voltage::from(10.0);
let i = Current::from(1.5);
let p: Power = v * i;
```

If you muliply a voltage phasor by current phasor, you get a `Pq` struct returned.  This is a basic use case to calculate real and reactive powers from three-phase voltage and current data.

```rust

use ac_power::reference_frames::{Abc, Dq0, AlphaBeta, Polar};
use ac_power::trig::{Theta, cos_sin};
use ac_power::newtypes::{Voltage, Current};
use approx::assert_abs_diff_eq;

// set the magnitude of the voltage and current
let v_mag = Voltage::from(340.0);
let i_mag = Current::from(8.2);

// create voltage and current phasors in the Abc reference frame
let v = Abc::from_polar(v_mag, Theta::from_degrees(0.0));
let i = Abc::from_polar(i_mag, Theta::from_degrees(45.0));

// calculate P and Q
let pq = v * i;

// calculate the power factor
let pf = pq.power_factor();

// check the power factor
assert_abs_diff_eq!(f32::from(pf), 0.707, epsilon = 0.0001);

// convert v and i to alpha_beta
let v_alpha_beta = AlphaBeta::from(v);
let i_alpha_beta = AlphaBeta::from(i);

// verify the power factor is still correct
let pf = (v_alpha_beta * i_alpha_beta).power_factor();
assert_abs_diff_eq!(f32::from(pf), 0.707, epsilon = 0.0001);

```

Many inverter control systems that implement advanced grid controls or grid forming controls also rely on the transforms implemented in this crate.  Use of this crate can not only make the application code much more readible, it can improve performance and elinate bugs due to the extensive optimization and verification of this crate.

Bellow is an example of a simple three-phase Phased Locked Loop implementation, a common DSP block in inverter controls and advanced power meters, to illustrate how the crate can be used to facillitate such applications.

```rust
use ac_power::reference_frames::{Abc, AlphaBeta, Dq};
use ac_power::trig::{chebyshev, cos_sin, Cos, Sin, Theta};
use ac_power::newtypes::Voltage;
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
    pub dq_pos: Dq<Voltage>,
    pub dq_neg: Dq<Voltage>,

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
            dq_pos: Dq::zero(),
            dq_neg: Dq::zero(),
            k_theta,
        }
    }

    pub fn update(&mut self, abc: Abc<Voltage>) {
        // clarke transform
        let alpha_beta = AlphaBeta::from(abc);

        // park transforms
        self.dq_pos = alpha_beta.to_dq(self.cos, self.sin);
        self.dq_neg = alpha_beta.to_dq(self.cos, -self.sin);

        // PI loop filter
        self.f = self.fref + self.filter.update(&mut self.filter_state, self.dq_pos.q.into());

        // update the phase info
        self.theta += (self.f * self.k_theta) as i32;
        (self.cos, self.sin) = cos_sin(self.theta);
    }
}
```

# Reference frames

The crate supports 6 difference reference frames.  These include 3 balanced reference frames:

1.  [Polar](crate::reference_frames::Polar) - Polar representation (aplitude and angle)
2.  [AlphaBeta](crate::reference_frames::AlphaBeta) - Two axis (alpha and beta) stationary reference frame representation
3.  [Dq](crate::reference_frames::Dq) - Two axis (d and q) rotating reference frame representation

And three reference frames for unbalanced representations (supports a zero sequence component):

1.  [Abc](crate::reference_frames::Abc) - Instantaneous signals
2.  [AlphaBeta0](crate::reference_frames::AlphaBeta0) - Two axis (alpha and beta) stationary reference frame representation with zero
3.  [Dq0](crate::reference_frames::Dq0) - Two axis (d and q) rotating reference frame representation with zero

# Transforms

The crate supports for following lossless conversions (zero sequence retained):

| From                                                  | To                                                   |
| :---------------------------------------------------- |:----------------------------------------------------:|
| [Abc](crate::reference_frames::Abc)                   | [AlphaBeta0](crate::reference_frames::AlphaBeta0)    |
| [Abc](crate::reference_frames::Abc)                   | [Dq0](crate::reference_frames::Dq0)                  |
| [AlphaBeta0](crate::reference_frames::AlphaBeta0)     | [Abc](crate::reference_frames::Abc)                  |
| [AlphaBeta0](crate::reference_frames::AlphaBeta0)     | [Dq0](crate::reference_frames::Dq0)                  |
| [Dq0](crate::reference_frames::Dq0)                   | [Abc](crate::reference_frames::Abc)                  |
| [Dq0](crate::reference_frames::Dq0)                   | [AlphaBeta0](crate::reference_frames::AlphaBeta0)    |
| [AlphaBeta](crate::reference_frames::Dq0)             | [Abc](crate::reference_frames::Abc)                  |
| [AlphaBeta](crate::reference_frames::Dq0)             | [AlphaBeta0](crate::reference_frames::AlphaBeta0)    |
| [AlphaBeta](crate::reference_frames::Dq0)             | [Dq0](crate::reference_frames::Dq0)                  |
| [Dq](crate::reference_frames::Dq0)                    | [Abc](crate::reference_frames::Abc)                  |
| [Dq](crate::reference_frames::Dq0)                    | [AlphaBeta0](crate::reference_frames::AlphaBeta0)    |
| [Dq](crate::reference_frames::Dq0)                    | [Dq0](crate::reference_frames::Dq0)                  |

The crate supports the following lossy conversions (zero sequence lost):

| From                                                  | To                                                 |
| :---------------------------------------------------- |:--------------------------------------------------:|
| [Abc](crate::reference_frames::Abc)                   | [AlphaBeta](crate::reference_frames::AlphaBeta)    |
| [Abc](crate::reference_frames::Abc)                   | [Dq](crate::reference_frames::Dq)                  |
| [AlphaBeta0](crate::reference_frames::AlphaBeta0)     | [AlphaBeta](crate::reference_frames::AlphaBeta)    |
| [AlphaBeta0](crate::reference_frames::AlphaBeta0)     | [Dq](crate::reference_frames::Dq)                  |
| [Dq0](crate::reference_frames::Dq0)                   | [AlphaBeta](crate::reference_frames::AlphaBeta)    |
| [Dq0](crate::reference_frames::Dq0)                   | [Dq](crate::reference_frames::Dq)                  |


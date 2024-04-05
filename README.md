![continuous integration](https://github.com/unifi-consortium/ac-power/actions/workflows/rust.yml/badge.svg)

# `ac-power`

Reference frames, transforms, and trig for embedded processing of AC power signals.

# Reference Frames

At the core of the library are data structs which represent three-phase AC vectors in different reference frames.

The crate supports 6 difference reference frames.  These include 3 balanced reference frames:

1.  [Polar](crate::reference_frames::Polar) - Polar representation (amplitude and angle)
2.  [AlphaBeta](crate::reference_frames::AlphaBeta) - Orthogonal (alpha and beta) stationary reference frame representation
3.  [Dq](crate::reference_frames::Dq) - Two axis (d and q) rotating reference frame representation

And three reference frames for unbalanced representations (supports a zero sequence component):

1.  [Abc](crate::reference_frames::Abc) - Instantaneous signals
2.  [AlphaBeta0](crate::reference_frames::AlphaBeta0) - Orthogonal(alpha and beta) stationary reference frame representation with zero
3.  [Dq0](crate::reference_frames::Dq0) - Two axis (d and q) rotating reference frame representation with zero

Converting between reference frames invokes power theory transforms.

```rust
use ac_power::{Abc, AlphaBeta0};

// create a vector in Abc reference frame
let abc = Abc {a: 100.0, b: 200.0, c: 50.0};

// convert to alpha-beta
let alpha_beta_zero = AlphaBeta0::from(abc);
```

<div class="warning">This crate uses power-variant rather than power-invariant versions of the transforms, which seem to be the more common convention among industry tooling and DSP.  The integrated power calculations account for this and implement the appropriate scaling.</div>

<div class="warning">Due to floating point rounding errors, these transforms are not perfectly reversible.  For example if you did the following conversion Abc-->AlphaBeta-->Abc, the resulting Abc value would not be exactly equal to the original.</div>

# Trigonometry

The library also includes a [trig module](crate::trig), which is useful when converting between stationary and rotating reference frames.

```rust
use ac_power::{Abc, Dq0};
use ac_power::trig::{Theta, cos_sin};

// create a vector in Abc reference frame
let abc = Abc {a: 100.0, b: 200.0, c: 50.0};

// convert to Dq0
let (cos, sin) = cos_sin(Theta::from_degrees(90.0));
let dq0 = abc.to_dq0(cos, sin);
```

There are additional functions in the [trig module](crate::trig) for rotating Sin/Cos pairs or generating Sin(Nx), Cos(Nx) pairs using Chebyshev method.

# Newtypes

From the example above we see that there are some [newtypes](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) defined in this crate.  Specifically, there are three defined in the [trig module](crate::trig):

1. [Theta(i32)](crate::trig::Theta) - An angle between -π and π radians
2. [Sin(f32)](crate::trig::Sin) - Sin of an angle
3. [Cos(f32)](crate::trig::Cos) - Cos of an angle

There are also 4 additional [newtypes](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) defined in this crate:

1. [Voltage(f32)](crate::Voltage) - An electric voltage
2. [Current(f32)](crate::Current) - An electric current
3. [Power(f32)](crate::Power) - An electric power
4. [Impedance(f32)](crate::Impedance) - An electric impedance

Meaningful type conversions automatically occur during mulitplication of different types.

```rust
use ac_power::{Voltage, Current, Power, Impedance};

let z = Impedance::from(10.0);
let i = Current::from(1.5);
let v: Voltage = i * z;
let p: Power = v * i;
```

The reference frames are implemented with generics, so they can be used with regular `f32`s as seen in the examples above, or any data-type that implements the necessary numeric traits.  The 4 additional [newtypes](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) defined above all do.

```rust
use ac_power::{Abc, Voltage};

// define a voltage vector in Abc reference frame
let v: Abc::<Voltage> = Abc {a: 1.0.into(), b: 2.0.into(), c: 3.0.into()};
```

# Power Calculations

When you create AC reference frame vectors out of [Voltage](crate::Voltage) and [Current](crate::Current) types, they can be multiplied by each other to return a [Pq](crate::pq::Pq) struct.  This is a basic use case to calculate real and reactive powers from three-phase voltage and current data.

```rust

use ac_power::{Abc, Dq0, AlphaBeta, Polar, Voltage, Current};
use ac_power::trig::{Theta, cos_sin};
use approx::assert_abs_diff_eq;

// set the magnitude of the voltage and current
let v_mag = Voltage::from(340.0);
let i_mag = Current::from(8.2);

// create voltage and current vectors in the Abc reference frame
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

# Advanced Use Cases

Many inverter control systems that implement advanced grid controls or grid forming controls also rely on the transforms implemented in this crate.  Use of this crate can not only make the application code much more readible, it can improve performance and eliminate bugs due to the extensive optimization and verification of this crate.  Bellow are a few examples.

## A Grid Synchronizing Phased-Locked-Loop (PLL)

Bellow is an example of a simple three-phase Phased Locked Loop implementation, a common DSP block in inverter controls and advanced power meters, to illustrate how the crate can be used to facillitate such applications.

```rust
use ac_power::{Abc, AlphaBeta, Dq};
use ac_power::trig::{cos_sin, Cos, Sin, Theta};
use ac_power::Voltage;
use idsp::iir::{Action, Biquad, Pid};

pub struct Pll {
    fref: f32,

    // loop filter
    filter: Biquad<f32>,
    filter_state: [f32; 2],

    // frequency/angle
    pub theta: Theta,
    pub sin: Sin,
    pub cos: Cos,
    pub f: f32,

    // rotating reference frames
    pub dq_pos: Dq<Voltage>,
    pub dq_neg: Dq<Voltage>,

    // theta integration constant
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

## A Three-Phase Waveform Generator

Bellow is an example of a three-phase waveform generator that supports unbalanced representations as well as harmonics.

```rust
use ac_power::number::Num;
use ac_power::trig::{chebyshev, cos_sin, Cos, Sin, Theta};
use ac_power::{Abc, Dq};

pub struct Waveform<T, const N: usize> {
    pub positive: [Dq<T>; N],
    pub negative: [Dq<T>; N],
    pub zero: Dq<T>,
}

impl<T: Num, const N: usize> Waveform<T, N> {
    pub fn new() -> Self {
        Self {
            positive: [Dq::zero(); N],
            negative: [Dq::zero(); N],
            zero: Dq::zero(),
        }
    }

    pub fn calculate(&self, theta: Theta) -> Abc<T> {
        let (cos, sin) = cos_sin(theta);

        let mut abc = Abc::zero() + self.zero.d * sin + self.zero.q * cos;

        // add the harmonics
        let (mut cosn1, mut sinn1) = (Cos::from(1.0), Sin::from(0.0));
        let (mut cosn, mut sinn) = (cos, sin);
        for (pos, neg) in self.positive.iter().zip(self.negative.iter()) {
            abc += pos.to_abc(cosn, sinn);
            abc += neg.to_abc(cosn, -sinn);

            // use chebychev function to calculate cos, sin of next harmonic
            let cosn2 = cosn1;
            let sinn2 = sinn1;
            cosn1 = cosn;
            sinn1 = sinn;
            (cosn, sinn) = chebyshev(cos, cosn1, sinn1, cosn2, sinn2);
        }

        abc
    }
}
```

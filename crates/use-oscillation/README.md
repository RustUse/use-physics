# use-oscillation

Oscillation and simple harmonic motion helpers for `RustUse`.

## Install

```toml
[dependencies]
use-oscillation = "0.0.1"
```

## Foundation

`use-oscillation` provides small scalar helpers for simple harmonic motion, period,
frequency, angular frequency, spring oscillators, pendulum approximations, damping,
resonance, and oscillator state summaries.

Inputs are expected to be SI-style numeric values:

- seconds for period and time
- hertz for frequency
- radians per second for angular frequency
- meters for displacement, amplitude, and pendulum length
- kilograms for mass
- newtons per meter for spring constant
- meters per second squared for gravitational acceleration
- joules for energy

Radians are used for phase.

## Example

```rust
use core::f64::consts::{PI, TAU};
use use_oscillation::{SimpleHarmonicOscillator, spring_period};

let oscillator = SimpleHarmonicOscillator::new(2.0, TAU, 0.0).unwrap();
let period = spring_period(8.0, 2.0).unwrap();

assert!((oscillator.displacement(0.0).unwrap() - 2.0).abs() < 1.0e-12);
assert!((period - PI).abs() < 1.0e-12);
```

## When to use directly

Choose `use-oscillation` when you only need reusable scalar helpers for simple oscillators.

## Scope

- This crate stays `f64`-first and scalar-only.
- It is not a wave library, signal-processing library, acoustics library, numerical simulator, or control-system package.
- Wave abstractions belong in the top-level `use-wave` set.
- Signal processing belongs in the top-level `use-signal` set.
- Acoustics belongs in the top-level `use-acoustics` set.
- Units belong in the top-level `use-units` set.

## Status

`use-oscillation` is a pre-1.0 crate with a deliberately small API.

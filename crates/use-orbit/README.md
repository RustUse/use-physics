# use-orbit

Orbital mechanics helpers for `RustUse`.

## Install

```toml
[dependencies]
use-orbit = "0.0.1"
```

## Foundation

`use-orbit` provides small `f64`-first helpers for circular and elliptical two-body orbital mechanics.

It focuses on scalar orbital relations such as gravitational parameter, orbital speed,
orbital period, semi-major axis, apoapsis and periapsis relations, vis-viva speed,
escape relations, and simple orbital state summaries.

Inputs are expected to be SI-style numeric values:

- kilograms for mass
- meters for radius, altitude, apsides, and semi-major axis
- cubic meters per second squared for gravitational parameter
- meters per second for speed and delta-v
- seconds for orbital period and transfer time
- joules per kilogram for specific orbital energy

The crate does not define a full unit system.

General gravity helpers belong in `use-gravity`.
General constants belong in the top-level `use-constants` set.
Unit abstractions belong in the top-level `use-units` set.
Simulation and numerical propagation belong in the top-level `use-simulation` set or a
future specialized crate.

## Example

```rust
use use_orbit::{CentralBody, EllipticalOrbit, hohmann_total_delta_v};

let earth = CentralBody::with_radius(5.972e24, 6.371e6);
let low_orbit_radius = earth.and_then(|body| body.orbital_radius_from_altitude(400_000.0));
let low_orbit_speed = earth.and_then(|body| {
	low_orbit_radius.and_then(|radius| body.circular_orbital_speed_at_radius(radius))
});
let transfer = hohmann_total_delta_v(398_600_441_800_000.0, 6_771_000.0, 42_164_000.0);

assert_eq!(low_orbit_radius, Some(6_771_000.0));
assert!(low_orbit_speed.is_some_and(|speed| speed > 7_600.0));
assert!(transfer.is_some_and(|delta_v| delta_v > 0.0));
assert!(EllipticalOrbit::new(100.0, 10.0, 20.0)
	.and_then(|orbit| orbit.period())
	.is_some_and(|period| period > 0.0));
```

## When to use directly

Choose `use-orbit` when you need small reusable orbit formulas without a larger astrodynamics stack.

## Scope

- APIs stay `f64`-first and dependency-free.
- The crate focuses on two-body scalar orbital helpers, not n-body simulation or ephemeris modeling.
- The crate keeps only a couple of convenience constants locally.
- Broader constants and units belong in the top-level `use-constants` and `use-units` sets.

## Status

`use-orbit` is a pre-1.0 crate with a deliberately small API.

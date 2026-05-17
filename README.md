# RustUse/use-physics

Composable `f64`-first mechanics, oscillation, rotational motion, torque, fluid flow, electricity,
magnetism, electromagnetism, plasma physics, gravity, orbital mechanics, momentum, special
relativity, quantum physics, particle metadata, nuclear physics, radiation physics, and
thermodynamics helpers for Rust.

## Workspace crates

| Crate                  | Path                           | Purpose                                              |
| ---------------------- | ------------------------------ | ---------------------------------------------------- |
| `use-physics`          | `crates/use-physics/`          | Feature-gated facade over the focused physics crates |
| `use-motion`           | `crates/use-motion/`           | Basic kinematics helpers                             |
| `use-oscillation`      | `crates/use-oscillation/`      | Simple harmonic motion and oscillator helpers        |
| `use-rotation`         | `crates/use-rotation/`         | Rotational motion and angular dynamics helpers       |
| `use-force`            | `crates/use-force/`            | Force, weight, and impulse helpers                   |
| `use-torque`           | `crates/use-torque/`           | Torque, lever-arm, and rotational balance helpers    |
| `use-energy`           | `crates/use-energy/`           | Work and mechanical energy helpers                   |
| `use-work`             | `crates/use-work/`             | Mechanical work and work-energy relation helpers     |
| `use-power`            | `crates/use-power/`            | Average, mechanical, and electrical power helpers    |
| `use-fluid`            | `crates/use-fluid/`            | Fluid mechanics scalar helpers                       |
| `use-electricity`      | `crates/use-electricity/`      | Electricity and simple circuit helpers               |
| `use-magnetism`        | `crates/use-magnetism/`        | Magnetism and magnetic-field helpers                 |
| `use-electromagnetism` | `crates/use-electromagnetism/` | Combined electric and magnetic field scalar helpers  |
| `use-plasma`           | `crates/use-plasma/`           | Plasma physics scalar helpers                        |
| `use-pressure`         | `crates/use-pressure/`         | Pressure and hydrostatic pressure helpers            |
| `use-density`          | `crates/use-density/`          | Density, mass, and volume helpers                    |
| `use-gravity`          | `crates/use-gravity/`          | Gravity, orbit, and gravitational energy helpers     |
| `use-orbit`            | `crates/use-orbit/`            | Orbital mechanics scalar helpers                     |
| `use-momentum`         | `crates/use-momentum/`         | Momentum, impulse, recoil, and collision helpers     |
| `use-relativity`       | `crates/use-relativity/`       | Special relativity scalar helpers                    |
| `use-quantum`          | `crates/use-quantum/`          | Quantum physics scalar helpers                       |
| `use-particle`         | `crates/use-particle/`         | Particle classification and metadata helpers         |
| `use-nuclear`          | `crates/use-nuclear/`          | Radioactive decay and nuclear scalar helpers         |
| `use-radiation`        | `crates/use-radiation/`        | Radiation intensity, dose, and attenuation helpers   |
| `use-thermodynamics`   | `crates/use-thermodynamics/`   | Ideal gas and heat-energy helpers                    |

## Installation

Use the workspace directly, or depend on a Git revision when you need workspace changes that are
newer than the latest published crates.io release.

## Basic usage

```rust
use use_energy::kinetic_energy;
use use_electricity::voltage;
use use_force::force;
use use_momentum::momentum;
use use_nuclear::activity;
use use_oscillation::period_from_frequency;
use use_particle::{ParticleKind, charge};
use use_rotation::angular_velocity;
use use_torque::torque;
use use_work::work_at_angle_degrees;

let applied_force = force(10.0, 2.0);
let applied_torque = torque(10.0, 2.0);
let circuit_voltage = voltage(2.0, 5.0);
let energy = kinetic_energy(2.0, 3.0);
let linear_momentum = momentum(2.0, 3.0);
let sample_activity = activity(2.0, 10.0);
let oscillation_period = period_from_frequency(2.0);
let electron_charge = charge(ParticleKind::Electron);
let angled_work = work_at_angle_degrees(10.0, 2.0, 60.0);
let spin_rate = angular_velocity(10.0, 2.0);

assert_eq!(applied_force, 20.0);
assert_eq!(applied_torque, Some(20.0));
assert_eq!(circuit_voltage, Some(10.0));
assert_eq!(energy, 9.0);
assert_eq!(linear_momentum, Some(6.0));
assert_eq!(sample_activity, Some(20.0));
assert_eq!(oscillation_period, Some(0.5));
assert_eq!(electron_charge.thirds, -3);
assert_eq!(angled_work, Some(10.0));
assert_eq!(spin_rate, Some(5.0));
```

## License

Licensed under either of the following, at your option:

- MIT license: https://github.com/RustUse/.github/blob/main/LICENSE-MIT
- Apache License, Version 2.0: https://github.com/RustUse/.github/blob/main/LICENSE-APACHE

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the local validation flow and repository policy.

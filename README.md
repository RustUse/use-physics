# RustUse/use-physics

Composable `f64`-first mechanics and thermodynamics helpers for Rust.

## Workspace crates

| Crate                | Path                         | Purpose                                              |
| -------------------- | ---------------------------- | ---------------------------------------------------- |
| `use-physics`        | `crates/use-physics/`        | Feature-gated facade over the focused physics crates |
| `use-motion`         | `crates/use-motion/`         | Basic kinematics helpers                             |
| `use-force`          | `crates/use-force/`          | Force, weight, and impulse helpers                   |
| `use-energy`         | `crates/use-energy/`         | Work and mechanical energy helpers                   |
| `use-power`          | `crates/use-power/`          | Average, mechanical, and electrical power helpers    |
| `use-pressure`       | `crates/use-pressure/`       | Pressure and hydrostatic pressure helpers            |
| `use-density`        | `crates/use-density/`        | Density, mass, and volume helpers                    |
| `use-thermodynamics` | `crates/use-thermodynamics/` | Ideal gas and heat-energy helpers                    |

## Installation

Use the workspace directly or depend on a Git revision until the first crates.io release is published.

## Basic usage

```rust
use use_energy::kinetic_energy;
use use_force::force;

let applied_force = force(10.0, 2.0);
let energy = kinetic_energy(2.0, 3.0);

assert_eq!(applied_force, 20.0);
assert_eq!(energy, 9.0);
```

## License

Licensed under either of the following, at your option:

- MIT license: https://github.com/RustUse/.github/blob/main/LICENSE-MIT
- Apache License, Version 2.0: https://github.com/RustUse/.github/blob/main/LICENSE-APACHE

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the local validation flow and repository policy.

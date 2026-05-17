# Release Policy

The root workspace metadata keeps `publish = false` as the default, while the publishable crate
manifests opt in with `publish = true`.

## First Publish Wave

The intended first publish candidates are every focused crate under `crates/` plus `use-physics`.

Publish all focused crates first in this dry-run and manual publish order:

1. `use-motion`
2. `use-rotation`
3. `use-force`
4. `use-torque`
5. `use-energy`
6. `use-work`
7. `use-power`
8. `use-fluid`
9. `use-electricity`
10. `use-magnetism`
11. `use-electromagnetism`
12. `use-pressure`
13. `use-density`
14. `use-gravity`
15. `use-momentum`
16. `use-particle`
17. `use-nuclear`
18. `use-thermodynamics`

Wait for crates.io index propagation, then publish `use-physics`.

## Publish Surface

Before the first publish wave, confirm that the release surface:

- keeps the workspace-level default at `publish = false`
- keeps every focused crate under `crates/` at `publish = true`
- keeps `crates/use-physics/Cargo.toml` at `publish = true`

## Versioning

- The workspace currently uses lockstep `0.x.y` versioning.
- Before `1.0`, breaking changes should bump the minor version.
- Before `1.0`, additive compatible changes should bump the patch version.

## Automated Release Validation

The repository includes a dedicated release-validation path:

- `.github/workflows/publish-readiness.yml` runs on pull requests, pushes to `main`, and manual dispatch.
- `make release-readiness` runs the same high-value local checks for examples, no-default-features coverage, and focused-crate publish dry-runs.
- The focused-crate dry-run path uses the same ordered first-wave list that the maintainer docs use for the initial public release.
- `.github/workflows/facade-publish-readiness.yml` is a manual post-publication check that dry-runs `use-physics` only after the focused crates are live on crates.io.

## Version and Changelog Automation

The repository includes `release-plz` configuration in `release-plz.toml` and maintainer workflows under `.github/workflows/release-plz-*.yml`.

- `Release PR Automation` opens or updates a release PR with lockstep version changes for every publishable crate in the workspace.
- The workspace is configured with one `version_group` so all published crates keep the same version.
- The root `CHANGELOG.md` remains the shared changelog and is updated through the `use-physics` package entry.

## Publish Readiness Checklist

1. Confirm `cargo fmt` is clean.
2. Confirm `cargo check --workspace --all-features` passes.
3. Confirm `cargo check --workspace --all-features --examples` passes.
4. Confirm `cargo test --workspace --all-features` passes.
5. Confirm `cargo test --workspace --no-default-features` passes.
6. Confirm `cargo clippy --workspace --all-targets --all-features` passes.
7. Confirm `cargo deny check` and `cargo audit` pass.
8. Review README examples, crate metadata, `Cargo.lock`, and changelog entries.
9. Confirm the focused-crate dry-run order remains `use-motion`, `use-rotation`, `use-force`, `use-torque`, `use-energy`, `use-work`, `use-power`, `use-fluid`, `use-electricity`, `use-magnetism`, `use-electromagnetism`, `use-pressure`, `use-density`, `use-gravity`, `use-momentum`, `use-particle`, `use-nuclear`, then `use-thermodynamics`.

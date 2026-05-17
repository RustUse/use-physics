# Maintainer Release Flow

This document describes how maintainers should run releases with the current `release-plz` setup.

## Current model

- `Release PR Automation` opens or updates a release PR from `main`.
- `release-plz` keeps every publishable crate in the workspace in one lockstep version group.
- The shared root `CHANGELOG.md` is generated through the `use-physics` package entry and includes focused-crate commits.
- `Release Publish Automation` runs automatically on pushes to `main` after the initial manual publish wave is complete and the repository enables the guarded auto-publish path.

## One-time post-initial-release setup

- Configure crates.io Trusted Publishing for every published crate with repository owner `RustUse`, repository name `use-physics`, and workflow filename `release-plz-release.yml`.
- Set the repository variable `CRATES_IO_AUTOPUBLISH_ENABLED` to `true` only after the first manual crates.io wave is complete.

## Normal post-initial-release flow

1. Merge ordinary PRs into `main` with clean conventional-commit style in the final commit subject or squash-merge title.
2. Let `Release PR Automation` open or update the release PR.
3. Review the lockstep version bump and generated `CHANGELOG.md`.
4. Merge the release PR after the required checks pass.
5. Let `Release Publish Automation` publish from the merged release commit, or manually dispatch it with `post-initial-release = true` if you need a controlled rerun.

## Initial public release exception

Do not use `Release Publish Automation` for the first public crates.io wave.

1. Confirm every focused crate plus `use-physics` are still the intended first-wave publishable crates.
2. Run the full publish-readiness checks.
3. Dry-run and then publish focused crates in this order: `use-motion`, `use-oscillation`, `use-rotation`, `use-rigidbody`, `use-force`, `use-torque`, `use-energy`, `use-work`, `use-power`, `use-fluid`, `use-electricity`, `use-magnetism`, `use-electromagnetism`, `use-plasma`, `use-pressure`, `use-density`, `use-gravity`, `use-orbit`, `use-momentum`, `use-collision`, `use-relativity`, `use-quantum`, `use-particle`, `use-nuclear`, `use-radiation`, `use-thermodynamics`.
4. Wait for crates.io index propagation.
5. Run `cargo publish --dry-run -p use-physics` or the manual `Facade Publish Readiness` workflow.
6. Publish `use-physics`.

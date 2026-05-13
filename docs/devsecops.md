# DevSecOps Foundation

This repository uses a pragmatic GitHub Actions baseline to keep Rust CI, dependency hygiene, static analysis, secret detection, and SBOM generation reviewable for the first public release wave.

## Workflows

### `CI`

File: `.github/workflows/ci.yml`

- Triggers on pull requests to `main` and pushes to `main`
- Runs `cargo fmt --all -- --check`
- Runs `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- Runs `cargo test --workspace --all-features`
- Runs `cargo build --workspace --all-features`

### `Pull Request Quality Gate`

File: `.github/workflows/pull-request.yml`

- Re-runs the main Rust quality checks on pull requests
- Keeps one readable required status check for merge policy

### `Cargo Audit`

File: `.github/workflows/cargo-audit.yml`

- Installs `cargo-audit`
- Runs `cargo audit`

### `Cargo Deny`

File: `.github/workflows/cargo-deny.yml`

- Installs `cargo-deny`
- Runs `cargo deny check`

### `CodeQL`

File: `.github/workflows/codeql.yml`

- Runs GitHub CodeQL for Rust on public-repository runs
- Builds the workspace with all features enabled before analysis

### `Gitleaks`

File: `.github/workflows/secrets.yml`

- Scans the full repository history for secrets

### `Trivy`

File: `.github/workflows/trivy.yml`

- Scans the repository filesystem for high and critical findings

### `SBOM`

File: `.github/workflows/sbom.yml`

- Generates a CycloneDX JSON SBOM from `crates/use-physics/Cargo.toml`
- Uploads the result as a workflow artifact

### `Advisory Rust Quality`

File: `.github/workflows/advisory-rust-quality.yml`

- Runs `cargo-machete` in advisory mode
- Runs `cargo-semver-checks` in advisory mode against the pull request base revision

### `Publish Readiness`

File: `.github/workflows/publish-readiness.yml`

- Compiles workspace examples with all features enabled
- Tests the workspace without default features
- Runs `cargo publish --dry-run --allow-dirty` for every focused crate in the workspace

### `Facade Publish Readiness`

File: `.github/workflows/facade-publish-readiness.yml`

- Verifies that every focused crate already resolves from crates.io
- Runs `cargo publish --dry-run --allow-dirty -p use-physics`

### `Release PR Automation`

File: `.github/workflows/release-plz-pr.yml`

- Runs `release-plz release-pr` to prepare lockstep version bumps and changelog updates

### `Release Publish Automation`

File: `.github/workflows/release-plz-release.yml`

- Runs `release-plz release` for post-initial-release publishing
- Uses GitHub OIDC trusted publishing

## Local Commands

The fastest local path is still plain Cargo:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check --workspace --all-features --examples
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo build --workspace --all-features
```

The repository also provides a `Makefile` with these convenience commands:

- `make fmt`
- `make lint`
- `make test`
- `make test-minimal`
- `make build`
- `make examples`
- `make audit`
- `make deny`
- `make sbom`
- `make publish-dry-run-focused`
- `make publish-dry-run-facade`
- `make release-readiness`
- `make facade-post-publish-validation`
- `make verify`

`make sbom` writes the generated CycloneDX file under `crates/use-physics/`.

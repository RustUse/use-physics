# Forge Strategy

RustUse may be mirrored across multiple public Git forges to improve
availability, discoverability, and contributor access without fragmenting the
project.

## Canonical repository

GitHub is currently the canonical write target for RustUse/use-physics.

Pull requests that change the main branch, tags, releases, or publishing state
are finalized there.

GitLab mirror support is documented in this repository, but live activation is
optional until the mirror URLs and matching SSH key material are configured.

## Mirrors

Mirrors may exist on GitLab, Codeberg or Forgejo, SourceHut, or other public
forges.

- GitLab is the first supported mirror surface and may accept issues or merge
  requests.
- Codeberg or Forgejo is the recommended second mirror once the GitLab path is
  stable.
- SourceHut is treated as an optional later mirror with documentation first,
  manual sync if needed, and no dedicated CI for now.

## Recommended rollout order

RustUse should expand mirror support in stages instead of trying to automate all
forges at once.

1. Start with GitHub as canonical plus a public GitLab mirror.
2. Add Codeberg or Forgejo when a second public mirror is useful for redundancy
   or contributor access.
3. Add SourceHut only when the project is ready to maintain another distinct
   contribution surface.

For the first public launch, it is acceptable to keep the mirror workflow
dormant and defer live mirror activation until after the canonical GitHub
surface is stable.

Dedicated Forgejo workflow files are intentionally omitted at this stage. Some
Forgejo hosts can reuse GitHub Actions style workflows, and RustUse should avoid
duplicate CI until a specific mirror host actually needs separate automation.

## Activation checklist

The checked-in `.github/workflows/mirror.yml` workflow stays dormant until the
canonical repository has:

- `GITLAB_MIRROR_URL`
- optional `CODEBERG_MIRROR_URL`
- optional `SOURCEHUT_MIRROR_URL`
- `GITLAB_MIRROR_SSH_KEY`
- optional `CODEBERG_MIRROR_SSH_KEY`
- optional `SOURCEHUT_MIRROR_SSH_KEY`

Activation should happen in this order:

1. Configure GitLab first and confirm canonical GitHub CI is already stable.
2. Verify the mirror URLs are push targets for the canonical repository only.
3. Confirm each configured mirror URL has a matching SSH key secret.
4. Keep `scripts/sync-mirrors.sh` available as a manual fallback and emergency audit path.

## Contribution flow

Contributions may originate from any supported platform. Before final merge and
release, accepted changes are brought into the canonical GitHub repository.

When a maintainer ports an external merge request or patch into GitHub, the
final canonical pull request should preserve authorship and reference the
original issue, merge request, or patch URL.

## Sync model

- GitHub `main` mirrors outward.
- Tags mirror outward.
- Releases are coordinated from GitHub.
- crates.io publishing happens only from canonical release automation.
- Mirror issues or merge requests may be referenced in GitHub pull requests.
- Mirror sync may be handled manually or by a GitHub-hosted mirroring workflow
  after canonical CI succeeds and remotes and secrets are configured.

## Security and provenance

- Release authority belongs to the canonical repository.
- crates.io publishing must not run from mirrors.
- Mirror repositories should not receive publish tokens.
- Mirror SSH keys should be scoped only to repository mirroring, not release automation.
- Mirror-only CI is validation, not release authority.
- External contributions should preserve authorship when ported.
- Reference the original issue, merge request, or patch URL in the final
  canonical pull request.

## Why not multi-primary?

Multi-primary Git hosting adds avoidable conflict, moderation, release, and
provenance complexity. RustUse starts with a simpler model: many public entry
points, one canonical release path.

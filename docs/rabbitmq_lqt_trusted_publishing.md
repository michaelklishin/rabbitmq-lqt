# Trusted Publishing Setup: `rabbitmq-lqt`

> **Repository:** [github.com/michaelklishin/rabbitmq-lqt](https://github.com/michaelklishin/rabbitmq-lqt)
> **Crates:** [`rabbitmq-lqt-lib`](https://crates.io/crates/rabbitmq-lqt-lib), [`rabbitmq-lqt-obfuscation`](https://crates.io/crates/rabbitmq-lqt-obfuscation), [`rabbitmq-lqt-ql-core`](https://crates.io/crates/rabbitmq-lqt-ql-core), [`rabbitmq-lqt-ql`](https://crates.io/crates/rabbitmq-lqt-ql), [`rabbitmq-lqt-ui`](https://crates.io/crates/rabbitmq-lqt-ui), [`rabbitmq-lqt-cli`](https://crates.io/crates/rabbitmq-lqt-cli)
> **Registry:** crates.io

## Overview

This document describes how to adopt crates.io Trusted Publishing for the `rabbitmq-lqt` workspace.
Trusted Publishing replaces long-lived API tokens with short-lived OIDC credentials scoped to
a specific GitHub Actions workflow, eliminating stored secrets from the publishing pipeline.

This is a **workspace with six publishable crates** that must be published in dependency order:
`rabbitmq-lqt-lib` → `rabbitmq-lqt-obfuscation` → `rabbitmq-lqt-ql-core` → `rabbitmq-lqt-ql` → `rabbitmq-lqt-ui` → `rabbitmq-lqt-cli`.

One workspace crate (`rabbitmq-lqt-ql-wasm`) is marked `publish = false` and is excluded.


## Prerequisites

- You are an **owner** of all six crates on crates.io
- Each crate has at least one published version
- You have admin access to `michaelklishin/rabbitmq-lqt` on GitHub


## Step 1: Create a GitHub Actions Environment

1. Go to **Settings → Environments** in the repository
2. Click **New environment**, name it `release`
3. Configure protection rules (all optional but recommended):
   - **Required reviewers:** add yourself. This prevents accidental publishes
     from automated tag pushes without human approval
   - **Deployment branches:** restrict to `main` (or your default branch) and tags matching `v*`

The environment name `release` must match the value in the workflow file **and** the
Trusted Publisher Configuration on crates.io. If you omit the environment, leave it
blank in both places.


## Step 2: Register Trusted Publishers on crates.io

Register a Trusted Publisher for **each** of the six crates. Repeat for every crate:

1. Go to `crates.io/crates/<crate-name>/settings`
2. Under **Trusted Publishing**, click **Add** and fill in:

| Field                 | Value                                |
|-----------------------|--------------------------------------|
| GitHub owner          | `michaelklishin`                     |
| Repository name       | `rabbitmq-lqt`                       |
| Workflow filename     | `release.yml`                        |
| Environment (optional)| `release`                            |

3. Save. The workflow filename is **case-sensitive** and must match exactly

Crates to configure:
- [`rabbitmq-lqt-lib`](https://crates.io/crates/rabbitmq-lqt-lib/settings)
- [`rabbitmq-lqt-obfuscation`](https://crates.io/crates/rabbitmq-lqt-obfuscation/settings)
- [`rabbitmq-lqt-ql-core`](https://crates.io/crates/rabbitmq-lqt-ql-core/settings)
- [`rabbitmq-lqt-ql`](https://crates.io/crates/rabbitmq-lqt-ql/settings)
- [`rabbitmq-lqt-ui`](https://crates.io/crates/rabbitmq-lqt-ui/settings)
- [`rabbitmq-lqt-cli`](https://crates.io/crates/rabbitmq-lqt-cli/settings)


## Step 3: Rename the Release Workflow

The workflow file must be renamed from `release.yaml` to `release.yml` to match the
filename registered on crates.io. The filename is **case-sensitive**.


## Step 4: Update the Release Workflow

The existing release workflow has been updated to:

1. **Migrate from `rust-build-package-release-action@v1` to `@v2`** across all jobs
2. **Add a `publish` job** that publishes all six crates to crates.io in dependency order
   using Trusted Publishing (OIDC)
3. **Add `id-token: write` permission** to enable OIDC token exchange

The `publish` job uses `rust-build-package-release-action`'s `publish-crate` command
with the `package` input to publish each workspace crate in dependency order. The command
wraps `cargo publish` with automatic version validation (tag vs `Cargo.toml`) and a
dry-run pre-check before the real publish. It also treats "version already exists" as
a no-op, which makes the pipeline idempotent.

### Dependency order

```
rabbitmq-lqt-lib         (no workspace deps)
rabbitmq-lqt-obfuscation (no workspace deps)
rabbitmq-lqt-ql-core     (no workspace deps)
rabbitmq-lqt-ql          (depends on: rabbitmq-lqt-ql-core, rabbitmq-lqt-lib)
rabbitmq-lqt-ui          (depends on: rabbitmq-lqt-lib, rabbitmq-lqt-ql)
rabbitmq-lqt-cli         (depends on: rabbitmq-lqt-lib, rabbitmq-lqt-obfuscation, rabbitmq-lqt-ql, rabbitmq-lqt-ui)
```

### What each piece does

- **`permissions.id-token: write`** — lets GitHub Actions request a signed JWT from
  GitHub's OIDC provider. Without this, the token exchange will fail
- **`rust-lang/crates-io-auth-action@v1`** — exchanges the GitHub OIDC JWT for a
  short-lived crates.io publish token (~30 min). Its **post-step** automatically revokes
  the token when the job completes, even on failure
- **`environment: release`** — ties the job to the GitHub Environment created in Step 1,
  enforcing any protection rules you configured
- **`validate-version`** — checks that the git tag matches the version in `Cargo.toml`,
  catching mismatches before attempting to publish
- **`publish-crate` with `package`** — publishes a specific workspace crate. Runs
  `cargo publish --dry-run` first to catch packaging errors, then performs the real
  publish. Outputs `version` and `published` status
- **Dependency order** — crates are published sequentially matching the workspace
  dependency graph


## Step 5: Test

1. Bump the version in `Cargo.toml` (a patch or pre-release is fine)
2. Commit, tag, and push:

```bash
git tag v<new-version>
git push origin v<new-version>
```

3. Watch the workflow in the **Actions** tab
4. Verify each crate's new version appears on crates.io

If the environment has required reviewers, you will need to approve the deployment
in the GitHub UI before the job proceeds.


## Step 6: Enforce Trusted Publishing (Optional)

Once you have confirmed that at least one release has succeeded via the new workflow,
for **each** of the six crates:

1. Go to `crates.io/crates/<crate-name>/settings`
2. Enable **Require Trusted Publishing** (if available)

This disables all traditional API-token-based publishing for the crate. Even if an
old token leaked, it could no longer be used to publish.


## Step 7: Clean Up Old Tokens

- If you previously stored a `CARGO_REGISTRY_TOKEN` or `CRATES_TOKEN` secret in the
  repository (Settings → Secrets and variables → Actions), **delete it**
- Revoke the corresponding API token on [crates.io/settings/tokens](https://crates.io/settings/tokens)

Confirm active tokens on the crates.io tokens page.


## Troubleshooting

| Symptom | Likely cause |
|---------|-------------|
| `403 Forbidden` on token exchange | Workflow filename, owner, repo, or environment doesn't match the Trusted Publisher Configuration on crates.io (case-sensitive). Must be registered for **each** crate |
| `Error: OIDC token not available` | Missing `permissions: id-token: write` in the workflow, or the job isn't running on a GitHub-hosted runner |
| Job hangs waiting for approval | The `release` environment has required reviewers configured. Approve it in the Actions UI |
| `401 Unauthorized` on `cargo publish` | The `crates-io-auth-action` step didn't run or its output wasn't passed correctly. Check the `CARGO_REGISTRY_TOKEN` env var |
| Later crate fails with dependency not found | The earlier crate may not have propagated on crates.io yet. Re-run the job — `publish-crate` skips already-published versions |


## References

- [crates.io Trusted Publishing docs](https://crates.io/docs/trusted-publishing)
- [RFC 3691 — Trusted Publishing for crates.io](https://rust-lang.github.io/rfcs/3691-trusted-publishing-cratesio.html)
- [`rust-lang/crates-io-auth-action`](https://github.com/rust-lang/crates-io-auth-action)
- [GitHub OIDC documentation](https://docs.github.com/en/actions/security-for-github-actions/security-hardening-your-deployments/about-security-hardening-with-openid-connect)

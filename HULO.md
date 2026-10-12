# HULO fork of epanet-rs

## Purpose

This fork carries HULO patches to [epanet-rs](https://github.com/Vitens/epanet-rs).
We test each patch against EPANET 2.3.5 on HULO's validation levels.
We send each patch upstream to `Vitens/epanet-rs` as a pull request.
When upstream merges a patch, we drop it from this fork at the next sync.

## Branches

- `master` mirrors upstream `master`. Update it only by a fast-forward to `upstream/master` (see
  Sync with upstream). Do not commit HULO work on it.
- `hulo/integration` is the branch that swg-core pins. It is `master` plus the HULO patches.
- `hulo/hh-NNNN-<slug>` holds one patch for one ticket. Open it from `hulo/integration`.
- Merge each patch branch into `hulo/integration` with a squash merge, through a pull request.
  The squash commit is the patch: one commit per patch, recorded in the Patches table.

`master` and `hulo/integration` are protected: a pull request is necessary, and force pushes and
deletions are not permitted. The protection is not enforced for admins. The one authorised use of
this admin bypass is the fast-forward of `master` in a sync (step 1 of Sync with upstream).

## Tags

- After each solver-patch merge into `hulo/integration`, tag the merge commit
  `v<upstream version>-hulo.N`. The upstream version is the `version` in `Cargo.toml` (today
  `0.2.3`).
- A sync merge also gets a tag (step 5 of Sync with upstream). No other merge gets a tag: for
  example, the set-up merge that adds the fork files (`HULO.md`, `.github/`) gets no tag.
- N counts up by 1 for each tag. It restarts at 1 after a sync that changes the upstream
  version. A sync that keeps the upstream version continues the count, because a tag name can
  exist only once.
- The baseline is `v0.2.3-hulo.0` = `83842e1` (the integration head after the set-up merge: upstream `1a387056` plus the fork files) (upstream `master` after the 0.2.3 release, no
  HULO patch).
- Do not change the crate `version` in `Cargo.toml`. swg-core asks for `^0.2.2`, and a
  prerelease version (for example `0.2.3-hulo.1`) does not match it: Cargo then only warns
  "patch was not used" and builds with the crates.io release.
- A tag never starts the release workflow (see Continuous integration).

## How swg-core pins this fork

swg-core keeps the `epanet-rs` version from crates.io and replaces the source in its root
`Cargo.toml`:

```toml
[patch.crates-io]
epanet-rs = { git = "https://github.com/daniel-hulo/epanet-rs", rev = "<commit>" }
```

Use the full commit of a `v<upstream version>-hulo.N` tag on `hulo/integration`.

## Patches

One row for each solver patch. Status values:

- `open`: the pull request into `hulo/integration` is open.
- `merged`: the patch is in `hulo/integration`.
- `upstreamed`: upstream has merged the patch; drop it at the next sync.
- `dropped`: the patch is no longer in `hulo/integration`.

| Tag | Squash commit | Finding | Upstream PR | Status |
|---|---|---|---|---|
| (at merge) | (at merge) | HH-4658: adds `SolverState::solve_stats` (`SolveStats`: GGA iterations and link status change at exit, per solve); the INP reader gives an error, not a solver panic, for a GPV without a curve, a GPV/PCV curve with 1 point, and C-M head loss | (not sent) | open |

## Sync with upstream

Do not rebase `hulo/integration`. It is protected against force pushes, and the squash commits
of the patches must stay. A sync changes `hulo/integration` only through the merge pull request
from the sync branch (step 4): never a rebase, never a force-push. A sync is a merge:

1. An admin fast-forwards `master` to `upstream/master` and pushes it directly
   (`git push origin upstream/master:refs/heads/master`, a fast-forward, not a force push).
   The pull request rule on `master` does not apply to admins, so the push is accepted. This is
   the one authorised bypass of the branch protection. Do not push anything else to `master`.
2. Make a branch `hulo/sync-<date>` from `hulo/integration`, and merge `master` into it
   (`git merge master`, no rebase).
3. In the same branch, drop the patches that upstream has merged:
   - If upstream merged the patch as it is, the merge already holds the change once. Do not
     revert the squash commit: that removes upstream's copy of the change too.
   - If upstream merged a different version of the change, revert the squash commit
     (`git revert <squash commit>`) and keep upstream's version.
   - Check the result: `git diff master hulo/sync-<date>` shows only the open patches and the
     fork files (`HULO.md`, `.github/`).
4. Open a pull request `chore(fork): sync upstream <date>` into `hulo/integration`.
   Merge it with a merge commit, not a squash merge, so that `master` stays an ancestor of
   `hulo/integration`.
5. Run the HULO validation levels again, then tag the merge commit (see Tags).
6. Set the status of each dropped patch to `dropped` in the Patches table.

## Continuous integration

Only `rust.yml` runs on pushes: on each push to `master` and `hulo/**` branches, and on each pull
request into them. `validate.yml` runs only on pull requests into `hulo/integration` and on
demand. `benchmark.yml` and `release.yml` run on demand only.

| Workflow | What it does | When it runs |
|---|---|---|
| `rust.yml` | Format check, clippy, build and tests. | Each push to, and each pull request into, `master` and `hulo/**` branches. |
| `validate.yml` | Builds EPANET 2.3.5 (tag `v2.3.5`) from source, then runs `epanet-rs validate` against `runepanet` on each network of `epanet-example-networks`. | Each pull request into `hulo/integration`, and on demand. |
| `benchmark.yml` | Times epanet-rs against EPANET 2.3.5 with `hyperfine` on Linux and Windows. | On demand only. |
| `release.yml` | Builds release binaries for Linux, macOS and Windows, and makes a GitHub release. | On demand only. |

Why `validate.yml`, `benchmark.yml` and `release.yml` do not run on each push:

- `validate.yml` builds EPANET from source and runs long comparisons. That is too slow for each
  push. It is useful on a patch, so it runs on each pull request into `hulo/integration`.
- `benchmark.yml` gives timings for a person to read. It does not decide a merge.
- `release.yml` must not run on a tag: the fork does not make releases, because swg-core pins a
  commit. Upstream's tag trigger is removed, so a `v*-hulo.N` tag does not start it.

# HULO fork of epanet-rs

## Purpose

This fork carries HULO patches to [epanet-rs](https://github.com/Vitens/epanet-rs).
We test each patch against EPANET 2.3.5 on HULO's validation levels.
We send each patch upstream to `Vitens/epanet-rs` as a pull request.
When upstream merges a patch, we drop it from this fork at the next sync.

## Branches

- `master` mirrors upstream `master`. Update it by fast-forward only. Do not commit HULO work on it.
- `hulo/integration` is the branch that swg-core pins. It is `master` plus the HULO patches.
- `hulo/hh-NNNN-<slug>` holds one patch for one ticket. Open it from `hulo/integration`.
- Merge each patch branch into `hulo/integration` with a squash merge, through a pull request.
- After each merge, tag the merge commit `v0.2.3-hulo.N` (N = 1, 2, 3, ...).

`master` and `hulo/integration` are protected: a pull request is necessary, and force pushes and
deletions are not permitted.

## How swg-core pins this fork

swg-core keeps the `epanet-rs` version from crates.io and replaces the source in its root
`Cargo.toml`:

```toml
[patch.crates-io]
epanet-rs = { git = "https://github.com/daniel-hulo/epanet-rs", rev = "<commit>" }
```

Use the full commit of a `v0.2.3-hulo.N` tag on `hulo/integration`.

## Patches

| Tag | Branch | Finding | Upstream PR |
|---|---|---|---|
| (none) | `hulo/hh-4647-fork-setup` | Fork set-up (HH-4647): this file, CI on demand only. No solver patch. | Not applicable |

## Sync with upstream

To take new upstream work, fast-forward `master` to `upstream/master`.
Then rebase `hulo/integration` onto the new `master` in its own pull request, named
`chore(fork): sync upstream <date>`. Remove patches that upstream has merged, and run the
HULO validation levels again before the next tag.

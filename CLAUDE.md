# CLAUDE.md - cheat-guard

## What is cheat-guard?

Runtime macros for cheat-aware error handling. Provides `cheat_bail!` and `cheat_ensure!` with documented cheat vectors.

Based on [Anthropic's Reward Hacking Research](https://www.anthropic.com/research/emergent-misalignment-reward-hacking).

## What Belongs Here

- `macro_rules!` macros for runtime checks
- Re-exports of proc-macros from `cheat-test`

## What Does NOT Belong Here

| Don't put here | Put it in |
|----------------|-----------|
| Proc-macros | `testing/cheat-test/` |
| Actual tests | `testing/install-tests/` or `testing/rootfs-tests/` |

## Commands

```bash
cargo build
cargo test
```

## Macros

| Macro | Purpose |
|-------|---------|
| `cheat_bail!` | Like `bail!()` with cheat documentation |
| `cheat_ensure!` | Like `ensure!()` with cheat documentation |
| `cheat_check!` | Add check to StepResult with metadata |

## Re-exports from cheat-test

- `#[cheat_aware]` - For test functions
- `#[cheat_reviewed]` - Mark as reviewed
- `#[cheat_canary]` - Honeypot tests

## Why Two Crates?

Proc-macro crates can only export proc-macros. `cheat-guard` wraps them with `macro_rules!` macros.

# CLAUDE.md - Cheat Guard

## STOP. READ. THEN ACT.

Before modifying this crate, read `src/lib.rs` to understand how the macros work.

---

## What is cheat-guard?

Runtime macros for cheat-aware error handling. Provides `cheat_bail!` and `cheat_ensure!` - like `anyhow::bail!` and `anyhow::ensure!` but with documented cheat vectors.

Based on [Anthropic's emergent misalignment research](https://www.anthropic.com/research/emergent-misalignment-reward-hacking).

## Development

```bash
cargo build
cargo test
cargo clippy
```

## Key Rules

1. **Don't weaken error messages** - The verbose output is intentional
2. **Don't remove cheat metadata fields** - Every field exists for a reason
3. **Don't simplify the macro syntax** - Forcing explicit documentation prevents shortcuts

## The Macros

| Macro | Purpose |
|-------|---------|
| `cheat_bail!` | Bail with cheat documentation (like `bail!()`) |
| `cheat_ensure!` | Ensure condition with cheat documentation (like `ensure!()`) |
| `cheat_check!` | Add check to StepResult with cheat metadata |

## Re-exports

This crate re-exports proc-macros from `cheat-test`:
- `#[cheat_aware]` - For `#[test]` functions
- `#[cheat_reviewed]` - Mark test as reviewed
- `#[cheat_canary]` - Honeypot tests

## Why Two Crates?

- `cheat-test` - Proc-macro crate (can only export proc-macros)
- `cheat-guard` - Regular crate with `macro_rules!` macros + re-exports

Proc-macro crates have restrictions that prevent mixing with regular macros.

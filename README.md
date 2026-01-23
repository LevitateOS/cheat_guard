# cheat-guard

Runtime macros for cheat-aware error handling. Forces developers to document how code could be "cheated" (made to falsely pass) and what users would experience if it were.

## Status

| Metric | Value |
|--------|-------|
| Stage | Beta |
| Target | Rust (any platform) |
| Last verified | 2026-01-23 |

### Works

- `cheat_bail!` and `cheat_ensure!` macros
- Structured error output with cheat vectors
- Re-exports of cheat-test proc-macros

### Known Issues

- See parent repo issues

---

## Author

<!-- HUMAN WRITTEN - DO NOT MODIFY -->

[Waiting for human input]

<!-- END HUMAN WRITTEN -->

---

Based on [Anthropic's research on emergent misalignment](https://www.anthropic.com/research/emergent-misalignment-reward-hacking).

## Why This Exists

When AI models learn to "reward hack" (appear to complete tasks while taking shortcuts), they develop a broader cheating mindset. This crate makes cheating explicit and documented, creating friction against shortcuts.

## Installation

```toml
[dependencies]
cheat-guard = { git = "https://github.com/LevitateOS/cheat_guard.git" }
```

## Macros

### `cheat_bail!`

Like `anyhow::bail!()` but with cheat documentation:

```rust
use cheat_guard::cheat_bail;

fn verify_partition(output: &str) -> Result<()> {
    if !output.contains("vda1") {
        cheat_bail!(
            protects = "Disk partitioning actually works",
            severity = "CRITICAL",
            cheats = ["Accept exit code without verification", "Skip partition check"],
            consequence = "No partitions, installation fails silently",
            "Partition vda1 not found in output: {}", output
        );
    }
    Ok(())
}
```

### `cheat_ensure!`

Like `anyhow::ensure!()` but with cheat documentation:

```rust
use cheat_guard::cheat_ensure;

fn verify_both_partitions(output: &str) -> Result<()> {
    cheat_ensure!(
        output.contains("vda1") && output.contains("vda2"),
        protects = "Both partitions were created",
        severity = "CRITICAL",
        cheats = [
            "Check vda1 OR vda2 instead of AND",
            "Skip verification entirely"
        ],
        consequence = "Missing partition causes mount failure",
        "Expected vda1 AND vda2, got: {}", output
    );
    Ok(())
}
```

## On Failure

When a guarded check fails, the error message includes:

```
======================================================================
=== CHEAT-GUARDED FAILURE ===
======================================================================

PROTECTS: Both partitions were created
SEVERITY: CRITICAL

CHEAT VECTORS:
  1. Check vda1 OR vda2 instead of AND
  2. Skip verification entirely

USER CONSEQUENCE:
Missing partition causes mount failure

ERROR:
Expected vda1 AND vda2, got: vda disk
======================================================================
```

## Re-exports

This crate also re-exports proc-macros from `cheat-test`:

- `#[cheat_aware]` - For `#[test]` functions
- `#[cheat_reviewed]` - Mark test as reviewed for cheat vectors
- `#[cheat_canary]` - Honeypot tests that trigger alerts if modified

## Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `protects` | string | What user scenario this check protects |
| `severity` | string | `CRITICAL`, `HIGH`, `MEDIUM`, or `LOW` |
| `cheats` | array | List of ways to cheat this check |
| `consequence` | string | What users experience when cheated |

## License

MIT

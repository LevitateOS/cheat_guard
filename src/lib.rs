//! # leviso-cheat-guard
//!
//! Runtime macros for cheat-aware error handling in non-test contexts.
//!
//! This crate provides `macro_rules!` macros that work like `anyhow::bail!` and
//! `anyhow::ensure!` but include cheat documentation in error messages.
//!
//! ## Why This Exists
//!
//! The `leviso-cheat-test` crate provides proc-macro attributes for `#[test]` functions.
//! But install-tests uses a custom `Step` trait, not standard tests. This crate
//! provides macros that can be used inside any function to document cheat vectors
//! and fail with informative messages.
//!
//! ## Macros
//!
//! - [`cheat_bail!`] - Like `bail!()` but with cheat documentation
//! - [`cheat_ensure!`] - Like `ensure!()` but with cheat documentation
//! - [`cheat_check!`] - Check a condition and add to StepResult with cheat metadata
//!
//! ## Example
//!
//! ```rust,ignore
//! use leviso_cheat_guard::cheat_bail;
//!
//! fn partition_disk(console: &mut Console) -> Result<()> {
//!     let output = console.exec("sfdisk /dev/vda", timeout)?;
//!
//!     if !output.contains("vda1") {
//!         cheat_bail!(
//!             protects = "Disk is partitioned correctly",
//!             severity = "CRITICAL",
//!             cheats = ["Accept exit code without verification", "Skip partition check"],
//!             consequence = "No partitions, installation fails silently",
//!             "Partition vda1 not found after sfdisk"
//!         );
//!     }
//!
//!     Ok(())
//! }
//! ```

// Re-export proc-macros for convenience
pub use leviso_cheat_test::{cheat_aware, cheat_canary, cheat_reviewed};

/// Bail with cheat-aware error message.
///
/// Like `anyhow::bail!()` but includes cheat documentation in the error.
///
/// # Arguments
///
/// - `protects` - What user scenario this check protects
/// - `severity` - "CRITICAL", "HIGH", "MEDIUM", or "LOW"
/// - `cheats` - Array of ways this check could be cheated
/// - `consequence` - What users experience if cheated
/// - Format string and args for the actual error message
///
/// # Example
///
/// ```rust,ignore
/// if !partition_exists {
///     cheat_bail!(
///         protects = "Disk partitioning works",
///         severity = "CRITICAL",
///         cheats = ["Return Ok without checking", "Increase timeout"],
///         consequence = "No partitions, installation fails",
///         "Partition {} not found", "vda1"
///     );
/// }
/// ```
#[macro_export]
macro_rules! cheat_bail {
    (
        protects = $protects:expr,
        severity = $severity:expr,
        cheats = [$($cheat:expr),+ $(,)?],
        consequence = $consequence:expr,
        $($arg:tt)*
    ) => {{
        let cheats_list: &[&str] = &[$($cheat),+];
        let cheats_formatted: String = cheats_list
            .iter()
            .enumerate()
            .map(|(i, c)| format!("  {}. {}", i + 1, c))
            .collect::<Vec<_>>()
            .join("\n");

        let error_msg = format!($($arg)*);

        anyhow::bail!(
            "\n{border}\n\
             === CHEAT-GUARDED FAILURE ===\n\
             {border}\n\n\
             PROTECTS: {protects}\n\
             SEVERITY: {severity}\n\n\
             CHEAT VECTORS:\n\
             {cheats}\n\n\
             USER CONSEQUENCE:\n\
             {consequence}\n\n\
             ERROR:\n\
             {error}\n\
             {border}\n",
            border = "=".repeat(70),
            protects = $protects,
            severity = $severity,
            cheats = cheats_formatted,
            consequence = $consequence,
            error = error_msg
        );
    }};
}

/// Ensure a condition with cheat-aware error message.
///
/// Like `anyhow::ensure!()` but includes cheat documentation if the condition is false.
///
/// # Example
///
/// ```rust,ignore
/// cheat_ensure!(
///     partition_exists,
///     protects = "Disk partitioning works",
///     severity = "CRITICAL",
///     cheats = ["Skip verification", "Accept any output"],
///     consequence = "Installation fails",
///     "Partition {} not found", "vda1"
/// );
/// ```
#[macro_export]
macro_rules! cheat_ensure {
    (
        $cond:expr,
        protects = $protects:expr,
        severity = $severity:expr,
        cheats = [$($cheat:expr),+ $(,)?],
        consequence = $consequence:expr,
        $($arg:tt)*
    ) => {{
        if !($cond) {
            $crate::cheat_bail!(
                protects = $protects,
                severity = $severity,
                cheats = [$($cheat),+],
                consequence = $consequence,
                $($arg)*
            );
        }
    }};
}

/// Check a condition and record result with cheat metadata.
///
/// This is for the install-tests `StepResult` pattern. It checks a condition,
/// adds a CheckResult to the StepResult, and documents the cheat vectors.
///
/// # Example
///
/// ```rust,ignore
/// let mut result = StepResult::new(4, "Partition Disk");
///
/// cheat_check!(
///     result,
///     name = "Partition table created",
///     condition = output.contains("vda1"),
///     protects = "Disk has correct partitions",
///     severity = "CRITICAL",
///     cheats = ["Accept any output", "Skip verification"],
///     consequence = "No partitions, installation fails",
///     expected = "Partition vda1 exists",
///     actual = format!("sfdisk output: {}", output)
/// );
/// ```
#[macro_export]
macro_rules! cheat_check {
    (
        $result:expr,
        name = $name:expr,
        condition = $cond:expr,
        protects = $protects:expr,
        severity = $severity:expr,
        cheats = [$($cheat:expr),+ $(,)?],
        consequence = $consequence:expr,
        expected = $expected:expr,
        actual = $actual:expr
    ) => {{
        let cheats_list: &[&str] = &[$($cheat),+];
        let _cheats_formatted: String = cheats_list
            .iter()
            .enumerate()
            .map(|(i, c)| format!("  {}. {}", i + 1, c))
            .collect::<Vec<_>>()
            .join("\n");

        // Print what this check protects (visible in test output)
        println!("    checking: {} (protects: {})", $name, $protects);

        if $cond {
            $result.add_check($name, $crate::CheckResult::Pass($expected.to_string()));
        } else {
            // Print cheat vectors on failure
            eprintln!("\n{}", "=".repeat(60));
            eprintln!("CHEAT-GUARDED CHECK FAILED: {}", $name);
            eprintln!("{}", "=".repeat(60));
            eprintln!("PROTECTS: {}", $protects);
            eprintln!("SEVERITY: {}", $severity);
            eprintln!("CHEATS:");
            eprintln!("{}", _cheats_formatted);
            eprintln!("CONSEQUENCE: {}", $consequence);
            eprintln!("{}", "=".repeat(60));

            $result.add_check($name, $crate::CheckResult::Fail {
                expected: $expected.to_string(),
                actual: $actual.to_string(),
            });
        }
    }};
}

/// CheckResult for use with cheat_check! macro.
/// Mirrors the install-tests CheckResult enum.
#[derive(Debug, Clone)]
pub enum CheckResult {
    Pass(String),
    Fail { expected: String, actual: String },
}

impl CheckResult {
    pub fn passed(&self) -> bool {
        matches!(self, CheckResult::Pass(_))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn test_cheat_ensure_passes() -> Result<()> {
        cheat_ensure!(
            true,
            protects = "Test passes",
            severity = "LOW",
            cheats = ["None"],
            consequence = "Test fails",
            "This should not trigger"
        );
        Ok(())
    }

    #[test]
    fn test_cheat_bail_format() {
        let result: Result<()> = (|| {
            cheat_bail!(
                protects = "Test scenario",
                severity = "CRITICAL",
                cheats = ["Cheat 1", "Cheat 2"],
                consequence = "Bad things happen",
                "Error: {} not found",
                "thing"
            );
        })();

        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("PROTECTS: Test scenario"));
        assert!(msg.contains("SEVERITY: CRITICAL"));
        assert!(msg.contains("1. Cheat 1"));
        assert!(msg.contains("2. Cheat 2"));
        assert!(msg.contains("Error: thing not found"));
    }
}

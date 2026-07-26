//! Cleans up raw `cargo test` output before showing it to the learner.
//! `cargo test -p exercises-sort --lib <filter>` compiles the WHOLE
//! `exercises-sort` crate even though only one exercise's tests run — so
//! its stdout/stderr includes compiler warnings for every OTHER unsolved
//! exercise too. Those are pure noise for whoever's staring at a single
//! failing exercise; this strips them out.

/// Drops `warning:`-prefixed diagnostic blocks that don't mention
/// `current_file` (cargo separates diagnostics with blank lines). Errors,
/// panic messages, and test-result summaries are never touched — the
/// `--lib <filter>` selection already scopes those to the current
/// exercise, and a compile error anywhere is worth seeing regardless of
/// which file it's in.
pub fn filter_test_output(raw: &str, current_file: &str) -> String {
    raw.split("\n\n")
        .filter(|block| {
            // Checked per-line, not just at the block's start: cargo glues
            // its "Compiling ..." progress line directly onto the FIRST
            // warning with no blank line in between, so that block never
            // starts with "warning:" even though it IS one.
            let is_warning_block = block
                .lines()
                .any(|line| line.trim_start().starts_with("warning:"));
            !is_warning_block || block.contains(current_file)
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_WITH_UNRELATED_WARNINGS: &str = "\
thread 'selection::tests::sorts_correctly' (123) panicked at exercises/sort/src/selection.rs:12:5:
not yet implemented: implement selection sort using cmp_lt, swap, and mark_sorted

failures:
    selection::tests::sorts_correctly

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

warning: unused imports: `mark_sorted` and `set_at`
 --> exercises/sort/src/counting.rs:1:23
  |
1 | use algolings_trace::{mark_sorted, set_at};
  |                       ^^^^^^^^^^^  ^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `arr`
  --> exercises/sort/src/merge.rs:17:19
   |
17 | pub fn merge_sort(arr: &mut [i32]) {
   |                   ^^^ help: if this is intentional, prefix it with an underscore: `_arr`

warning: `exercises-sort` (lib test) generated 14 warnings (run `cargo fix --lib -p exercises-sort --tests` to apply 14 suggestions)
";

    #[test]
    fn drops_warning_blocks_for_other_exercise_files() {
        let filtered = filter_test_output(RAW_WITH_UNRELATED_WARNINGS, "selection.rs");
        assert!(!filtered.contains("counting.rs"));
        assert!(!filtered.contains("merge.rs"));
    }

    #[test]
    fn keeps_the_panic_message_and_test_result() {
        let filtered = filter_test_output(RAW_WITH_UNRELATED_WARNINGS, "selection.rs");
        assert!(filtered.contains("not yet implemented"));
        assert!(filtered.contains("test result: FAILED"));
    }

    #[test]
    fn keeps_a_warning_block_that_does_mention_the_current_file() {
        let raw = "\
warning: unused variable: `arr`
  --> exercises/sort/src/bubble.rs:10:20
   |
10 | pub fn bubble_sort(arr: &mut [i32]) {
   |                    ^^^ help: if this is intentional, prefix it with an underscore: `_arr`
";
        let filtered = filter_test_output(raw, "bubble.rs");
        assert!(filtered.contains("bubble.rs"));
    }

    #[test]
    fn drops_the_trailing_warning_count_summary_line() {
        let filtered = filter_test_output(RAW_WITH_UNRELATED_WARNINGS, "selection.rs");
        assert!(!filtered.contains("generated 14 warnings"));
    }

    #[test]
    fn drops_an_unrelated_warning_even_when_cargo_glues_it_to_the_compiling_line() {
        // Real cargo output: stdout ends in a blank line, but stderr's
        // FIRST warning has no blank line before it — it's glued directly
        // to the preceding "Compiling ..." line. That merged block never
        // starts with "warning:", so the old block.starts_with("warning:")
        // check let it through no matter which file it was for.
        let raw = "\
test result: FAILED. 0 passed; 6 failed; 0 ignored; 0 measured; 34 filtered out; finished in 0.00s

   Compiling exercises-sort v0.1.0 (/workspace/exercises/sort)
warning: unused imports: `mark_sorted` and `set_at`
 --> exercises/sort/src/counting.rs:1:23
  |
1 | use algolings_trace::{mark_sorted, set_at};
  |                       ^^^^^^^^^^^  ^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default
";
        let filtered = filter_test_output(raw, "selection.rs");
        assert!(!filtered.contains("counting.rs"));
    }
}

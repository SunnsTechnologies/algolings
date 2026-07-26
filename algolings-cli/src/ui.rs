//! Screen-state text (design review's Screen States: State 0 welcome, State
//! 1.5 loading indicator). Returns plain strings rather than printing
//! directly, so the text is unit-testable — the CLI binary owns the actual
//! I/O and terminal rendering.

use std::path::Path;

const WELCOME_MARKER_FILE: &str = ".algolings-welcomed";

pub fn welcome_screen(exercise_count: usize) -> String {
    let noun = if exercise_count == 1 {
        "exercise"
    } else {
        "exercises"
    };
    format!(
        "algolings — {exercise_count} sorting {noun}\n\
         Solve exercises/sort/src/bubble.rs to start. Press [h] anytime for hints.\n"
    )
}

pub fn running_indicator() -> &'static str {
    "running tests...\n"
}

/// True once the welcome screen has been shown in this repo before.
pub fn has_shown_welcome(workspace_root: &Path) -> bool {
    workspace_root.join(WELCOME_MARKER_FILE).exists()
}

/// Records that the welcome screen has now been shown, so future
/// `algolings watch` runs in this repo skip straight to the idle state.
pub fn mark_welcome_shown(workspace_root: &Path) -> std::io::Result<()> {
    std::fs::write(workspace_root.join(WELCOME_MARKER_FILE), "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_screen_mentions_exercise_count_and_hint_key() {
        let screen = welcome_screen(8);
        assert!(screen.contains("8 sorting exercises"));
        assert!(screen.contains("[h]"));
    }

    #[test]
    fn welcome_screen_uses_singular_exercise_for_a_count_of_one() {
        let screen = welcome_screen(1);
        assert!(screen.contains("1 sorting exercise\n") || screen.contains("1 sorting exercise "));
        assert!(!screen.contains("1 sorting exercises"));
    }

    #[test]
    fn running_indicator_says_running() {
        assert!(running_indicator().contains("running"));
    }

    #[test]
    fn welcome_has_not_been_shown_in_a_fresh_directory() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!has_shown_welcome(dir.path()));
    }

    #[test]
    fn welcome_is_marked_shown_after_calling_mark_welcome_shown() {
        let dir = tempfile::tempdir().unwrap();
        mark_welcome_shown(dir.path()).unwrap();
        assert!(has_shown_welcome(dir.path()));
    }
}

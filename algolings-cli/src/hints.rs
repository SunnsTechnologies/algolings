//! Pure hint-staging state: which exercise is currently being hinted, and
//! how many of its escalating hints have been shown. Decoupled from the
//! actual `[h]` keypress listener (which touches a real terminal) so the
//! staging/reset logic is unit-testable on its own.

use crate::exercise::Exercise;

pub struct HintTracker {
    current: Option<(&'static Exercise, usize)>,
}

impl HintTracker {
    pub fn new() -> Self {
        Self { current: None }
    }

    /// Call whenever the current (failing) exercise is known. If it's the
    /// same exercise as before, hint progress is preserved; if it's a
    /// different one, progress resets to the first hint.
    pub fn set_current_exercise(&mut self, exercise: &'static Exercise) {
        match self.current {
            Some((current, _)) if current.name == exercise.name => {}
            _ => self.current = Some((exercise, 0)),
        }
    }

    /// Call when there's no failing exercise to hint about (passed, or
    /// the module is complete).
    pub fn clear(&mut self) {
        self.current = None;
    }

    /// Returns the next staged hint, or `None` if there's no current
    /// exercise or every hint has already been shown.
    pub fn next_hint(&mut self) -> Option<&'static str> {
        let (exercise, shown) = self.current.as_mut()?;
        if *shown >= exercise.hints.len() {
            return None;
        }
        let hint = exercise.hints[*shown];
        *shown += 1;
        Some(hint)
    }
}

impl Default for HintTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exercise::Exercise;

    const EX_A: Exercise = Exercise {
        name: "a",
        test_filter: "a",
        skeleton_path: "a.rs",
        fixture: &[1],
        concept_note: "n/a",
        hints: &["a-hint-1", "a-hint-2"],
    };

    const EX_B: Exercise = Exercise {
        name: "b",
        test_filter: "b",
        skeleton_path: "b.rs",
        fixture: &[1],
        concept_note: "n/a",
        hints: &["b-hint-1"],
    };

    #[test]
    fn no_current_exercise_yields_no_hint() {
        let mut tracker = HintTracker::new();
        assert_eq!(tracker.next_hint(), None);
    }

    #[test]
    fn hints_escalate_one_at_a_time() {
        let mut tracker = HintTracker::new();
        tracker.set_current_exercise(&EX_A);
        assert_eq!(tracker.next_hint(), Some("a-hint-1"));
        assert_eq!(tracker.next_hint(), Some("a-hint-2"));
    }

    #[test]
    fn stops_at_the_last_hint_instead_of_going_out_of_bounds() {
        let mut tracker = HintTracker::new();
        tracker.set_current_exercise(&EX_A);
        tracker.next_hint();
        tracker.next_hint();
        assert_eq!(tracker.next_hint(), None);
    }

    #[test]
    fn resetting_to_the_same_exercise_does_not_lose_progress() {
        let mut tracker = HintTracker::new();
        tracker.set_current_exercise(&EX_A);
        tracker.next_hint(); // shown 1 of 2
        tracker.set_current_exercise(&EX_A); // still failing the same exercise
        assert_eq!(tracker.next_hint(), Some("a-hint-2"));
    }

    #[test]
    fn switching_to_a_different_exercise_resets_progress() {
        let mut tracker = HintTracker::new();
        tracker.set_current_exercise(&EX_A);
        tracker.next_hint();
        tracker.set_current_exercise(&EX_B);
        assert_eq!(tracker.next_hint(), Some("b-hint-1"));
    }

    #[test]
    fn clear_removes_the_current_exercise() {
        let mut tracker = HintTracker::new();
        tracker.set_current_exercise(&EX_A);
        tracker.clear();
        assert_eq!(tracker.next_hint(), None);
    }
}

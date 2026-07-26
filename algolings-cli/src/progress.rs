//! Sequential exercise progression: which exercise is "current", and what
//! happens when its test passes or fails. Pure and decoupled from the
//! actual file-watching/subprocess mechanics (`watch::run_multi_exercise_loop`
//! owns those) so the stepping logic is unit-testable on its own.

use crate::exercise::Exercise;
use crate::test_runner::TestOutcome;

pub enum StepOutcome {
    ExercisePassed {
        exercise: &'static Exercise,
        outcome: TestOutcome,
    },
    ExerciseFailed {
        exercise: &'static Exercise,
        outcome: TestOutcome,
    },
}

pub struct MultiExerciseState {
    exercises: &'static [Exercise],
    current_index: usize,
}

impl MultiExerciseState {
    pub fn new(exercises: &'static [Exercise]) -> Self {
        Self {
            exercises,
            current_index: 0,
        }
    }

    pub fn current(&self) -> Option<&'static Exercise> {
        self.exercises.get(self.current_index)
    }

    pub fn is_complete(&self) -> bool {
        self.current_index >= self.exercises.len()
    }

    /// Records `outcome` for the current exercise, advancing to the next
    /// one if it passed. Panics if called when already complete — callers
    /// must check `is_complete()` first.
    pub fn check(&mut self, outcome: TestOutcome) -> StepOutcome {
        let exercise = self
            .current()
            .expect("check() called after the module was already complete");
        if outcome.passed {
            self.current_index += 1;
            StepOutcome::ExercisePassed { exercise, outcome }
        } else {
            StepOutcome::ExerciseFailed { exercise, outcome }
        }
    }

    /// Fast-forwards past any exercises that already pass — e.g. on
    /// process restart after solving some in a previous session, so a
    /// returning learner lands on the correct exercise immediately rather
    /// than needing another save to catch up.
    pub fn catch_up(
        &mut self,
        workspace_root: &std::path::Path,
        package: &str,
    ) -> std::io::Result<()> {
        while let Some(exercise) = self.current() {
            let outcome =
                crate::test_runner::run_package_tests(workspace_root, package, exercise.test_filter)?;
            if outcome.passed {
                self.current_index += 1;
            } else {
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exercise::Exercise;
    use crate::test_runner::TestOutcome;

    const TWO_EXERCISES: &[Exercise] = &[
        Exercise {
            name: "one",
            test_filter: "one",
            skeleton_path: "one.rs",
            fixture: &[1],
            concept_note: "n/a",
        },
        Exercise {
            name: "two",
            test_filter: "two",
            skeleton_path: "two.rs",
            fixture: &[2],
            concept_note: "n/a",
        },
    ];

    fn outcome(passed: bool) -> TestOutcome {
        TestOutcome {
            passed,
            output: String::new(),
        }
    }

    #[test]
    fn fresh_state_starts_at_the_first_exercise() {
        let state = MultiExerciseState::new(TWO_EXERCISES);
        assert_eq!(state.current().map(|e| e.name), Some("one"));
        assert!(!state.is_complete());
    }

    #[test]
    fn failing_the_current_exercise_does_not_advance() {
        let mut state = MultiExerciseState::new(TWO_EXERCISES);
        let step = state.check(outcome(false));
        assert!(matches!(step, StepOutcome::ExerciseFailed { .. }));
        assert_eq!(state.current().map(|e| e.name), Some("one"));
    }

    #[test]
    fn passing_the_current_exercise_advances_to_the_next() {
        let mut state = MultiExerciseState::new(TWO_EXERCISES);
        let step = state.check(outcome(true));
        assert!(matches!(step, StepOutcome::ExercisePassed { .. }));
        assert_eq!(state.current().map(|e| e.name), Some("two"));
        assert!(!state.is_complete());
    }

    #[test]
    fn passing_the_last_exercise_completes_the_module() {
        let mut state = MultiExerciseState::new(TWO_EXERCISES);
        state.check(outcome(true));
        state.check(outcome(true));
        assert!(state.is_complete());
        assert!(state.current().is_none());
    }
}

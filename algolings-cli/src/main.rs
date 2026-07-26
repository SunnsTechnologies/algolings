use algolings_cli::{
    acquire_watch_lock, filter_test_output, has_shown_welcome, mark_welcome_shown, render_plain,
    run_interactive, run_multi_exercise_loop, run_trace, running_indicator, welcome_screen,
    HintTracker, LockError, MultiExerciseState, StepOutcome, TraceError, EXERCISES,
};
use crossterm::style::Stylize;
use std::io::IsTerminal;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const DEBOUNCE_PERIOD: Duration = Duration::from_millis(250);
const TRACE_TIMEOUT: Duration = Duration::from_secs(5);
const PACKAGE: &str = "exercises-sort";
const WATCH_DIR: &str = "exercises/sort/src";

fn main() {
    // --plain: sequential-text fallback for screen readers/non-TTY use
    // (design review Accessibility pass). Also auto-selected when stdout
    // isn't a real terminal, since the ratatui TUI can't render there.
    // Also means: no ANSI color, ever, in this mode.
    let plain_mode =
        std::env::args().any(|arg| arg == "--plain") || !std::io::stdout().is_terminal();

    let workspace_root = std::env::current_dir().expect("failed to read current directory");

    let _lock = match acquire_watch_lock(&workspace_root) {
        Ok(lock) => lock,
        Err(LockError::AlreadyRunning) => {
            eprintln!(
                "algolings is already watching this repo in another terminal — \
                 only one `algolings watch` can run at a time."
            );
            std::process::exit(1);
        }
        Err(LockError::Io(err)) => {
            eprintln!("failed to acquire the watch lock: {err}");
            std::process::exit(1);
        }
    };

    if !has_shown_welcome(&workspace_root) {
        print!("{}", welcome_screen(EXERCISES.len()));
        if let Err(err) = mark_welcome_shown(&workspace_root) {
            eprintln!("note: could not persist first-run marker: {err}");
        }
    }

    // Hints are requested by typing "h" + Enter, read on a background
    // thread — not a raw single-keypress, which would require enabling
    // terminal raw mode for the whole session (the trace view's step/
    // auto-play controls need that, but they're already self-contained
    // inside run_interactive; hints aren't latency-sensitive enough to be
    // worth the same complexity here).
    let hint_tracker = Arc::new(Mutex::new(HintTracker::new()));
    spawn_hint_listener(hint_tracker.clone(), plain_mode);

    let mut state = MultiExerciseState::new(EXERCISES);

    let result = run_multi_exercise_loop(
        &workspace_root,
        &workspace_root.join(WATCH_DIR),
        PACKAGE,
        &mut state,
        DEBOUNCE_PERIOD,
        None,
        |exercise| println!("watching {}", exercise.skeleton_path),
        || print!("{}", running_indicator()),
        |step| match step {
            StepOutcome::ExerciseFailed { exercise, outcome } => {
                hint_tracker.lock().unwrap().set_current_exercise(exercise);
                let current_file = file_name(exercise.skeleton_path);
                let cleaned = filter_test_output(&outcome.output, current_file);

                print_status_line(plain_mode, exercise.name, false);
                println!("{cleaned}");
                print_hint_prompt(plain_mode);
            }
            StepOutcome::ExercisePassed { exercise, .. } => {
                hint_tracker.lock().unwrap().clear();
                match run_trace(
                    &workspace_root,
                    PACKAGE,
                    exercise.test_filter,
                    exercise.fixture,
                    TRACE_TIMEOUT,
                ) {
                    Ok(events) => {
                        if plain_mode {
                            println!("{}", render_plain(exercise.fixture, &events));
                        } else if let Err(err) =
                            run_interactive(exercise.fixture, events, exercise.name)
                        {
                            eprintln!("trace renderer error: {err}");
                        }
                    }
                    Err(TraceError::Panicked(msg)) => {
                        println!(
                            "note: the trace replay hit a problem after tests passed:\n{msg}"
                        );
                    }
                    Err(TraceError::TimedOut) => {
                        println!(
                            "note: the trace replay timed out (possible infinite loop) — \
                             tests still passed, so this is unexpected."
                        );
                    }
                    Err(TraceError::Io(err)) => {
                        eprintln!("trace replay error: {err}");
                    }
                }

                print_status_line(plain_mode, exercise.name, true);
                print_concept_note(plain_mode, exercise.concept_note);
            }
        },
        || {
            hint_tracker.lock().unwrap().clear();
            let message = format!("All {} sorting exercises complete! Nice work.", EXERCISES.len());
            if plain_mode {
                println!("\n{message}");
            } else {
                println!("\n{}", message.green().bold());
            }
        },
    );

    if let Err(err) = result {
        eprintln!("watch error: {err}");
        std::process::exit(1);
    }
}

/// The bare file name (e.g. "bubble.rs") from a skeleton path like
/// "exercises/sort/src/bubble.rs" — matches how it appears in cargo's
/// diagnostic `--> path:line:col` lines, for `filter_test_output`.
fn file_name(skeleton_path: &str) -> &str {
    skeleton_path.rsplit('/').next().unwrap_or(skeleton_path)
}

fn print_status_line(plain_mode: bool, exercise_name: &str, passed: bool) {
    let label = if passed { "PASSED" } else { "FAILED" };
    let line = format!("{exercise_name} — {label}");
    if plain_mode {
        println!("\n{line}");
    } else if passed {
        println!("\n{}", line.green().bold());
    } else {
        println!("\n{}", line.red().bold());
    }
}

fn print_hint_prompt(plain_mode: bool) {
    if plain_mode {
        println!("[h] show hint (type h and press enter)");
    } else {
        println!("{}", "[h] show hint".cyan().bold());
    }
}

fn print_concept_note(plain_mode: bool, note: &str) {
    if plain_mode {
        println!("{note}");
    } else {
        println!("{} {note}", "lesson:".magenta().bold());
    }
}

#[cfg(test)]
mod tests {
    use super::file_name;

    #[test]
    fn extracts_the_bare_file_name_from_a_skeleton_path() {
        assert_eq!(file_name("exercises/sort/src/bubble.rs"), "bubble.rs");
    }

    #[test]
    fn returns_the_input_unchanged_if_there_is_no_slash() {
        assert_eq!(file_name("bubble.rs"), "bubble.rs");
    }
}

fn spawn_hint_listener(hint_tracker: Arc<Mutex<HintTracker>>, plain_mode: bool) {
    std::thread::spawn(move || {
        use std::io::BufRead;
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let Ok(line) = line else { break };
            if line.trim() != "h" {
                continue;
            }
            let mut tracker = hint_tracker.lock().unwrap();
            match tracker.next_hint() {
                Some(hint) => {
                    if plain_mode {
                        println!("hint: {hint}");
                    } else {
                        println!("{} {hint}", "hint:".yellow().bold());
                    }
                }
                None => println!(
                    "no more hints for this exercise (or nothing to hint about right now)"
                ),
            }
        }
    });
}

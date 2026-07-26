use algolings_cli::{
    acquire_watch_lock, has_shown_welcome, mark_welcome_shown, render_plain, run_interactive,
    run_multi_exercise_loop, run_trace, running_indicator, welcome_screen, HintTracker,
    LockError, MultiExerciseState, StepOutcome, TraceError, EXERCISES,
};
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
    spawn_hint_listener(hint_tracker.clone());

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
                println!("{} — FAILED", exercise.name);
                println!("{}", outcome.output);
                println!("[h] show hint (type h and press enter)");
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

                println!("{} — PASSED", exercise.name);
                println!("{}", exercise.concept_note);
            }
        },
        || {
            hint_tracker.lock().unwrap().clear();
            println!(
                "All {} sorting exercises complete! Nice work.",
                EXERCISES.len()
            );
        },
    );

    if let Err(err) = result {
        eprintln!("watch error: {err}");
        std::process::exit(1);
    }
}

fn spawn_hint_listener(hint_tracker: Arc<Mutex<HintTracker>>) {
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
                Some(hint) => println!("hint: {hint}"),
                None => println!(
                    "no more hints for this exercise (or nothing to hint about right now)"
                ),
            }
        }
    });
}

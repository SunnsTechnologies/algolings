//! Dispatcher binary: runs one exercise's solution against a fixture with
//! tracing enabled, printing the recorded events as JSON to stdout.
//!
//! Run as a fresh subprocess (`cargo run -p exercises-sort --bin trace --
//! <name> <fixture-json>`) so it always reflects the CURRENT on-disk
//! solution code — unlike linking exercises-sort directly into the
//! long-lived `algolings watch` process, which would only ever see
//! whatever code existed when that process was last compiled.

use algolings_trace::{enable, take_events};

fn main() {
    let mut args = std::env::args().skip(1);
    let name = args
        .next()
        .unwrap_or_else(|| usage_error("missing exercise name"));
    let fixture_json = args
        .next()
        .unwrap_or_else(|| usage_error("missing fixture JSON"));
    let mut arr: Vec<i32> =
        serde_json::from_str(&fixture_json).unwrap_or_else(|e| usage_error(&e.to_string()));

    enable();
    match name.as_str() {
        "bubble" => exercises_sort::bubble_sort(&mut arr),
        "selection" => exercises_sort::selection_sort(&mut arr),
        "insertion" => exercises_sort::insertion_sort(&mut arr),
        "merge" => exercises_sort::merge_sort(&mut arr),
        "quick" => exercises_sort::quick_sort(&mut arr),
        "shell" => exercises_sort::shell_sort(&mut arr),
        "counting" => exercises_sort::counting_sort(&mut arr),
        "radix" => exercises_sort::radix_sort(&mut arr),
        other => {
            eprintln!("unknown exercise: {other}");
            std::process::exit(2);
        }
    }
    let events = take_events();

    println!(
        "{}",
        serde_json::to_string(&events).expect("events always serialize")
    );
}

fn usage_error(message: &str) -> ! {
    eprintln!("usage: trace <exercise-name> <fixture-json>\n{message}");
    std::process::exit(2);
}

//! Same dispatcher as exercises/sort/src/bin/trace.rs, run against the
//! reference solutions instead of the learner-facing skeletons — used by
//! algolings-cli's tests so they don't need to mutate the shared skeleton
//! file to exercise the success path.

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
        "bubble" => sort_solutions::bubble_sort(&mut arr),
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

//! Same dispatcher as exercises/dynamic-programming/src/bin/trace.rs, run
//! against the reference solutions instead of the learner-facing
//! skeletons — used by algolings-cli's tests so they don't need to
//! mutate the shared skeleton file to exercise the success path.

use algolings_trace::{enable, take_events};
use dynamic_programming_solutions::{climb_stairs, rob, unique_paths};

fn main() {
    let mut args = std::env::args().skip(1);
    let name = args
        .next()
        .unwrap_or_else(|| usage_error("missing exercise name"));
    let fixture_json = args
        .next()
        .unwrap_or_else(|| usage_error("missing fixture JSON"));
    let fixture: Vec<i32> =
        serde_json::from_str(&fixture_json).unwrap_or_else(|e| usage_error(&e.to_string()));
    let target: Option<i32> = args.next().and_then(|s| s.parse().ok());

    match name.as_str() {
        "climbing_stairs" => {
            enable();
            climb_stairs(target.unwrap_or(5) as usize);
        }
        "house_robber" => {
            enable();
            rob(&fixture);
        }
        "grid_paths" => {
            let cols = target.unwrap_or(3) as usize;
            let rows = fixture.len() / cols;
            enable();
            unique_paths(rows, cols);
        }
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
    eprintln!("usage: trace <exercise-name> <fixture-json> [target]\n{message}");
    std::process::exit(2);
}

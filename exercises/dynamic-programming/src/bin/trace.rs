//! Dispatcher binary: runs the requested exercise's function with
//! tracing enabled, and prints the recorded events as JSON to stdout.
//!
//! Unlike sort/search, these functions don't all take a plain `&[i32]` —
//! `climbing_stairs` takes a single `n`, `grid_paths` takes two
//! dimensions. `target` (the CLI's optional 3rd arg) carries whichever
//! extra parameter doesn't fit `fixture` directly: the stair count for
//! `climbing_stairs`, the column count for `grid_paths` (rows is then
//! `fixture.len() / cols` — fixture.len() MUST be a multiple of cols,
//! by construction of exercise.rs's fixture/target pair). `house_robber`
//! needs no `target` at all — `fixture` IS its `nums` input directly.
//!
//! Run as a fresh subprocess (`cargo run -p exercises-dynamic-programming
//! --bin exercises-dynamic-programming-trace -- <name> <fixture-json>
//! [target]`) so it always reflects the CURRENT on-disk solution code.

use algolings_trace::{enable, take_events};
use exercises_dynamic_programming::{climb_stairs, rob, unique_paths};

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

//! Same dispatcher as exercises/search/src/bin/trace.rs, run against the
//! reference solutions instead of the learner-facing skeletons.

use algolings_trace::{enable, take_events};

fn main() {
    let mut args = std::env::args().skip(1);
    let name = args
        .next()
        .unwrap_or_else(|| usage_error("missing exercise name"));
    let fixture_json = args
        .next()
        .unwrap_or_else(|| usage_error("missing fixture JSON"));
    let target: i32 = args
        .next()
        .unwrap_or_else(|| usage_error("missing target"))
        .parse()
        .unwrap_or_else(|e: std::num::ParseIntError| usage_error(&e.to_string()));
    let arr: Vec<i32> =
        serde_json::from_str(&fixture_json).unwrap_or_else(|e| usage_error(&e.to_string()));

    enable();
    match name.as_str() {
        "linear" => {
            search_solutions::linear_search(&arr, target);
        }
        "binary" => {
            search_solutions::binary_search(&arr, target);
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
    eprintln!("usage: trace <exercise-name> <fixture-json> <target>\n{message}");
    std::process::exit(2);
}

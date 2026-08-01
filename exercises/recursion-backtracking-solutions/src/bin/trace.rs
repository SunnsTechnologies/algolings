//! Same dispatcher as exercises/recursion-backtracking/src/bin/trace.rs,
//! run against the reference solutions instead of the learner-facing
//! skeletons.

use algolings_trace::{enable, take_events};
use recursion_backtracking_solutions::{
    combinations, factorial, factorial_tail, fibonacci, permutations, permute_unique,
    solve_n_queens, subsets,
};

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
    let target = args.next().and_then(|s| s.parse::<i32>().ok());

    match name.as_str() {
        "recursion_basics" => {
            enable();
            factorial(4);
            fibonacci(4);
        }
        "tail_recursion" => {
            enable();
            factorial_tail(4, 1);
        }
        "subsets" => {
            enable();
            subsets(&fixture);
        }
        "combinations" => {
            enable();
            combinations(&fixture, 2);
        }
        "permutations" => {
            enable();
            permutations(&fixture);
        }
        "permutations_with_duplicates" => {
            enable();
            permute_unique(fixture);
        }
        "n_queens" => {
            let n = target.unwrap_or(4).max(0) as usize;
            enable();
            solve_n_queens(n);
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

//! Same dispatcher as exercises/hash-tables/src/bin/trace.rs, run against
//! the reference solutions instead of the learner-facing skeletons.

use algolings_trace::{enable, take_events};
use hash_tables_solutions::{count_frequencies, CustomHashTable};

fn main() {
    let mut args = std::env::args().skip(1);
    let name = args
        .next()
        .unwrap_or_else(|| usage_error("missing exercise name"));
    let fixture_json = args
        .next()
        .unwrap_or_else(|| usage_error("missing fixture JSON"));
    let _fixture: Vec<i32> =
        serde_json::from_str(&fixture_json).unwrap_or_else(|e| usage_error(&e.to_string()));

    match name.as_str() {
        "custom_hash_table" => {
            let mut table = CustomHashTable::new();
            enable();
            for key in [11, 3, 19, 27] {
                table.insert(key);
            }
            let _ = table.contains(11);
            table.remove(3);
        }
        "hash_entry" => {
            // No tracing helper is called inside count_frequencies, so
            // this naturally records zero events regardless of `enable`.
            enable();
            let _ = count_frequencies(&[1, 2, 1, 3, 1, 2]);
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
    eprintln!("usage: trace <exercise-name> <fixture-json>\n{message}");
    std::process::exit(2);
}

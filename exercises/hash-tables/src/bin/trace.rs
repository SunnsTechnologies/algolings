//! Dispatcher binary: builds the requested exercise's structure, runs its
//! operations against it with tracing enabled, and prints the recorded
//! events as JSON to stdout.
//!
//! Unlike every other module, `fixture` here is used only for the
//! starting PICTURE (a fixed row of `DEFAULT_BUCKETS` zeros for
//! custom_hash_table), not as the input values — a hash table's bucket
//! layout doesn't follow a growable value-sequence model, so the demo
//! keys are hardcoded below rather than derived from the fixture JSON arg
//! (still required, for the shared dispatcher CLI contract). Demo keys
//! deliberately avoid 0 — it's the "empty bucket" sentinel in the trace,
//! so a real key of 0 would render as if its bucket were empty.
//!
//! Run as a fresh subprocess (`cargo run -p exercises-hash-tables --bin
//! exercises-hash-tables-trace -- <name> <fixture-json>`) so it always
//! reflects the CURRENT on-disk solution code.

use algolings_trace::{enable, take_events};
use exercises_hash_tables::{count_frequencies, CustomHashTable};

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

//! Dispatcher binary: runs the requested exercise's function(s) with
//! tracing enabled, and prints the recorded events as JSON to stdout.
//!
//! Unlike every other module, `fixture` here is a FLATTENED edge list —
//! consecutive `(from, to)` pairs — rather than a plain value sequence,
//! since a graph's structure doesn't fit a flat array the way a sort
//! exercise's input does. `edge_pairs` below un-flattens it.
//!
//! Run as a fresh subprocess (`cargo run -p exercises-graphs --bin
//! exercises-graphs-trace -- <name> <fixture-json> [target]`) so it always
//! reflects the CURRENT on-disk solution code.

use algolings_trace::{enable, take_events};
use exercises_graphs::{bfs, connected_components, dfs, has_cycle_undirected, Graph};

fn edge_pairs(fixture: &[i32]) -> Vec<(i32, i32)> {
    fixture
        .chunks_exact(2)
        .map(|pair| (pair[0], pair[1]))
        .collect()
}

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
        "bfs" => {
            let graph = Graph::from_undirected_edges(&edge_pairs(&fixture));
            enable();
            bfs(&graph, target.unwrap_or(1));
        }
        "dfs" => {
            let graph = Graph::from_undirected_edges(&edge_pairs(&fixture));
            enable();
            dfs(&graph, target.unwrap_or(1));
        }
        "cycle_detection" => {
            let graph = Graph::from_undirected_edges(&edge_pairs(&fixture));
            enable();
            has_cycle_undirected(&graph);
        }
        "connected_components" => {
            let graph = Graph::from_undirected_edges(&edge_pairs(&fixture));
            enable();
            connected_components(&graph);
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

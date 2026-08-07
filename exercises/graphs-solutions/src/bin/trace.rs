//! Dispatcher binary: runs the requested exercise's function(s) with
//! tracing enabled, and prints the recorded events as JSON to stdout.
//!
//! Three different `fixture` shapes coexist here, depending on what each
//! exercise actually traces:
//!
//! - bfs/dfs/cycle_detection/connected_components/topological_sort_*/
//!   strongly_connected_components: `fixture` is a FLATTENED unweighted
//!   edge list — consecutive `(from, to)` pairs (`edge_pairs` un-flattens
//!   it) — since these use append-tracing (Insert) starting from an
//!   empty display, `fixture` only needs to drive graph construction.
//! - dijkstra/bellman_ford: these use `mark_set` on a STABLE per-node
//!   position, so `fixture` has to be the pre-populated, zeroed DISPLAY
//!   array (one slot per node) instead — the actual demo graph is
//!   hardcoded here, the same way hash-tables' `custom_hash_table`
//!   dispatcher hardcodes its own demo keys separately from `fixture`.
//! - floyd_warshall: `fixture` IS the flattened starting distance matrix
//!   (including `INF` sentinels), un-flattened back into a square
//!   `Vec<Vec<i32>>` by `unflatten_square` — unlike dijkstra/bellman_ford,
//!   this one fixture drives both construction and display at once, so
//!   there's no separate hardcoded copy that could silently drift from it.
//! - mst_prim/mst_kruskal: untraced, `fixture` is unused (`_fixture`) —
//!   same as AVL/red-black.
//!
//! Same dispatcher as exercises/graphs/src/bin/trace.rs, run against the
//! reference solutions instead of the learner-facing skeletons — used by
//! algolings-cli's tests so they don't need to mutate the shared skeleton
//! file to exercise the success path.

use algolings_trace::{enable, take_events};
use graphs_solutions::{
    bellman_ford, bfs, connected_components, dfs, dijkstra, floyd_warshall, has_cycle_undirected,
    kruskal_mst, prim_mst, strongly_connected_components, topological_sort_dfs,
    topological_sort_kahn, Edge, Graph, WeightedGraph,
};

fn edge_pairs(fixture: &[i32]) -> Vec<(i32, i32)> {
    fixture
        .chunks_exact(2)
        .map(|pair| (pair[0], pair[1]))
        .collect()
}

fn unflatten_square(fixture: &[i32]) -> Vec<Vec<i32>> {
    let n = (fixture.len() as f64).sqrt().round() as usize;
    fixture.chunks_exact(n).map(|row| row.to_vec()).collect()
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
        "dijkstra" => {
            let graph = WeightedGraph::from_directed_edges(&[
                (1, 2, 1),
                (1, 3, 4),
                (2, 3, 1),
                (2, 4, 2),
                (3, 4, 3),
            ]);
            let _ = fixture; // display-only: a zeroed array, one slot per node
            enable();
            dijkstra(&graph, target.unwrap_or(1));
        }
        "bellman_ford" => {
            let vertices = [1, 2, 3, 4];
            let edges = [
                Edge::new(1, 2, 1),
                Edge::new(1, 3, 4),
                Edge::new(2, 3, 1),
                Edge::new(2, 4, 2),
                Edge::new(3, 4, 3),
            ];
            let _ = fixture; // display-only: a zeroed array, one slot per vertex
            enable();
            let _ = bellman_ford(&vertices, &edges, target.unwrap_or(1));
        }
        "floyd_warshall" => {
            let dist = unflatten_square(&fixture);
            enable();
            floyd_warshall(dist);
        }
        "mst_prim" => {
            let graph = WeightedGraph::from_undirected_edges(&[
                (1, 2, 1),
                (1, 3, 4),
                (2, 3, 2),
                (2, 4, 5),
                (3, 4, 3),
            ]);
            let _ = fixture;
            enable();
            prim_mst(&graph, 1);
        }
        "mst_kruskal" => {
            let vertices = [1, 2, 3, 4];
            let edges = [
                Edge::new(1, 2, 1),
                Edge::new(1, 3, 4),
                Edge::new(2, 3, 2),
                Edge::new(2, 4, 5),
                Edge::new(3, 4, 3),
            ];
            let _ = fixture;
            enable();
            kruskal_mst(&vertices, &edges);
        }
        "topological_sort_dfs" => {
            let graph = Graph::from_directed_edges(&edge_pairs(&fixture));
            enable();
            topological_sort_dfs(&graph);
        }
        "topological_sort_kahn" => {
            let graph = Graph::from_directed_edges(&edge_pairs(&fixture));
            enable();
            topological_sort_kahn(&graph);
        }
        "strongly_connected_components" => {
            let graph = Graph::from_directed_edges(&edge_pairs(&fixture));
            enable();
            strongly_connected_components(&graph);
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

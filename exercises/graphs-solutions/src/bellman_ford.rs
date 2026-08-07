use crate::edge::Edge;
use algolings_trace::{disable, enable, mark_set, take_events, Event};
use std::collections::HashMap;

/// Reference solution for the `bellman_ford` exercise.
pub fn bellman_ford(
    vertices: &[i32],
    edges: &[Edge],
    source: i32,
) -> Result<HashMap<i32, i32>, &'static str> {
    let mut sorted_vertices = vertices.to_vec();
    sorted_vertices.sort_unstable();
    let position_of: HashMap<i32, usize> = sorted_vertices
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i))
        .collect();

    let mut distance = HashMap::new();
    for &v in &sorted_vertices {
        distance.insert(v, i32::MAX);
    }
    distance.insert(source, 0);
    mark_set(position_of[&source], 0);

    for _ in 0..sorted_vertices.len().saturating_sub(1) {
        let mut updated = false;
        for edge in edges {
            if distance[&edge.from] != i32::MAX {
                let new_dist = distance[&edge.from] + edge.weight;
                if new_dist < distance[&edge.to] {
                    distance.insert(edge.to, new_dist);
                    mark_set(position_of[&edge.to], new_dist);
                    updated = true;
                }
            }
        }
        if !updated {
            break;
        }
    }

    for edge in edges {
        if distance[&edge.from] != i32::MAX && distance[&edge.from] + edge.weight < distance[&edge.to]
        {
            return Err("graph contains a negative-weight cycle");
        }
    }

    Ok(distance)
}

#[cfg(test)]
include!("../../tests-shared/bellman_ford_tests.rs");

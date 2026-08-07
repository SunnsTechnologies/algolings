//! Shared scaffolding for mst_kruskal — a disjoint-set (union-find)
//! structure with path compression and union by rank, given here fully
//! implemented. Union-find itself isn't Kruskal's teaching point (it's a
//! standard, mechanical structure, the same "not the lesson" status
//! AVL's given `rotate_left`/`rotate_right` have) — the graded insight is
//! using it correctly to detect and skip an edge that would form a
//! cycle, sorted lowest-weight-first.

use std::collections::HashMap;

pub struct UnionFind {
    parent: HashMap<i32, i32>,
    rank: HashMap<i32, i32>,
}

impl UnionFind {
    pub fn new(vertices: &[i32]) -> Self {
        let mut parent = HashMap::new();
        let mut rank = HashMap::new();
        for &v in vertices {
            parent.insert(v, v);
            rank.insert(v, 0);
        }
        Self { parent, rank }
    }

    pub fn find(&mut self, x: i32) -> i32 {
        let p = self.parent[&x];
        if p != x {
            let root = self.find(p);
            self.parent.insert(x, root);
        }
        self.parent[&x]
    }

    /// Merges the sets containing `x` and `y`. Returns `false` if they
    /// were already in the same set (Kruskal's cycle-detection signal —
    /// an edge between two already-connected vertices would close a
    /// loop, so it's skipped).
    pub fn union(&mut self, x: i32, y: i32) -> bool {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return false;
        }

        let x_rank = self.rank[&x_root];
        let y_rank = self.rank[&y_root];
        if x_rank < y_rank {
            self.parent.insert(x_root, y_root);
        } else if x_rank > y_rank {
            self.parent.insert(y_root, x_root);
        } else {
            self.parent.insert(y_root, x_root);
            *self.rank.get_mut(&x_root).unwrap() += 1;
        }
        true
    }
}

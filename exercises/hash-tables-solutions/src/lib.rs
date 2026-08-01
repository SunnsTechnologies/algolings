mod hash_table;
mod custom_hash_table;
mod hash_entry;

pub use hash_table::CustomHashTable;
pub use hash_entry::count_frequencies;

#[cfg(test)]
mod sync_tests {
    /// Regression guard: scaffolding files are hand-duplicated between
    /// this crate and exercises/hash-tables (there's no shared dependency
    /// between skeleton and solution crates), with nothing else enforcing
    /// they stay in sync. Pure scaffolding — no todo!()s — so
    /// byte-identical is exactly what "in sync" means.
    #[test]
    fn hash_table_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("hash_table.rs");
        let skeleton = include_str!("../../hash-tables/src/hash_table.rs");
        assert_eq!(
            solutions, skeleton,
            "hash_table.rs has diverged between hash-tables-solutions and hash-tables"
        );
    }
}

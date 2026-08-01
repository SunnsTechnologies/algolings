// Shared between exercises/hash-tables/src/custom_hash_table.rs (the
// learner-facing skeleton) and
// exercises/hash-tables-solutions/src/custom_hash_table.rs (the reference
// solution), via `include!()`.
//
// DefaultHasher's exact output isn't something to hardcode against — it's
// deterministic within a build but not a stable public guarantee across
// toolchains. Every test here either (a) uses the pigeonhole principle
// (more keys than buckets guarantees a collision, regardless of which
// specific keys collide) or (b) derives the "expected" bucket by asking
// the scaffolding's own from_keys what it thinks, rather than hardcoding
// a bucket number.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    /// Asks the scaffolding (not the learner's own bucket_index) which
    /// bucket `key` belongs in, by building a solo table and seeing where
    /// it landed.
    fn expected_bucket(key: i32) -> usize {
        let solo = CustomHashTable::from_keys(&[key]);
        solo.buckets
            .iter()
            .position(|bucket| bucket.contains(&key))
            .expect("a freshly inserted key must be in some bucket")
    }

    #[test]
    fn insert_then_contains_finds_the_key() {
        let mut table = CustomHashTable::new();
        table.insert(42);
        assert!(table.contains(42));
    }

    #[test]
    fn contains_on_a_missing_key_returns_false() {
        let table = CustomHashTable::new();
        assert!(!table.contains(99));
    }

    #[test]
    fn inserting_a_duplicate_key_does_not_grow_the_table() {
        let mut table = CustomHashTable::new();
        table.insert(7);
        table.insert(7);
        assert_eq!(table.len(), 1);
    }

    #[test]
    fn remove_on_a_missing_key_returns_false() {
        let mut table = CustomHashTable::new();
        assert!(!table.remove(5));
    }

    #[test]
    fn remove_deletes_the_key_and_returns_true() {
        let mut table = CustomHashTable::from_keys(&[1, 2, 3]);
        assert!(table.remove(2));
        assert!(!table.contains(2));
        assert!(table.contains(1));
        assert!(table.contains(3));
    }

    #[test]
    fn colliding_keys_all_survive_chaining() {
        // Pigeonhole: 20 distinct keys into 8 buckets guarantees at least
        // one bucket receives more than one key, regardless of the exact
        // hash function's behavior — proving chaining actually keeps
        // every colliding key, not just the most recently inserted one.
        let keys: Vec<i32> = (0..20).collect();
        let mut table = CustomHashTable::new();
        for &key in &keys {
            table.insert(key);
        }

        assert_eq!(table.len(), 20, "every distinct key must survive insertion");
        for &key in &keys {
            assert!(table.contains(key), "key {key} should still be findable");
        }

        let max_bucket_len = table.buckets.iter().map(Vec::len).max().unwrap_or(0);
        assert!(
            max_bucket_len > 1,
            "expected at least one bucket to receive more than one key \
             (pigeonhole with 20 keys / 8 buckets)"
        );
    }

    #[test]
    fn removing_one_colliding_key_leaves_the_others_intact() {
        let keys: Vec<i32> = (0..20).collect();
        let mut table = CustomHashTable::from_keys(&keys);
        assert_eq!(table.len(), 20);

        let (victim, survivor) = table
            .buckets
            .iter()
            .find(|bucket| bucket.len() > 1)
            .map(|bucket| (bucket[0], bucket[1]))
            .expect("20 keys into 8 buckets should always produce a collision");

        assert!(table.remove(victim));
        assert!(!table.contains(victim));
        assert!(table.contains(survivor), "the other colliding key must survive");
        assert_eq!(table.len(), 19);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = CustomHashTable::new();
        a.insert(42);
        let contains_a = a.contains(42);
        enable();
        let mut b = CustomHashTable::new();
        b.insert(42);
        let contains_b = b.contains(42);
        disable();
        assert_eq!(contains_a, contains_b);
    }

    #[test]
    fn tracing_enabled_captures_the_bucket_actually_used_for_insert() {
        let expected = expected_bucket(42);

        enable();
        let mut table = CustomHashTable::new();
        table.insert(42);
        let events = take_events();
        disable();

        assert_eq!(events, vec![Event::Set { i: expected, value: 42 }]);
    }

    #[test]
    fn tracing_enabled_captures_the_bucket_checked_for_contains() {
        let expected = expected_bucket(42);
        let table = CustomHashTable::from_keys(&[42]);

        enable();
        let found = table.contains(42);
        let events = take_events();
        disable();

        assert!(found);
        assert_eq!(events, vec![Event::Probe { i: expected }]);
    }

    #[test]
    fn tracing_emits_no_event_when_removing_a_missing_key() {
        let mut table = CustomHashTable::new();

        enable();
        let removed = table.remove(99);
        let events = take_events();
        disable();

        assert!(!removed);
        assert!(events.is_empty());
    }

    #[test]
    fn removing_the_only_key_in_a_bucket_traces_the_empty_sentinel() {
        let expected = expected_bucket(42);
        let mut table = CustomHashTable::from_keys(&[42]);

        enable();
        table.remove(42);
        let events = take_events();
        disable();

        assert_eq!(events, vec![Event::Set { i: expected, value: 0 }]);
    }

    #[test]
    fn removing_one_of_several_colliding_keys_traces_a_remaining_survivor() {
        // Deliberately doesn't assume a colliding bucket has exactly two
        // keys (it may well have three or more) — the expected display
        // value is derived from the REAL post-removal state, not a
        // hardcoded guess about which key ends up last.
        let keys: Vec<i32> = (0..20).collect();
        let mut table = CustomHashTable::from_keys(&keys);
        let (bucket_idx, victim) = table
            .buckets
            .iter()
            .enumerate()
            .find(|(_, bucket)| bucket.len() > 1)
            .map(|(idx, bucket)| (idx, bucket[0]))
            .expect("20 keys into 8 buckets should always produce a collision");

        enable();
        table.remove(victim);
        let events = take_events();
        disable();

        assert!(
            !table.buckets[bucket_idx].is_empty(),
            "bucket should still have chained keys after removing just one"
        );
        let remaining = table.buckets[bucket_idx].last().copied().unwrap();
        assert_eq!(
            events,
            vec![Event::Set { i: bucket_idx, value: remaining as i64 }]
        );
    }
}

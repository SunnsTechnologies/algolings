// Shared between exercises/hash-tables/src/hash_entry.rs (the
// learner-facing skeleton) and exercises/hash-tables-solutions/src/hash_entry.rs
// (the reference solution), via `include!()`.
//
// Untraced on purpose — count_frequencies never calls any algolings_trace
// helper, so there's nothing for a replay to show. std HashMap has no
// stable iteration order or index concept to trace against in the first
// place, unlike every other exercise's array/list/bucket-array model.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_produces_an_empty_map() {
        let counts = count_frequencies(&[]);
        assert!(counts.is_empty());
    }

    #[test]
    fn all_distinct_values_each_count_once() {
        let counts = count_frequencies(&[1, 2, 3]);
        assert_eq!(counts.get(&1), Some(&1));
        assert_eq!(counts.get(&2), Some(&1));
        assert_eq!(counts.get(&3), Some(&1));
        assert_eq!(counts.len(), 3);
    }

    #[test]
    fn repeated_values_are_counted_correctly() {
        let counts = count_frequencies(&[1, 2, 1, 3, 1, 2]);
        assert_eq!(counts.get(&1), Some(&3));
        assert_eq!(counts.get(&2), Some(&2));
        assert_eq!(counts.get(&3), Some(&1));
        assert_eq!(counts.len(), 3);
    }

    #[test]
    fn a_value_never_seen_is_absent_not_zero() {
        let counts = count_frequencies(&[1, 1, 1]);
        assert_eq!(counts.get(&99), None);
    }
}

use algolings_trace::probe;

// Implement linear search. Check each element in order until you find the
// target or reach the end.
//
// Use `probe(arr, i, &target)` instead of `arr[i] == target` — same
// behavior, but it lets algolings trace which indices your solution
// actually checked. Call `found(i)` right before returning `Some(i)`.
pub fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    todo!("implement linear search using probe and found")
}

#[cfg(test)]
include!("../../tests-shared/linear_search_tests.rs");

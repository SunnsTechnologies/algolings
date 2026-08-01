mod recursion_basics;
mod tail_recursion;
mod subsets;
mod combinations;
mod permutations;
mod permutations_with_duplicates;
mod n_queens;

pub use recursion_basics::{factorial, fibonacci};
pub use tail_recursion::factorial_tail;
pub use subsets::subsets;
pub use combinations::combinations;
pub use permutations::permutations;
pub use permutations_with_duplicates::permute_unique;
pub use n_queens::solve_n_queens;

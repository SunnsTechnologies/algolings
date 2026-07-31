//! Shared scaffolding for the stack_vec exercise. `push`/`pop`/`peek` are
//! the lesson (in `stack_vec.rs`); `new`/`from_values`/`to_vec`/`len`/
//! `is_empty` are always-implemented infrastructure so the exercise's
//! tests can build or read stack state without depending on the learner's
//! own push/pop being correct.

#[derive(Default)]
pub struct VecStack {
    pub data: Vec<i32>,
}

impl VecStack {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Builds a stack directly from `values`, without going through push —
    /// so pop/peek's tests don't depend on push being solved correctly.
    /// The LAST value in `values` ends up on top, as if each value had
    /// been pushed in that order.
    pub fn from_values(values: &[i32]) -> Self {
        Self {
            data: values.to_vec(),
        }
    }

    pub fn to_vec(&self) -> Vec<i32> {
        self.data.clone()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

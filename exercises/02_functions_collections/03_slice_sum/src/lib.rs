// Concept: borrowed slices and iterating without taking ownership.
// Task: implement `total(items)` so it returns the sum of every integer in the slice.
// Expected behavior/output: `total(&[3, 4, 5])` returns `12`.
// Hint: start at zero and add each value while iterating over the slice.
pub fn total(_items: &[i32]) -> i32 {
    // TODO: add every value in the slice.
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_adds_slice_values() {
        assert_eq!(total(&[3, 4, 5]), 12);
    }
}

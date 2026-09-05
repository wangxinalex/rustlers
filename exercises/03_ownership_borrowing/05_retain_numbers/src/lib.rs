// Concept: filtering a vector through a mutable borrow.
// Task: implement `retain_non_negative(numbers)` so it removes every negative number.
// Expected behavior/output: `[3, -1, 0, -5]` becomes `[3, 0]`.
// Hint: `retain` takes a closure shaped like `|number| condition`.
// Return `true` to keep an element. Its parameter is `&i32`; use `*number`
// to read the integer and compare it with zero, keeping zero too.
// The vector is changed in place; no clone or replacement is needed.
pub fn retain_non_negative(_numbers: &mut Vec<i32>) {
    // TODO: repair this function so it removes every negative number in place.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutable_borrow_retain_removes_negative_numbers() {
        let mut numbers = vec![3, -1, 0, -5];

        retain_non_negative(&mut numbers);

        assert_eq!(numbers, vec![3, 0]);
    }
}

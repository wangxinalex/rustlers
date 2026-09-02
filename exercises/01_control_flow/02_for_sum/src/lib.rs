// Concept: repeating work over an inclusive range with `for`.
// Task: use a `for` loop to sum every integer from 1 through `n`.
// Expected behavior/output: `sum_up_to(5)` returns `15`.
// Hint: start the accumulator at zero, then add each value from `1..=n`.
pub fn sum_up_to(n: u32) -> u32 {
    // TODO: start the accumulator at zero.
    let mut total = 1;
    for value in 1..=n {
        total += value;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_up_to_adds_inclusive_values() {
        assert_eq!(sum_up_to(5), 15);
    }
}

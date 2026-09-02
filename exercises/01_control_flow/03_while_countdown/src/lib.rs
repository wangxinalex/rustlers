// Concept: repeating a loop while a condition remains true.
// Task: use a `while` loop to collect values from `start` down to 1.
// Expected behavior/output: `countdown(3)` returns `[3, 2, 1]`.
// Hint: keep looping while the current value is greater than zero.
pub fn countdown(start: u32) -> Vec<u32> {
    // TODO: include 1 in the countdown.
    let mut current = start;
    let mut values = Vec::new();
    while current > 1 {
        values.push(current);
        current -= 1;
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn countdown_lists_values_until_one() {
        assert_eq!(countdown(3), vec![3, 2, 1]);
    }
}

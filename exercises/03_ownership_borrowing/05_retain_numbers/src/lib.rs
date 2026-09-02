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

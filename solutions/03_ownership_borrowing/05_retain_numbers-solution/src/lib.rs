pub fn retain_non_negative(numbers: &mut Vec<i32>) {
    numbers.retain(|number| *number >= 0);
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

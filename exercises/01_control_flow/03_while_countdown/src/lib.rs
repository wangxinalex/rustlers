pub fn countdown(start: u32) -> Vec<u32> {
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

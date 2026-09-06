pub fn total(items: &[i32]) -> i32 {
    let mut total = 0;
    for value in items {
        total += value;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_adds_slice_values() {
        assert_eq!(total(&[3, 4, 5]), 12);
    }
}

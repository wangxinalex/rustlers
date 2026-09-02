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

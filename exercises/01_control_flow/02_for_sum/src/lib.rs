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

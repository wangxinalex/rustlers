fn main() {
    let mut numbers = vec![3, -1, 0, -5];
    rustlers_03_ownership_borrowing_05_retain_numbers::retain_non_negative(&mut numbers);
    println!("{numbers:?}");
}

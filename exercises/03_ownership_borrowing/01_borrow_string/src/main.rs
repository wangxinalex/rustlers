fn main() {
    let text = "rust".to_string();
    println!(
        "{}",
        rustlers_03_ownership_borrowing_01_borrow_string::length_after_borrow(&text)
    );
}

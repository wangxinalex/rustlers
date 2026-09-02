fn main() {
    let mut text = String::from("learn");
    rustlers_03_ownership_borrowing_04_mutable_string::add_suffix(&mut text, "ing");
    println!("{text}");
}

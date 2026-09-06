fn main() {
    let mut text = String::from("learn");
    rustlers_03_ownership_borrowing_03_mutable_string_solution::add_suffix(&mut text, "ing");
    println!("{text}");
}

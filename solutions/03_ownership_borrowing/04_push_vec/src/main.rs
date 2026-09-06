fn main() {
    let mut items = Vec::new();
    rustlers_03_ownership_borrowing_04_push_vec_solution::add_item(&mut items, "tea");
    println!("{items:?}");
}

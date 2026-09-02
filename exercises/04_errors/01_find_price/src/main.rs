fn main() {
    let prices = [("tea", 12), ("coffee", 20)];
    println!(
        "{:?}",
        rustlers_04_errors_01_find_price::find_price(&prices, "tea")
    );
}

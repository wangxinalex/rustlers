fn main() {
    let actual = rustlers_05_mini_cli_01_text_stats_solution::stats("one two\nthree");
    println!(
        "lines={} words={} bytes={}",
        actual.lines, actual.words, actual.bytes
    );
}

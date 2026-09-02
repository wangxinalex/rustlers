fn main() {
    let actual = rustlers_05_mini_cli_01_text_stats::stats("one two\nthree");
    println!(
        "lines={} words={} bytes={}",
        actual.lines, actual.words, actual.bytes
    );
}

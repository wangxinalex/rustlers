use std::path::Path;

fn main() {
    let mut args = std::env::args();
    let program = args.next().unwrap_or_else(|| String::from("file-report"));
    let Some(path) = args.next() else {
        eprintln!("usage: {program} <path>");
        std::process::exit(2);
    };

    if args.next().is_some() {
        eprintln!("usage: {program} <path>");
        std::process::exit(2);
    }

    match rustlers_05_mini_cli_02_file_report::report_file(Path::new(&path)) {
        Ok(report) => println!("{report}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

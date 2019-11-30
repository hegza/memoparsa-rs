fn main() {
    let file_contents = include_str!("../../data/alpha.md");

    // create directory for example output
    match std::fs::create_dir("example_results") {
        Err(_) => println!("Example output directory exists at ./example_results/"),
        Ok(()) => {}
    }

    // do work
    memoparsa::save_as_ics(
        memoparsa::SourceFormat::Alpha,
        2019,
        file_contents,
        "example_results/alpha.ics",
    );
    println!("File created at example_results/alpha.ics.");
}

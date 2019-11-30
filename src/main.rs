#[macro_use]
extern crate clap;
extern crate exitcode;
use clap::{App, Arg};

static OUTPUT: &str = "parsa.ics";

fn main() {
    let matches = App::new("memoparsa")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .args_from_usage(
            "-o, --output=[FILE] 'Sets custom output file'
            <INPUT>              'Sets input file to use'",
        )
        .arg(
            Arg::from_usage("<FORMAT>             'Sets input format'")
                .validator(validate_input_format_spec),
        )
        .get_matches();
    println!("Tester program for cli implementation");
    if let Some(output) = matches.value_of("output") {
        println!("Selected output file: {}", output);
    } else {
        let output = OUTPUT;
        println!("Using default output file: {}", output);
    }
    if let Some(input) = matches.value_of("INPUT") {
        println!("Selected input file: {}", input);
    } else {
        println! {"Fatal error: no input file specified"};
        std::process::exit(exitcode::DATAERR);
    }
    if let Some(in_format) = matches.value_of("FORMAT") {
        println!("Selected input format: {}", in_format);
    } else {
        println! {"Fatal error: no input format specified"};
        std::process::exit(exitcode::DATAERR);
    }
    std::process::exit(exitcode::OK);
}

/** Checks that the input format specifier is one of the allowed formats, eg.
 *  alpha. */
fn validate_input_format_spec(s: String) -> Result<(), String> {
    match s.as_ref() {
        "alpha" | "ALPHA" => Ok(()),
        _ => {
            let mut msg = "unknown format: ".to_string();
            msg.push_str(&s);
            Err(msg)
        }
    }
}

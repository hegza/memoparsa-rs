#[macro_use]
extern crate clap;
use chrono::prelude::*;
use clap::{App, Arg};

fn main() {
    // enable full logging with RUST_LOG=memoparsa=trace
    env_logger::init();

    // handle command line arguments
    let matches = cli();

    let source_contents = std::fs::read_to_string(matches.value_of("input").unwrap()).unwrap();
    let format = match matches.value_of("format").unwrap().as_ref() {
        "alpha" | "ALPHA" => memoparsa::SourceFormat::Alpha,
        _ => panic!("unknown format"),
    };
    let year = matches
        .value_of("start-year")
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let output_file = matches.value_of("output");

    // do work
    match output_file {
        Some(output_file) => memoparsa::save_as_ics(format, year, &source_contents, output_file),
        None => memoparsa::parse(format, year, &source_contents),
    }

    std::process::exit(exitcode::OK);
}

fn cli<'a>() -> clap::ArgMatches<'a> {
    let matches = App::new("memoparsa")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(Arg::from_usage("-o, --output=[FILE] 'Sets custom output file'"))
        .args_from_usage(
            "<input>              'Sets input file to use'",
        )
        .arg(
            Arg::from_usage("<format>             'Sets input format'")
                .validator(validate_input_format_spec),
        )
        .arg(
            Arg::from_usage("-y, --start-year=[NUMBER] 'Sets custom start year as context for the input file. This year is used by default for formats that require it.'")
                .default_value_if("format", Some("alpha"), &Local::now().year().to_string())
        )
        .get_matches();
    println!("Tester program for cli implementation");
    if let Some(year) = matches.value_of("start-year") {
        println!("Selected start-year: {}", year);
    } else {
        let year = Local::now().year();
        println!(
            "Using default year, current year in local time-zone: {}",
            year
        );
    }
    if let Some(input) = matches.value_of("input") {
        println!("Selected input file: {}", input);
    } else {
        println! {"Fatal error: no input file specified"};
        std::process::exit(exitcode::DATAERR);
    }
    if let Some(in_format) = matches.value_of("format") {
        println!("Selected input format: {}", in_format);
    } else {
        println! {"Fatal error: no input format specified"};
        std::process::exit(exitcode::DATAERR);
    }
    matches
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

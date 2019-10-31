#[macro_use]
extern crate clap;
extern crate exitcode;
use clap::{Arg, App, SubCommand};
use std::process;

static OUTPUT: &str = "parsa.txt";

fn main() {
    let matches = App::new("memoparsa")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .args_from_usage(
            "-o, --output=[FILE] 'Sets custom output file'
            <INPUT>              'Sets input file to use'")
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
        println!{"Fatal error: no input file specified"};
        std::process::exit(exitcode::DATAERR);
    }
    std::process::exit(exitcode::OK);
}
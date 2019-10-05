#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("memoparsa")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .args_from_usage(
            "-o, --output=[FILE] 'Sets custom output file'
            <INPUT>              'Sets input file to use'")
        .get_matches();
    println!("Tester program for cli implementation")
}
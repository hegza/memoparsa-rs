use std::fs;
use crate::format::alpha::Event;

#[test]
fn alpha_parses_correct() {
    let file_contents = include_str!("alpha.md");

    // split into a queue of lines
    let lines = file_contents.split('\n');

    // process lines into DOM
    let entries = lines.filter_map(|line| Event::from_str(line, 2019).ok());

    for entry in entries {
        println!("{:?}", entry);
    }
}

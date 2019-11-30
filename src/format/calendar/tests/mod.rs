use super::parse_calendar;
use chrono::prelude::*;

#[test]
fn calendar_parses_correct() {
    let _ = env_logger::builder().is_test(true).try_init();

    let file_contents = include_str!("calendar.md");

    // feed the whole file into the library and process into DOM events
    let entries = parse_calendar(&file_contents, NaiveDate::from_ymd(2019, 8, 23));

    for entry in entries {
        println!("DOM Event: {:?}", entry);
    }
}

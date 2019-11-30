use ics::ICalendar;
use memoparsa::{CreateIcsEvent, Event, Tag};

#[test]
fn alpha_parses_correct() {
    let _ = env_logger::builder().is_test(true).try_init();

    let file_contents = include_str!("../../data/alpha.md");

    // split into a queue of lines
    let lines = file_contents.split('\n');

    // process lines into DOM
    let entries = lines.filter_map(|line| Event::from_str(line, 2019).ok());

    for entry in entries {
        println!("DOM Event: {:?}", entry);
    }
}

#[test]
fn alpha_converts_to_ics() {
    let _ = env_logger::builder().is_test(true).try_init();

    let file_contents = include_str!("../../data/alpha.md");

    // split into a queue of lines
    let lines = file_contents.split('\n');

    // process lines into DOM
    let ics_entries = lines
        .filter_map(|line| Event::from_str(line, 2019).ok())
        .filter(|entry| entry.tags.contains(&Tag::PublishToIcs))
        .collect::<Vec<Event>>();

    let mut calendar = ICalendar::new("2.0", "alpha");

    for entry in &ics_entries {
        let event = entry.create_ics_event();
        calendar.add_event(event);
    }

    println!("ICalendar object: {:?}", calendar);
}

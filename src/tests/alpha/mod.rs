use crate::format::alpha::{Tag, Event, DateVariant};
use ics::properties::{Comment, DtStart, DtEnd, Summary};
use ics::{ICalendar, ToDo, Event as IcsEvent};
use uuid::Uuid;
use chrono::prelude::*;

#[test]
fn alpha_parses_correct() {
    let file_contents = include_str!("alpha.md");

    // split into a queue of lines
    let lines = file_contents.split('\n');

    // process lines into DOM
    let entries = lines.filter_map(|line| Event::from_str(line, 2019).ok());

    for entry in entries {
        println!("DOM Event: {:?}", entry);
    }
}

const DATETIME_FORMAT: &str = "%Y%m%dT%H%M%SZ";
const DATE_FORMAT: &str = "%Y%m%d";

#[test]
fn alpha_converts_to_ics() {
    let file_contents = include_str!("alpha.md");

    // split into a queue of lines
    let lines = file_contents.split('\n');

    // process lines into DOM
    let ics_entries = lines.filter_map(|line| Event::from_str(line, 2019).ok())
            .filter(|entry| entry.tags.contains(&Tag::PublishToIcs)).collect::<Vec<Event>>();

    let mut calendar = ICalendar::new("2.0", "alpha");

    for entry in &ics_entries {
        let event = entry.create_ics_event().unwrap();
        calendar.add_event(event);
    }

    println!("ICalendar object: {:?}", calendar);
}

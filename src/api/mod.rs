use crate::format::alpha::{Event, Tag};
use crate::format::CreateIcsEvent;
use ics::ICalendar;
use std::path::Path;

pub enum SourceFormat {
    Alpha,
}

pub fn save_as_ics<P>(format: SourceFormat, year: i32, source: &str, destination: P)
where
    P: AsRef<Path>,
{
    match format {
        SourceFormat::Alpha => {
            save_alpha_as_ics(year, source, destination);
        }
    }
}

fn save_alpha_as_ics<P>(year: i32, source: &str, destination: P)
where
    P: AsRef<Path>,
{
    // split into a queue of lines
    let lines = source.split('\n');

    // process lines into DOM
    let events = lines
        .filter_map(|line| Event::from_str(line, year).ok())
        .filter(|entry| entry.tags.contains(&Tag::PublishToIcs))
        .collect::<Vec<Event>>();

    let mut calendar = ICalendar::new("2.0", "alpha");

    for entry in &events {
        let event = entry.create_ics_event();
        calendar.add_event(event);
    }

    calendar
        .save_file(destination.as_ref().to_str().unwrap())
        .unwrap();
}

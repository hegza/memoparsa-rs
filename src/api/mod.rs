// Event, Tag, and CreateIcsEvent are part of the API
pub use crate::format::{
    alpha::{Event, Tag},
    CreateIcsEvent,
};

use ics::{ICalendar, TimeZone, ZoneTime};
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

fn save_alpha_as_ics<P>(start_year: i32, source: &str, destination: P)
where
    P: AsRef<Path>,
{
    // split into a queue of lines
    let lines = source.split('\n');

    // process lines into DOM
    let mut events = Vec::new();
    let mut cur_year = start_year;
    for line in lines {
        let event = Event::from_str(line, cur_year);
        if event.is_ok() {
            let event = event.unwrap();
            if event.tags.contains(&Tag::PublishToIcs) {
                info!("publishing event {:?} to ics", event);
                events.push(event);
            }
        } else if let Ok(year) = line.parse::<i32>() {
            debug!("context changes year: {}", year);
            cur_year = year;
        } else {
            debug!("ignored line {}", line);
        }
    }

    let mut calendar = ICalendar::new("2.0", "alpha");

    // Add Helsinki timezone
    let tz = TimeZone::new(
        "Europe/Helsinki",
        // NOTE: if the law for daylight saving time changes in Finland, use ZoneTime::standard
        ZoneTime::daylight("19671025T040000", "+0200", "+0300"),
    );
    calendar.add_timezone(tz);

    for entry in &events {
        let event = entry.create_ics_event();
        calendar.add_event(event);
    }

    calendar
        .save_file(destination.as_ref().to_str().unwrap())
        .unwrap();
}

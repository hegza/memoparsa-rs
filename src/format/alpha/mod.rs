#[cfg(test)]
mod test;

use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::{Europe, Tz};
use std::collections::HashMap;

const TZ: Tz = Europe::Helsinki;

lazy_static! {
static ref TIME_FORMATS: Vec<&'static str> = vec![
    "%k:%M", // 23:59
    "%k%M",  // 2359
];

static ref IGNORED_WEEKDAY_LABELS: Vec<&'static str> = vec![
    "ma", "ti", "ke", "to", "pe", "la", "su", "Mo", "Tu", "We", "Th", "Fr", "Sa", "Su",
];

static ref TAG_BY_KEYCHAR: HashMap<char, Tag> = hashmap! {
    'p' => Tag::PublishToIcs,
};
}

/// Ordered from most specific and well specified to least specific / context dependent.
#[derive(Debug, PartialEq)]
enum DateVariant {
    TimeSpan(DateTime<Local>, DateTime<Local>),
    DateTime(DateTime<Local>),
    Date(Date<Local>),
    Month { year: u32, month: u32 },
    Year(u32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tag {
    PublishToIcs,
}

#[derive(Debug, PartialEq)]
pub struct Event {
    date: DateVariant,
    tags: Vec<Tag>,
    description: String,
}

#[derive(Debug)]
pub struct ParseError(String);

fn parse_date(s: &str, year: i32) -> Option<NaiveDate> {
    trace!("attempting to parse date from: {}", s);

    let parts = s.split('.').collect::<Vec<&str>>();
    // cannot contain both a day and a month
    if parts.len() < 2 {
        return None;
    }

    let day = match parts[0].parse::<u32>() {
        Ok(u) => u,
        Err(_) => return None,
    };
    let month = match parts[1].parse::<u32>() {
        Ok(u) => u,
        Err(_) => return None,
    };

    let date = NaiveDate::from_ymd(year, month, day);
    trace!("parsed: {:?}", date);
    Some(date)
}

fn parse_time(s: &str) -> Option<NaiveTime> {
    for fmt in TIME_FORMATS.iter() {
        if let Ok(t) = NaiveTime::parse_from_str(s, fmt) {
            return Some(t);
        }
    }
    None
}

fn parse_timespan(s: &str, year_of_start: i32) -> Option<(NaiveDate, NaiveDate)> {
    let timespan_parts = s.split('-').collect::<Vec<&str>>();
    if timespan_parts.len() != 2 {
        return None;
    }
    let left = timespan_parts[0];
    let right = timespan_parts[1];

    // end date must exist in full-form in a time span
    let end_date = match parse_date(right, year_of_start) {
        Some(date) => date,
        None => return None,
    };

    let start_date = {
        // try parsing the thing preceding the dash as the start date
        if let Some(date) = parse_date(left, year_of_start) {
            date
        } else {
            // parse thing preceding dash as the day
            let day = match left.trim_end_matches('.').parse::<u32>() {
                Ok(i) => i,
                Err(_) => return None,
            };

            // take month from end-date as the start date didn't specify it
            let month = end_date.month();
            NaiveDate::from_ymd(year_of_start, month, day)
        }
    };

    // if end date is before start date, the end date was probably of the next year
    if start_date < end_date {
        Some((start_date, end_date))
    } else {
        Some((
            start_date,
            // HACK: Duration::days gets broken by leap days (year is actually 365.25)
            end_date.checked_add_signed(Duration::days(365)).unwrap(),
        ))
    }
}

fn maybe_parse_and_consume_tags(parts: &mut Vec<&str>) -> Vec<Tag> {
    // pick out the first continuous stream of tokens as the tag list candidate
    let candidate = parts[0];

    let mut tags: Vec<Tag> = vec![];
    for c in candidate.chars() {
        match TAG_BY_KEYCHAR.get(&c) {
            Some(tag) => tags.push(*tag),
            None => return vec![],
        };
    }

    // success: remove the tag candidate and return tags
    parts.remove(0);
    tags
}

fn maybe_remove_weekday_label(parts: &mut Vec<&str>) {
    // remove the first part if it's the weekday label
    if IGNORED_WEEKDAY_LABELS.contains(parts.first().unwrap()) {
        parts.remove(0);

        trace!("removed weekday-label");
        trace!("> {:?}", &parts);
    }
}

impl Event {
    pub fn from_str(s: &str, year: i32) -> Result<Self, ParseError> {
        debug!("start parsing Event::from_str(\"{}\", {})", s, year);

        // split input string into parts on whitespace
        let mut parts = s.split_whitespace().collect::<Vec<&str>>();
        if parts.len() < 2 {
            return Err(ParseError(format!("could not parse Event from \"{}\", not enough elements to create both date and description", s)));
        }

        let date = {
            // if the first element is identified as a weekday label, remove it
            maybe_remove_weekday_label(&mut parts);

            // try parse time from the second element
            let time_result = parse_time(parts[1]);

            let mut datevariant = None;
            // try parse a time-span from the first element
            if let Some((start_date, end_date)) = parse_timespan(parts.first().unwrap(), year) {
                trace!("parsed time-span: {:?}", (start_date, end_date));

                // HACK: 23:59 for end time seems like a sensible default
                let end_date_time = end_date.and_hms(23, 59, 0);
                // try add a start time from the second element
                let start_date_time = match time_result {
                    Some(time) => start_date.and_hms(time.hour(), time.minute(), 0),
                    // HACK: 06:00 seems like a sensible default
                    None => start_date.and_hms(6, 0, 0),
                };

                datevariant = Some(DateVariant::TimeSpan(
                    TZ.from_local_datetime(&start_date_time)
                        .unwrap()
                        .with_timezone(&Local),
                    TZ.from_local_datetime(&end_date_time)
                        .unwrap()
                        .with_timezone(&Local),
                ));
            }
            // try parse a date from the first element
            else if let Some(date) = parse_date(parts.first().unwrap(), year) {
                trace!("parsed date: {:?}", date);

                // try add a start time from the second element
                let dv;
                if let Some(time) = time_result {
                    let date_time = date.and_hms(time.hour(), time.minute(), 0);
                    dv = DateVariant::DateTime(
                        TZ.from_local_datetime(&date_time)
                            .unwrap()
                            .with_timezone(&Local),
                    );
                } else {
                    dv =
                        DateVariant::Date(TZ.from_local_date(&date).unwrap().with_timezone(&Local));
                }
                datevariant = Some(dv);
            };

            // if a date was parsed, consume the strings used in making it
            if datevariant.is_some() {
                parts.remove(0);
                trace!("consumed date");
                trace!("> {:?}", &parts);

                // if a time was parsed, consume the strings used in making it
                if time_result.is_some() {
                    parts.remove(0);
                    trace!("consumed time");
                    trace!("> {:?}", &parts);
                }

                datevariant.unwrap()
            } else {
                return Err(ParseError(format!("could not parse date from {}", s)));
            }
        };

        // parse tags if possible
        let tags = maybe_parse_and_consume_tags(&mut parts);

        let description = parts.join(" ");

        let event = Event {
            date,
            tags,
            description,
        };
        debug!("parsed: {:?}", event);
        Ok(event)
    }
}

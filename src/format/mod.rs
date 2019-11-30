pub mod alpha;
pub mod calendar;

use chrono::prelude::*;
use chrono_tz::{Europe, Tz};
use ics::properties::{Comment, DtEnd, DtStart, Summary};
use uuid::Uuid;

const TZ: Tz = Europe::Helsinki;
const DATETIME_FORMAT: &str = "%Y%m%dT%H%M%SZ";
const DATE_FORMAT: &str = "%Y%m%d";

lazy_static! {
static ref TIME_FORMATS: Vec<&'static str> = vec![
    "%k:%M", // 23:59
    "%k%M",  // 2359
];
}

pub trait Event {
    fn date(&self) -> &DateVariant;
    fn description(&self) -> &str;
}

pub trait CreateIcsEvent {
    fn create_ics_event<'a>(&'a self) -> ics::Event<'a>;
}

/// Ordered from most specific and well specified to least specific / context dependent.
#[derive(Debug, PartialEq, Clone)]
pub enum DateVariant {
    TimeSpan(DateTime<Local>, DateTime<Local>),
    DateTime(DateTime<Local>),
    Date(Date<Local>),
    Month { year: u32, month: u32 },
    Year(u32),
}

impl<T> CreateIcsEvent for T
where
    T: Event,
{
    fn create_ics_event(&self) -> ics::Event {
        let mut event = ics::Event::new(
            Uuid::new_v4().to_string(),
            Utc::now().format(DATETIME_FORMAT).to_string(),
        );
        match self.date() {
            DateVariant::TimeSpan(start, end) => {
                event.push(DtStart::new(start.format(DATETIME_FORMAT).to_string()));
                event.push(DtEnd::new(end.format(DATETIME_FORMAT).to_string()));
            }
            DateVariant::DateTime(date) => {
                event.push(DtStart::new(date.format(DATETIME_FORMAT).to_string()));
            }
            DateVariant::Date(date) => {
                let date_fmt = date.format(DATE_FORMAT);
                let date_str = date_fmt.to_string();
                event.push(DtStart::new(date_str));
            }
            DateVariant::Month { year, month } => {
                unimplemented!("converting 'year/month' events into .ics is not implemented");
            }
            DateVariant::Year(_) => {
                unimplemented!("converting 'year' events into .ics is not implemented");
            }
        }
        event.push(Summary::new(self.description()));
        event.push(Comment::new("created with memoparsa"));
        event
    }
}

pub fn parse_date(s: &str, year: i32) -> Option<NaiveDate> {
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

pub fn parse_timespan(s: &str) -> Option<(NaiveTime, NaiveTime)> {
    let timespan_parts = s.split('-').collect::<Vec<&str>>();
    if timespan_parts.len() != 2 {
        return None;
    }
    let left = timespan_parts[0];
    let right = timespan_parts[1];

    let end_time = match parse_time(right) {
        Some(time) => time,
        None => return None,
    };

    let start_time = match parse_time(left) {
        Some(time) => time,
        None => return None,
    };

    Some((start_time, end_time))
}

pub fn parse_time(s: &str) -> Option<NaiveTime> {
    for fmt in TIME_FORMATS.iter() {
        if let Ok(t) = NaiveTime::parse_from_str(s, fmt) {
            return Some(t);
        }
    }
    None
}

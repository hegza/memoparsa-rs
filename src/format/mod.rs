pub mod alpha;

use chrono::prelude::*;
use ics::properties::{Comment, DtEnd, DtStart, Summary};
use uuid::Uuid;

const DATETIME_FORMAT: &str = "%Y%m%dT%H%M%SZ";
const DATE_FORMAT: &str = "%Y%m%d";

pub trait Event {
    fn date(&self) -> &DateVariant;
    fn description(&self) -> &str;
}

pub trait CreateIcsEvent {
    fn create_ics_event(&self) -> ics::Event;
}

/// Ordered from most specific and well specified to least specific / context dependent.
#[derive(Debug, PartialEq)]
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

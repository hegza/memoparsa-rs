use super::*;
use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Event {
    pub date: DateVariant,
    pub description: String,
}

enum LineType {
    Date,
    Event,
}

lazy_static! {
    static ref LINETYPE_BY_TOKEN: HashMap<&'static str, LineType> = hashmap! {
        "###" => LineType::Date,
        "*" => LineType::Event,
        "-" => LineType::Event,
    };
}

enum Context {
    Year(i32),
    Date(NaiveDate),
}

pub fn parse_calendar(source: &str, date_ctx: NaiveDate) -> Vec<Event> {
    let mut events = Vec::new();
    let mut ctx = Context::Date(date_ctx);

    let lines = source.split('\n');
    for line in lines {
        let mut tokens = line.split_whitespace().collect::<Vec<&str>>();

        // linebreak: date context is no longer valid
        if tokens.len() == 0 {
            match ctx {
                Context::Date(date) => {
                    // degrade context date into a year
                    ctx = Context::Year(date.year());
                    continue;
                }
                Context::Year(year) => {}
            }
        }

        if tokens.len() < 2 {
            trace!(
                "ignore line: \"{:?}\" because it's less than 2 tokens long",
                line
            );
            continue;
        }

        // if the first token is not a valid starter token, break off to the next line
        let starter_token_candidate = tokens.remove(0);
        let line_type = LINETYPE_BY_TOKEN.get(&starter_token_candidate);
        if line_type.is_none() {
            trace!(
                "ignore line: \"{:?}\" due to unidentified start token",
                line
            );
            continue;
        }
        let line_type = line_type.unwrap();

        match line_type {
            LineType::Date => {
                let year = match ctx {
                    Context::Year(year) => year,
                    Context::Date(date) => date.year(),
                };
                for token in tokens {
                    // try parse the first or the second token into a date
                    if let Some(date) = parse_date(token, year) {
                        trace!("set date context to {:?} based on \"{:?}\"", date, line);
                        ctx = Context::Date(date);
                        continue;
                    } else {
                        trace!(
                            "ignore date candidate: \"{:?}\" because the second element is not a date",
                            line
                        );
                        continue;
                    }
                }
            }
            LineType::Event => {
                // events only apply when there is a valid date context
                if let Context::Date(date) = ctx {
                    let timing_candidate = tokens.remove(0);
                    // try parse the first token into a time span
                    if let Some((start_time, end_time)) = parse_timespan(timing_candidate) {
                        let start_date = date.and_hms(start_time.hour(), start_time.minute(), 0);
                        let end_date = date.and_hms(end_time.hour(), end_time.minute(), 0);
                        let dv = DateVariant::TimeSpan(
                            TZ.from_local_datetime(&start_date)
                                .unwrap()
                                .with_timezone(&Local),
                            TZ.from_local_datetime(&end_date)
                                .unwrap()
                                .with_timezone(&Local),
                        );

                        let event = Event {
                            date: dv,
                            description: tokens.join(" "),
                        };
                        debug!("create event: {:?}", event);
                        events.push(event);
                    }
                    // try parse the first token into a time
                    if let Some(time) = parse_time(timing_candidate) {
                        let date_time = date.and_hms(time.hour(), time.minute(), 0);
                        let dv = DateVariant::DateTime(
                            TZ.from_local_datetime(&date_time)
                                .unwrap()
                                .with_timezone(&Local),
                        );
                        let event = Event {
                            date: dv,
                            description: tokens.join(" "),
                        };
                        debug!("create event: {:?}", event);
                        events.push(event);
                    }
                    trace!(
                        "ignore line \"{:?}\" because \"{:?}\" is not a time-span nor a time",
                        line,
                        timing_candidate
                    );
                }
            }
        }
    }

    events
}

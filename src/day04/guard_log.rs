use chrono::prelude::*;
use regex::Regex;
use std::fmt;
use std::error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GuardLog {
    pub utc: DateTime<Utc>,
    pub log_type: GuardLogType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GuardLogType {
    Wake,
    Sleep,
    BeginShift(u32)
}

impl std::str::FromStr for GuardLog {
    type Err = ParseGuardLogError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       lazy_static! {
            static ref RE: Regex = Regex::new("^\\[(\\d{4}-\\d{2}-\\d{2} \\d{2}:\\d{2})\\] (.*)$").unwrap();
        }

        for capture in RE.captures_iter(s) {
            let utc = Utc.datetime_from_str(&capture[1], "%Y-%m-%d %H:%M")?;
            let log_type = parse_guard_type(&capture[2])?;

            return Ok(GuardLog {
                utc,
                log_type,
            });
        }

        Err(ParseGuardLogError{ message: String::from("Log format not a match") })
    }
}

fn parse_guard_type(s: &str) -> Result<GuardLogType, ParseGuardLogError> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^Guard #(\\d+) begins shift$").unwrap();
    }

    if s == "wakes up" {
        return Ok(GuardLogType::Wake);

    } else if s == "falls asleep" {
        return Ok(GuardLogType::Sleep)

    } else if s.starts_with("Guard #") {
        for capture in RE.captures_iter(s) {
            let id: u32 = capture[1].parse()?;
            return Ok(GuardLogType::BeginShift(id));
        }

        return Err(ParseGuardLogError{ message: String::from("Could not recognize structure of shift start message") })
    }

    Err(ParseGuardLogError{ message: String::from("Could not understand log message") })
}

#[derive(Debug)]
pub struct ParseGuardLogError {
    message: String,
}

impl From<chrono::ParseError> for ParseGuardLogError {
    fn from(_: chrono::ParseError) -> Self {
        ParseGuardLogError {
            message: String::from("Could not convert timestamp"),
        }
    }
}

impl From<std::num::ParseIntError> for ParseGuardLogError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseGuardLogError {
            message: String::from("Could not convert guard id to u32")
        }

    }
}

impl fmt::Display for ParseGuardLogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing log entry: {}", self.message)
    }
}

impl error::Error for ParseGuardLogError {

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_wake() {
        let log: GuardLog = "[1518-08-21 00:39] wakes up".parse().unwrap();
        assert_eq!(GuardLog { utc: Utc.ymd(1518, 8, 21).and_hms(0, 39, 0), log_type: GuardLogType::Wake }, log);
    }

    #[test]
    fn parse_asleep() {
        let log: GuardLog = "[1518-06-16 00:41] falls asleep".parse().unwrap();
        assert_eq!(GuardLogType::Sleep, log.log_type);
    }

    #[test]
    fn parse_begin_shift() {
        let log: GuardLog = "[1518-08-27 00:00] Guard #3323 begins shift".parse().unwrap();
        assert_eq!(GuardLogType::BeginShift(3323), log.log_type);
    }

    #[test]
    fn can_sort_list_of_log_entries() {
        let mut entries: Vec<GuardLog> = vec![
            "[1518-08-21 00:39] wakes up".parse().unwrap(),
            "[1518-08-11 00:56] wakes up".parse().unwrap(),
            "[1518-10-10 23:52] Guard #2707 begins shift".parse().unwrap(),
        ];

        entries.sort();

        assert_eq!(Utc.ymd(1518, 8, 11).and_hms(0, 56, 0), entries[0].utc);
        assert_eq!(Utc.ymd(1518, 8, 21).and_hms(0, 39, 0), entries[1].utc);
        assert_eq!(Utc.ymd(1518, 10, 10).and_hms(23, 52, 0), entries[2].utc);
    }
}
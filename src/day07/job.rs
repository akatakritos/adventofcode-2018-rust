use regex::Regex;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Job {
    pub name: char,
    pub prereq: char,
}

#[derive(Debug)]
pub enum JobParseError {
    UnexpectedFormat,
}

impl Display for JobParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JobParseError::UnexpectedFormat => {
                write!(f, "Unexpected format parsing a job description")
            }
        }
    }
}

impl Error for JobParseError {}

impl FromStr for Job {
    type Err = JobParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new("Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
        }

        for capture in RE.captures_iter(s) {
            let prereq = capture[1].chars().nth(0).unwrap();
            let name = capture[2].chars().nth(0).unwrap();

            return Ok(Job { name, prereq });
        }

        Err(JobParseError::UnexpectedFormat)
    }
}

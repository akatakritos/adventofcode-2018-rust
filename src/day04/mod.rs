mod guard_log;

use chrono::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

pub use self::guard_log::GuardLog;
pub use self::guard_log::GuardLogType;

pub fn read_logs(filename: &str) -> Result<Vec<GuardLog>, Box<dyn Error>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);

    let mut results = vec![];
    for line in reader.lines() {
        let line = line.unwrap();

        if line.len() > 0 {
            results.push(line.parse()?);
        }
    }

    results.sort();
    Ok(results)
}

pub fn find_sleepiest_guard(logs: &Vec<GuardLog>) -> u32 {
    let mut current_guard: Option<u32> = None;
    let mut started_sleep: Option<DateTime<Utc>> = None;
    let mut sleep_times: HashMap<u32, u32> = HashMap::new();

    for log in logs.iter() {
        match log.log_type {
            GuardLogType::BeginShift(id) => current_guard = Some(id),
            GuardLogType::Sleep => started_sleep = Some(log.utc),
            GuardLogType::Wake => {
                let duration = log.utc.signed_duration_since(started_sleep.unwrap());
                let sleep_amount = sleep_times.entry(current_guard.unwrap()).or_insert(0);
                *sleep_amount += duration.num_minutes() as u32;
            }
        }
    }

    let mut max_id = 0;
    let mut max_sleep = 0;
    for (id, total_sleep) in sleep_times.iter() {
        if *total_sleep > max_sleep {
            max_sleep = *total_sleep;
            max_id = *id;
        }
    }

    max_id
}

pub fn find_sleepiest_minute(logs: &Vec<GuardLog>, guard_id: u32) -> (u32, u32) {
    let mut sleep_map: HashMap<u32, u32> = HashMap::new();

    let mut on_shift = false;
    let mut started_sleep: Option<DateTime<Utc>> = None;

    for log in logs.iter() {
        match log.log_type {
            GuardLogType::BeginShift(id) => {
                on_shift = id == guard_id;
            },
            GuardLogType::Sleep => {
                if on_shift {
                    started_sleep = Some(log.utc);
                }
            },
            GuardLogType::Wake => {
                if on_shift {
                    for minute in started_sleep.unwrap().minute()..log.utc.minute() {
                        let count = sleep_map.entry(minute).or_insert(0);
                        *count += 1;
                    }
                    started_sleep = None;
                }
            }
        }
    }


    let mut max_minute = 0;
    let mut max_count = 0;
    for (minute, count) in sleep_map.iter() {
        if *count > max_count {
            max_count = *count;
            max_minute = *minute;
        }
    }

    (max_minute, max_count)
}

pub fn find_sleepiest_guard_minute(logs: &Vec<GuardLog>) -> (u32, u32) {
    let mut guard_ids: Vec<u32> = logs.iter()
        .filter(|log| match log.log_type {
            GuardLogType::BeginShift(_) => true,
            _ => false
        })
        .map(|log| match log.log_type {
            GuardLogType::BeginShift(id) => id,
            _ => panic!("nope")
        })
        .collect();

        guard_ids.dedup();

        let mut max_guard_minute = (0, 0);
        let mut max_count = 0;

        for guard_id in guard_ids {
            let (minute, count) = find_sleepiest_minute(&logs, guard_id);
            if count > max_count {
                max_count = count;
                max_guard_minute = (guard_id, minute);
            }
        }

        max_guard_minute
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_logs_gets_all_items() {
        let logs = read_logs("inputs\\day04.txt").unwrap();

        assert_eq!(1129, logs.len());
    }

    #[test]
    fn find_sleepiest_guard_sample() {
        let mut logs: Vec<GuardLog> = vec![
            "[1518-11-01 00:00] Guard #10 begins shift".parse().unwrap(),
            "[1518-11-01 00:05] falls asleep".parse().unwrap(),
            "[1518-11-01 00:25] wakes up".parse().unwrap(),
            "[1518-11-01 00:30] falls asleep".parse().unwrap(),
            "[1518-11-01 00:55] wakes up".parse().unwrap(),
            "[1518-11-01 23:58] Guard #99 begins shift".parse().unwrap(),
            "[1518-11-02 00:40] falls asleep".parse().unwrap(),
            "[1518-11-02 00:50] wakes up".parse().unwrap(),
            "[1518-11-03 00:05] Guard #10 begins shift".parse().unwrap(),
            "[1518-11-03 00:24] falls asleep".parse().unwrap(),
            "[1518-11-03 00:29] wakes up".parse().unwrap(),
            "[1518-11-04 00:02] Guard #99 begins shift".parse().unwrap(),
            "[1518-11-04 00:36] falls asleep".parse().unwrap(),
            "[1518-11-04 00:46] wakes up".parse().unwrap(),
            "[1518-11-05 00:03] Guard #99 begins shift".parse().unwrap(),
            "[1518-11-05 00:45] falls asleep".parse().unwrap(),
            "[1518-11-05 00:55] wakes up".parse().unwrap(),
        ];

        logs.sort();

        let id = find_sleepiest_guard(&logs);
        assert_eq!(10, id);
    }

    #[test]
    fn find_sleepiest_minute_sample() {
        let mut logs: Vec<GuardLog> = vec![
            "[1518-11-01 00:00] Guard #10 begins shift".parse().unwrap(),
            "[1518-11-01 00:05] falls asleep".parse().unwrap(),
            "[1518-11-01 00:25] wakes up".parse().unwrap(),
            "[1518-11-01 00:30] falls asleep".parse().unwrap(),
            "[1518-11-01 00:55] wakes up".parse().unwrap(),
            "[1518-11-01 23:58] Guard #99 begins shift".parse().unwrap(),
            "[1518-11-02 00:40] falls asleep".parse().unwrap(),
            "[1518-11-02 00:50] wakes up".parse().unwrap(),
            "[1518-11-03 00:05] Guard #10 begins shift".parse().unwrap(),
            "[1518-11-03 00:24] falls asleep".parse().unwrap(),
            "[1518-11-03 00:29] wakes up".parse().unwrap(),
            "[1518-11-04 00:02] Guard #99 begins shift".parse().unwrap(),
            "[1518-11-04 00:36] falls asleep".parse().unwrap(),
            "[1518-11-04 00:46] wakes up".parse().unwrap(),
            "[1518-11-05 00:03] Guard #99 begins shift".parse().unwrap(),
            "[1518-11-05 00:45] falls asleep".parse().unwrap(),
            "[1518-11-05 00:55] wakes up".parse().unwrap(),
        ];

        logs.sort();

        let (id, _) = find_sleepiest_minute(&logs, 10);
        assert_eq!(24, id);
    }

    #[test]
    fn sleepiest_times_minute_input() {
        let logs = read_logs("inputs\\day04.txt").unwrap();

        let guard_id = find_sleepiest_guard(&logs);
        let (minute, _) = find_sleepiest_minute(&logs, guard_id);

        assert_eq!(77084, guard_id * minute);
    }

    #[test]
    fn find_sleepiest_guard_minute_sample() {
        let mut logs: Vec<GuardLog> = vec![
            "[1518-11-01 00:00] Guard #10 begins shift".parse().unwrap(),
            "[1518-11-01 00:05] falls asleep".parse().unwrap(),
            "[1518-11-01 00:25] wakes up".parse().unwrap(),
            "[1518-11-01 00:30] falls asleep".parse().unwrap(),
            "[1518-11-01 00:55] wakes up".parse().unwrap(),
            "[1518-11-01 23:58] Guard #99 begins shift".parse().unwrap(),
            "[1518-11-02 00:40] falls asleep".parse().unwrap(),
            "[1518-11-02 00:50] wakes up".parse().unwrap(),
            "[1518-11-03 00:05] Guard #10 begins shift".parse().unwrap(),
            "[1518-11-03 00:24] falls asleep".parse().unwrap(),
            "[1518-11-03 00:29] wakes up".parse().unwrap(),
            "[1518-11-04 00:02] Guard #99 begins shift".parse().unwrap(),
            "[1518-11-04 00:36] falls asleep".parse().unwrap(),
            "[1518-11-04 00:46] wakes up".parse().unwrap(),
            "[1518-11-05 00:03] Guard #99 begins shift".parse().unwrap(),
            "[1518-11-05 00:45] falls asleep".parse().unwrap(),
            "[1518-11-05 00:55] wakes up".parse().unwrap(),
        ];

        logs.sort();

        let (guard_id, minute) = find_sleepiest_guard_minute(&logs);

        assert_eq!(99, guard_id);
        assert_eq!(45, minute);
    }

    #[test]
    fn find_sleepiest_guard_minute_input() {
        let logs = read_logs("inputs\\day04.txt").unwrap();
        let (guard_id, minute) = find_sleepiest_guard_minute(&logs);

        assert_eq!(23047, guard_id * minute);
    }

}
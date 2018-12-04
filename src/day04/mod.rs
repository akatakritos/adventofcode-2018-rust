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
    let mut sleep_times: HashMap<u32, u32> = HashMap::new();

    let intervals = sleep_intervals(&logs);
    for interval in intervals.iter() {
        let sleep_amount = sleep_times.entry(interval.guard_id).or_insert(0);
        *sleep_amount += interval.minutes;
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


    let intervals = sleep_intervals(&logs);
    let guard_intervals = intervals.iter()
        .filter(|i| i.guard_id == guard_id);

    for interval in guard_intervals {
        let start_minute = interval.started_at.minute();
        let end_minute = start_minute + interval.minutes;

        for minute in start_minute..end_minute {
            let count = sleep_map.entry(minute).or_insert(0);
            *count += 1;
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
        .filter(|log| log.is_begin_shift())
        .map(|log| log.unwrap_guard_id())
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

fn sleep_intervals(logs: &Vec<GuardLog>) -> Vec<SleepInterval> {
    let mut current_guard: Option<u32> = None;
    let mut started_at: Option<DateTime<Utc>> = None;
    let mut results: Vec<SleepInterval> = vec![];

    for log in logs.iter() {
        match log.log_type {
            GuardLogType::BeginShift(guard_id) => current_guard = Some(guard_id),
            GuardLogType::Sleep => started_at = Some(log.utc),
            GuardLogType::Wake => {
                let duration = log.utc.signed_duration_since(started_at.unwrap());

                results.push(SleepInterval{
                    guard_id: current_guard.unwrap(),
                    minutes: duration.num_minutes() as u32,
                    started_at: started_at.unwrap()
                });
            }
        }
    }

    results
}

struct SleepInterval {
    guard_id: u32,
    minutes: u32,
    started_at: DateTime<Utc>
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
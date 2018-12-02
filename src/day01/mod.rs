use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;
use std::collections::HashSet;

pub fn read_input(filename: &str) -> Result<Vec<i32>, io::Error> {
    let f = File::open(filename)?;
    let reader = BufReader::new(&f);
    let mut result: Vec<i32> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        if line.len() > 0 {
            let value: i32 = line.parse().unwrap();
            result.push(value);
        }
    }

    Ok(result)
}

pub fn calculate_frequency(filename: &str) -> Result<i32, Box<Error>> {
    let frequencies = read_input(filename)?;
    let sum = frequencies.iter().sum();

    Ok(sum)
}

pub fn find_first_duplicate_frequency(filename: &str) -> Result<i32, Box<Error>> {
    let frequencies = read_input(filename)?;
    let mut past_frequences = HashSet::new();
    let mut current_frequency = 0;
    const MAX_ITERS: i32 = 1000;

    for i in 1..MAX_ITERS {
        for freq in frequencies.iter() {
            current_frequency += *freq;

            if past_frequences.contains(&current_frequency) {
                println!("Found match after {} iteratations and {} unqiue frequencies",
                    i, past_frequences.len());
                return Ok(current_frequency);
            } else {
                past_frequences.insert(current_frequency);
            }
        }
    }

    panic!(format!("Could not find a repeat in {} iterations", MAX_ITERS));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_input_handles_file() {
        let results = read_input("inputs\\day01.txt").unwrap();
        assert_eq!(963, results.len());
    }

    #[test]
    fn calculate_frequency_gets_correct_result() {
        let frequency = calculate_frequency("inputs\\day01.txt").unwrap();
        assert_eq!(411, frequency);
    }

    #[test]
    fn find_first_duplicate_frequency_gets_it_right() {
        let dupe = find_first_duplicate_frequency("inputs\\day01.txt")
            .unwrap();
        assert_eq!(56360, dupe);
    }
}
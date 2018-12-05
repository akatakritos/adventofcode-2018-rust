use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

pub fn read_file(filename: &str) -> Result<Vec<char>, Box<dyn Error>> {
    let f = File::open(filename)?;

    let result: Vec<char> = f.bytes()
        .map(|b| b.unwrap() as char)
        .collect();

    Ok(result)
}

pub fn compress_polarities(chars: &Vec<char>) -> Vec<char> {
    // use a stack
    let mut result: Vec<char> = vec![];

    for c in chars.iter() {
        println!("{:?} - checking {}", result, *c);

        if result.len() == 0 {
            result.push(*c);
        } else if is_polar_pair(result[result.len() - 1], *c) {
            result.pop(); // remove it!
        } else {
            result.push(*c);
        }
    }

    result
}

fn is_polar_pair(c1: char, c2: char) -> bool {
    let byte1 = c1 as i8;
    let byte2 = c2 as i8;
    let diff = (byte1 - byte2).abs();
    return diff == 32;
}

pub fn find_shortest(chars: &Vec<char>) -> (char, usize) {
    let mut shortest_unit: Option<char> = None;
    let mut shortest_size: Option<usize> = None;

    let a = 'a' as u8;
    let z = 'z' as u8;

    for unit in a..z {
        let working_data = clone_without(&chars, unit as char);
        let result = compress_polarities(&working_data);

        if shortest_size.is_none() {
            shortest_size = Some(result.len());
            shortest_unit = Some(unit as char);
        } else if result.len() < shortest_size.unwrap() {
            shortest_size = Some(result.len());
            shortest_unit = Some(unit as char);
        }
    }

    (shortest_unit.unwrap(), shortest_size.unwrap())
}

fn clone_without(chars: &Vec<char>, unit: char) -> Vec<char>{
    let mut result = vec![];

    for c in chars.iter() {
        if !is_unit(unit, *c) {
            result.push(*c);
        }
    }

    result
}

fn is_unit(unit: char, c: char) -> bool {
    return c == unit || (c as i8 - unit as i8).abs() == 32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_file_gets_whole_string() {
        let chars = read_file("inputs\\day05.txt").expect("Error read_file");
        assert_eq!(50000, chars.len());
    }

    #[test]
    fn compress_polarities_sample() {
        let sample: Vec<char> = "dabAcCaCBAcCcaDA".chars().collect();
        let result = compress_polarities(&sample);
        assert_eq!(10, result.len());
    }

    #[test]
    fn compress_polarities_input() {
        let input = read_file("inputs\\day05.txt").unwrap();
        let result = compress_polarities(&input);
        assert_eq!(11152, result.len());
    }

    #[test]
    fn find_shortest_sample() {
        let sample: Vec<char> = "dabAcCaCBAcCcaDA".chars().collect();
        let (unit, length) = find_shortest(&sample);
        assert_eq!('c', unit);
        assert_eq!(4, length);
    }

    #[test]
    fn find_shortest_input() {
        let input = read_file("inputs\\day05.txt").unwrap();
        let (_, length) = find_shortest(&input);
        assert_eq!(6136, length);
    }
}
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

pub fn compress_polarities(chars: &mut Vec<char>) {
    loop {
        if !compress_single_pass(chars) {
            return;
        }
    }
}

fn compress_single_pass(chars: &mut Vec<char>) -> bool {
    let mut i = 0;
    let mut removed = false;

    while i < chars.len() - 1 {
        if is_polar_pair(chars[i], chars[i+1]) {
            chars.remove(i);
            chars.remove(i);
            removed = true;
        }

        i += 1;
    }

    removed
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
        let mut working_data = clone_without(&chars, unit as char);
        compress_polarities(&mut working_data);


        if shortest_size.is_none() {
            shortest_size = Some(working_data.len());
            shortest_unit = Some(unit as char);
        } else if working_data.len() < shortest_size.unwrap() {
            shortest_size = Some(working_data.len());
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
        let mut sample: Vec<char> = "dabAcCaCBAcCcaDA".chars().collect();
        compress_polarities(&mut sample);
        assert_eq!(10, sample.len());
    }

    #[test]
    fn compress_polarities_input() {
        let mut input = read_file("inputs\\day05.txt").unwrap();
        compress_polarities(&mut input);
        assert_eq!(11152, input.len());
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
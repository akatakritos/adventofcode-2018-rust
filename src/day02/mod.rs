use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

pub struct BoxId {
    pub id: String,
}

impl BoxId {
    pub fn new(id: String) -> BoxId {
        BoxId {
            id,
        }
    }

    fn hash(&self) -> HashMap<char, i32> {
        let mut map = HashMap::new();

        for c in self.id.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }

        map
    }

    pub fn has_exactly_two(&self) -> bool{
        self.hash()
            .values()
            .any(|count| *count == 2)
    }

    pub fn has_exactly_three(&self) -> bool {
        self.hash()
            .values()
            .any(|count| *count == 3)
    }

    pub fn count_differences_against(&self, other: &BoxId) -> usize {
        self.id.chars()
            .zip(other.id.chars())
            .filter(|(left, right)| left != right)
            .count()
    }

    pub fn common_letters_with(&self, other: &BoxId) -> String {
        let common_chars = self.id.chars()
            .zip(other.id.chars())
            .filter(|(left, right)| left == right)
            .map(|(left, _)| left);

        let mut result = String::new();
        for c in common_chars {
            result.push(c);
        }

        result
    }
}


pub fn load_boxes(filename: &str) -> Result<Vec<BoxId>, Box<Error>> {
    let f = File::open(filename)?;

    let reader = BufReader::new(f);
    let mut results = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        if line.len() > 0 {
            results.push(BoxId::new(line))
        }
    }

    Ok(results)
}

pub fn checksum(ids: Vec<BoxId>) -> i32 {
    let mut count2 = 0;
    let mut count3 = 0;

    for id in ids.iter() {
        if id.has_exactly_three() {
            count3 += 1;
        }

        if id.has_exactly_two() {
            count2 += 1;
        }
    }

    count2 * count3
}

pub fn find_correct_pair(boxes: &Vec<BoxId>) -> Option<(&BoxId, &BoxId)> {
    for b1 in boxes.iter() {
        for b2 in boxes.iter() {
            if b1.count_differences_against(&b2) == 1 {
                return Some((b1, b2))
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_boxes_gets_the_right_count() {
        let boxes = load_boxes("inputs\\day02.txt").unwrap();
        assert_eq!(250, boxes.len());
    }

    #[test]
    fn abcdef_has_exactly_two() {
        let boxid = BoxId::new(String::from("abcdef"));
        assert!(boxid.has_exactly_two() == false);
    }

    #[test]
    fn bababc_has_exactly_two() {
        let boxid = BoxId::new(String::from("bababc"));
        assert!(boxid.has_exactly_two());
    }

    #[test]
    fn bababc_has_exactly_three() {
        let boxid = BoxId::new(String::from("bababc"));
        assert!(boxid.has_exactly_three());
    }

    #[test]
    fn checksum_for_sample_gets_12() {
        let boxes: Vec<BoxId> = [
            "abcdef",
            "bababc",
            "abbcde",
            "abcccd",
            "aabcdd",
            "abcdee",
            "ababab"]
            .iter()
            .map(|s| BoxId::new(String::from(*s)))
            .collect();

        assert_eq!(12, checksum(boxes));
    }

    #[test]
    fn checksum_for_input() {
        let boxes = load_boxes("inputs\\day02.txt").unwrap();
        let sum = checksum(boxes);
        assert_eq!(5704, sum);
    }

    #[test]
    fn diff_count_for_abcde_and_axcye_is_two() {
        let box1 = BoxId::new(String::from("abcde"));
        let box2 = BoxId::new(String::from("axcye"));
        let count = box1.count_differences_against(&box2);

        assert_eq!(2, count);
    }

    #[test]
    fn find_correct_pair_for_sample() {
        let boxes: Vec<BoxId>= ["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"]
            .iter()
            .map(|s| BoxId::new(String::from(*s)))
            .collect();

        let (match1, match2) = find_correct_pair(&boxes).unwrap();

        assert_eq!(String::from("fghij"), match1.id);
        assert_eq!(String::from("fguij"), match2.id);
    }

    #[test]
    fn common_letters_with_sample_correct() {
        let b1 = BoxId::new(String::from("fghij"));
        let b2 = BoxId::new(String::from("fguij"));
        let common = b1.common_letters_with(&b2);

        assert_eq!("fgij", common);
    }

    #[test]
    fn common_letters_from_input() {
        let boxes = load_boxes("inputs\\day02.txt").unwrap();
        let (box1, box2) = find_correct_pair(&boxes).unwrap();

        let common = box1.common_letters_with(&box2);

        assert_eq!("umdryabviapkozistwcnihjqx", common);
    }
}
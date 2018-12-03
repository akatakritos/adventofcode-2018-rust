mod fabricclaim;

use std::error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use self::fabricclaim::FabricClaim;

pub fn read_input(filename: &str) -> Result<Vec<FabricClaim>, Box<error::Error>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);

    let mut results = vec![];
    for line in reader.lines() {
        let line = line.unwrap();

        if line.len() > 0 {
            let claim: FabricClaim = line.parse().unwrap();
            results.push(claim);
        }
    }

    Ok(results)
}

fn find_needed_size(claims: &Vec<FabricClaim>) -> (i32, i32) {
    let mut max_height = 0;
    let mut max_width = 0;

    for claim in claims.iter() {
        if claim.left + claim.width > max_width {
            max_width = claim.left + claim.width;
        }

        if claim.top + claim.height > max_height {
            max_height = claim.top + claim.height
        }
    }

    (max_width, max_height)
}

pub fn calculate_overlap_area(claims: &Vec<FabricClaim>) -> usize {
    let (width, height) = find_needed_size(&claims);

    let mut fabric = vec![vec![0i32; width as usize]; height as usize];

    for claim in claims.iter() {
        apply(&mut fabric, &claim);
    }

    let count = fabric.iter()
        .flat_map(|row| row.iter())
        .filter(|cell| *cell > &1)
        .count();

    count
}

fn apply(fabric: &mut Vec<Vec<i32>>, claim: &FabricClaim) {
    for x in 0..claim.width {
        for y in 0..claim.height {
            let i = (claim.top + y) as usize;
            let j = (claim.left + x) as usize;
            fabric[i][j] += 1
        }
    }
}

pub fn find_non_overlapping_claim(claims: &Vec<FabricClaim>) -> Option<&FabricClaim> {
    let (width, height) = find_needed_size(&claims);

    let mut fabric = vec![vec![0i32; width as usize]; height as usize];

    for claim in claims.iter() {
        apply(&mut fabric, &claim);
    }

    for claim in claims.iter() {
        if is_non_overlapping(claim, &fabric) {
            return Some(claim);
        }
    }

    None
}

fn is_non_overlapping(claim: &FabricClaim, fabric: &Vec<Vec<i32>>) -> bool {
    for x in 0..claim.width {
        for y in 0..claim.height {
            let i = (claim.top + y) as usize;
            let j = (claim.left + x) as usize;
            if fabric[i][j] > 1 {
                return false;
            }
        }
    }

    true
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_input_gets_line_count() {
        let claims = read_input("inputs\\day03.txt").unwrap();

        assert_eq!(1335, claims.len());
    }

    #[test]
    fn find_needed_size_gets_it_right() {
        let claims = read_input("inputs\\day03.txt").unwrap();

        let (width, height) = find_needed_size(&claims);

        assert_eq!(999, width);
        assert_eq!(998, height);
    }

    #[test]
    fn calculate_overlap_area_sample() {
        let claims = vec![
            "#1 @ 1,3: 4x4".parse().unwrap(),
            "#2 @ 3,1: 4x4".parse().unwrap(),
            "#3 @ 5,5: 2x2".parse().unwrap(),
        ];

        let area = calculate_overlap_area(&claims);

        assert_eq!(4, area);
    }

    #[test]
    fn calculate_overlap_area_is_correct() {
        let claims = read_input("inputs\\day03.txt").unwrap();
        let area = calculate_overlap_area(&claims);

        assert_eq!(110383, area);
    }

    #[test]
    fn find_non_overlapping_claim_sample() {
        let claims = vec![
            "#1 @ 1,3: 4x4".parse().unwrap(),
            "#2 @ 3,1: 4x4".parse().unwrap(),
            "#3 @ 5,5: 2x2".parse().unwrap(),
        ];

        let non_overlapper = find_non_overlapping_claim(&claims).unwrap();

        assert_eq!(3, non_overlapper.id);
    }

    #[test]
    fn find_non_overlapping_claim_real() {
        let claims = read_input("inputs\\day03.txt").unwrap();

        let non_overlapper = find_non_overlapping_claim(&claims).unwrap();

        assert_eq!(129, non_overlapper.id);
    }
}
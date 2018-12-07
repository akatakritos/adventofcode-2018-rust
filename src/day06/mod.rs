use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

type Point = (i32, i32);

pub fn find_max_enclosed_area(points: &[Point]) -> (Point, usize) {
    let maxx = points.iter().map(|(x, _)| x).max().unwrap();
    let maxy = points.iter().map(|(_, y)| y).max().unwrap();

    let maxx = *maxx as usize + 1;
    let maxy = *maxy as usize + 1;
    // println!("maxx= {}, maxy={}", maxx, maxy);

    let mut grid: Vec<Vec<Option<usize>>> = vec![vec![None; maxy]; maxx];

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            grid[x][y] = find_closest(&points, x, y);
        }
    }

    let mut infinite_indexes = HashSet::new();

    for x in 0..maxx {
        if let Some(index) = grid[x][0] {
            infinite_indexes.insert(index);
        }

        if let Some(index) = grid[x][maxy - 1] {
            infinite_indexes.insert(index);
        }
    }

    for y in 0..maxy {
        if let Some(index) = grid[0][y] {
            infinite_indexes.insert(index);
        }

        if let Some(index) = grid[maxx - 1][y] {
            infinite_indexes.insert(index);
        }
    }

    let mut finite_sizes = HashMap::new();

    for i in 0..points.len() {
        if !infinite_indexes.contains(&(i as usize)) {
            let count = finite_sizes.entry(i).or_insert(0);
            *count += count_indexes(&grid, i);;
        }
    }

    let mut min = (10000, 0);
    for (index, count) in finite_sizes.iter() {
        // println!("{} = {}", (97u8 + *index as u8) as char, count);
        let (_, max_count) = min;

        if *count > max_count {
            min = (*index, *count);
        }
    }

    // for y in 0..maxy {
    //     for x in 0..maxx {
    //         match grid[x][y] {
    //             Some(i) => print!("{}", (97u8 + i as u8) as char),
    //             None => print!(".")
    //         }
    //     }
    //     println!();
    // }

    let (index, count) = min;
    (points[index], count)
}

pub fn find_area_of_min_region(points: &Vec<Point>, max_distance: usize) -> usize {
    let maxx = points.iter().map(|(x, _)| x).max().unwrap();
    let maxy = points.iter().map(|(_, y)| y).max().unwrap();

    let maxx = *maxx as usize + 1;
    let maxy = *maxy as usize + 1;
    // println!("maxx= {}, maxy={}", maxx, maxy);

    let mut count = 0;
    for x in 0..maxx {
        for y in 0..maxy {
            let total_distance = find_total_manhattan_distance(&points, &(x as i32, y as i32));
            if total_distance < max_distance {
                count += 1;
            }
        }
    }

    count
}

fn find_total_manhattan_distance(points: &Vec<Point>, point: &Point) -> usize {
    points.iter().map(|p| manhattan_distance(p, point)).sum()
}

fn count_indexes(grid: &Vec<Vec<Option<usize>>>, index: usize) -> usize {
    let mut count = 0;

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == Some(index) {
                count += 1;
            }
        }
    }

    count
}

fn manhattan_distance(p1: &Point, p2: &Point) -> usize {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    ((x2 - x1).abs() + (y2 - y1).abs()) as usize
}

fn find_closest(points: &[Point], x: usize, y: usize) -> Option<usize> {
    let mut closest_index = 0;
    let mut shortest_distance = 10000000;
    let mut is_same = false;

    for i in 0..points.len() {
        let distance = manhattan_distance(&points[i], &(x as i32, y as i32));

        if distance < shortest_distance {
            shortest_distance = distance;
            is_same = false;
            closest_index = i;
        } else if distance == shortest_distance {
            is_same = true;
        }
    }

    if is_same {
        None
    } else {
        Some(closest_index)
    }
}

pub fn read_input(filename: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    let s = fs::read_to_string(filename)?;
    let mut result = vec![];

    for line in s.lines() {
        let parts = line.split(", ").collect::<Vec<&str>>();
        let x = parts[0].parse().unwrap();
        let y = parts[1].parse().unwrap();
        result.push((x, y));
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> Vec<Point> {
        vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)]
    }

    #[test]
    fn find_max_enclosed_area_sample() {
        let points = sample();
        let (p, count) = find_max_enclosed_area(&points);

        assert_eq!((5, 5), p);
        assert_eq!(17, count);
    }

    #[test]
    fn find_max_enclosed_area_input() {
        let points = read_input("inputs\\day06.txt").unwrap();
        let (_, count) = find_max_enclosed_area(&points);

        assert_eq!(2917, count);
    }

    #[test]
    fn find_max_region_size_sample() {
        let points = sample();
        let size = find_area_of_min_region(&points, 32);
        assert_eq!(16, size);
    }

    #[test]
    fn find_max_region_size_input() {
        let points = read_input("inputs\\day06.txt").unwrap();
        let size = find_area_of_min_region(&points, 10_000);
        assert_eq!(44202, size);
    }
}

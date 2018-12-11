extern crate adventofcode;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: adventofcode 01")
    }

    let day = args[1].parse();

    match day {
        Ok(1) => day_one(),
        Ok(2) => day_two(),
        Ok(3) => day_three(),
        Ok(4) => day_four(),
        Ok(5) => day_five(),
        Ok(6) => day_six(),
        Ok(7) => day_seven(),
        Ok(8) => day_eight(),
        Ok(9) => day_nine(),
        Ok(d) => panic!(format!("Day {} not implemented.", d)),
        Err(_) => panic!(format!("Can't parse '{}' as a day", args[1])),
    }
}

fn day_one() {
    let filename = "inputs\\day01.txt";
    let result1 = adventofcode::day01::calculate_frequency(filename).unwrap();
    println!("ending frequency: {}", result1);

    let result2 = adventofcode::day01::find_first_duplicate_frequency(filename).unwrap();
    println!("first dupe frequence: {}", result2);
}

fn day_two() {
    let filename = "inputs\\day02.txt";
    let boxes = adventofcode::day02::load_boxes(filename).unwrap();

    let result1 = adventofcode::day02::checksum(&boxes);
    println!("checksum: {}", result1);

    let (b1, b2) = adventofcode::day02::find_correct_pair(&boxes).unwrap();
    let result2 = b1.common_letters_with(&b2);
    println!("Common letters of correct boxes: {}", result2);
}

fn day_three() {
    let claims = adventofcode::day03::read_input("inputs\\day03.txt").unwrap();
    let result1 = adventofcode::day03::calculate_overlap_area(&claims);
    println!("overlapping area: {}sq inches", result1);

    let result2 = adventofcode::day03::find_non_overlapping_claim(&claims).unwrap();
    println!("non-overlapped claim: {:?}", result2);
}

fn day_four() {
    let logs = adventofcode::day04::read_logs("inputs\\day04.txt").unwrap();
    let guard_id = adventofcode::day04::find_sleepiest_guard(&logs);
    let (minute, count) = adventofcode::day04::find_sleepiest_minute(&logs, guard_id);
    println!(
        "Guard #{} slept the most, particularly during minute {} for a total of {} days",
        guard_id, minute, count
    );

    let (guard_id, minute) = adventofcode::day04::find_sleepiest_guard_minute(&logs);
    println!(
        "Guard #{} spent minute {} asleep more than any other guard.",
        guard_id, minute
    );
}

fn day_five() {
    let input = adventofcode::day05::read_file("inputs\\day05.txt").unwrap();

    let result1 = adventofcode::day05::compress_polarities(&input);
    println!(
        "After fully reacting, the polymer is {} units long.",
        result1.len()
    );

    let (unit, length) = adventofcode::day05::find_shortest(&input);
    println!(
        "Afer removing all the '{}' units, the fully reacted polymer is {} units long.",
        unit, length
    );
}

fn day_six() {
    let input = adventofcode::day06::read_input("inputs\\day06.txt").unwrap();

    let ((x, y), size) = adventofcode::day06::find_max_enclosed_area(&input);
    println!(
        "Max enclosed area is around point ({}, {}) and has size {}",
        x, y, size
    );

    let result2 = adventofcode::day06::find_area_of_min_region(&input, 10_0000);
    println!(
        "The region less than 10k manahattan distance has {} points",
        result2
    );
}

fn day_seven() {
    let input = adventofcode::day07::read_input("inputs\\day07.txt").unwrap();
    let order = adventofcode::day07::topological_sort(&input);
    let time = adventofcode::day07::work_length(&input, 5, 60);

    println!("You can process them in this order: {}", order);
    println!(
        "If it takes a base cost of 60 seconds, with 5 workers it will take {}s to complete",
        time
    );
}

fn day_eight() {
    let input = adventofcode::day08::read_file("inputs\\day08.txt");
    let checksum = adventofcode::day08::metadata_checksum(&input);
    let value = adventofcode::day08::calculate_node_value(&input);

    println!("The checksum of the license file is {}.", checksum);
    println!("The vale of the license file is {}", value);
}

fn day_nine() {
    const PLAYERS: usize = 486;
    const PART1_LAST_MARBLE: u32 = 70833;
    const PART2_LAST_MARBLE: u32 = PART1_LAST_MARBLE * 100;

    let score1 = adventofcode::day09::winning_score(PLAYERS, PART1_LAST_MARBLE);
    println!(
        "With {} players playing {} marbles, the winner will have a score of {}.",
        PLAYERS, PART1_LAST_MARBLE, score1
    );

    let score2 = adventofcode::day09::winning_score(PLAYERS, PART2_LAST_MARBLE);
    println!(
        "With {} players playing {} marbles, the winner will have a score of {}.",
        PLAYERS, PART2_LAST_MARBLE, score2
    );
}

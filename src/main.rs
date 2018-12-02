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
        Ok(d) => panic!(format!("Day {} not implemented.", d)),
        Err(_) => panic!(format!("Can't parse '{}' as a day", args[1]))
    }
}

fn day_one() {
    let filename = "inputs\\day01.txt";
    let result1 = adventofcode::day01::calculate_frequency(filename)
        .unwrap();
    println!("ending frequency: {}", result1);

    let result2 = adventofcode::day01::find_first_duplicate_frequency(filename)
        .unwrap();
    println!("first dupe frequence: {}", result2);
}

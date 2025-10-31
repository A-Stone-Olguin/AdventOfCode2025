use clap::Parser;
use std::fs;
use advent_of_code_2025::days::day_factory::day_factory;

#[derive(Parser)]
struct Arguments {
    day: u8,
}

fn day_input_help_message(day: u8) -> String {
    format!(
        "Try getting the file from https://adventofcode.com/2025/day/{}/input.\n\
         Note that you will need to be logged in.\n\
         Please put the input in the file named `input/Day{}.txt`",
        day, day
    )
}

fn main() {
    let args = Arguments::parse();

    let data = match fs::read_to_string(format!("{}/input/Day{}.txt", env!("CARGO_MANIFEST_DIR"), args.day)) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            println!("{}", day_input_help_message(args.day));
            return;
        }
    };

    let day = day_factory(args.day);

    println!("Day {} Part 1 Result: Result: {}", args.day, day.part1(&data));
    println!("Day {} Part 2 Result: Result: {}", args.day, day.part2(&data));
}


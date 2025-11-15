use clap::Parser;
use std::vec;
use std::time::SystemTime;
use advent_of_code_2025::days::day_factory::day_factory;
use advent_of_code_2025::db::sqlite::Sqlite;
use advent_of_code_2025::db::record::write_timings_to_readme;
use advent_of_code_2025::utils::get_input_data::get_input_data;

#[derive(Parser)]
struct Arguments {
    day: u8,
    number_iterations: u32
}

fn main() {
    let args = Arguments::parse();

    let day = day_factory(args.day);

    let data = match get_input_data(args.day) {
        Ok(data) => data,
        Err(e) => panic!("{e}"),
    };

    if args.number_iterations == 0 {
        println!("Only printing results");
        println!("Day {} Part 1 Result: Result: {}", args.day, day.part1(&data));
        println!("Day {} Part 2 Result: Result: {}", args.day, day.part2(&data));
        return;
    }

    let mut db = match Sqlite::new() {
        Ok(db) => db,
        Err(e) => panic!("Failed to initialize database. Error: {}", e),
    };

    let mut prepared_statements = match db.prepare_stmts() {
        Ok(stmts) => stmts,
        Err(e) => panic!("Failed to prepare statements. Error: {}", e),
    };

    let mut timings_part1: Vec<i64> = vec![];
    let mut timings_part2: Vec<i64> = vec![];
    for _ in 0..args.number_iterations {
        let mut start = SystemTime::now();
        day.part1(&data);
        let duration = match start.elapsed() {
            Ok(elapsed) => elapsed.as_millis() as i64,
            Err(_) => 0,
        };
        timings_part1.push(duration);

        start = SystemTime::now();
        day.part2(&data);
        let duration = match start.elapsed() {
            Ok(elapsed) => elapsed.as_millis() as i64,
            Err(_) => 0,
        };
        timings_part2.push(duration);
    }
    for (i, vector) in [timings_part1, timings_part2].iter().enumerate() {
        match prepared_statements.insert_timings(args.day, (i+1) as u8, vector) {
            Ok(_) => println!("Finished recording timing data for Day {} Part {}\n", args.day, i+1),
            Err(e) => println!("Failed to insert timing. Error: {}", e),
        }
    }

    // Write to README
    let results = match prepared_statements.get_timings() {
        Ok(results) => results,
        Err(e) => panic!("Failed to read timings. Error: {}", e),
    };
    match write_timings_to_readme(results) {
        Ok(_) => println!("Finished writing timings to README"),
        Err(_) => panic!("Failed to write timings to README"),
    }
}


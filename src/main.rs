use clap::Parser;
use std::{fs, vec};
use std::fmt::Write;
use std::time::SystemTime;
use advent_of_code_2025::days::day_factory::day_factory;
use advent_of_code_2025::db::sqlite::{Sqlite};

#[derive(Parser)]
struct Arguments {
    day: u8,
    number_iterations: u32
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

    let mut md_table = String::new();

    let results = match prepared_statements.get_timings() {
        Ok(results) => results,
        Err(e) => panic!("Failed to read timings. Error: {}", e),
    };

    writeln!(md_table, "| Day | Part | Min Time (ms) | Median Time (ms) | Max Time (ms) | Number Iterations |").expect("Expected to write header");
    writeln!(md_table, "| --- | ---- | ------------- | ---------------- | ------------- | ----------------- |").expect("Expected to write header separator");
    
    for result in results {
        match writeln!(md_table, "| {} | {} | {} | {} | {} | {} |",
            result.day,
            result.part,
            result.min_time_ms,
            result.median_time_ms,
            result.max_time_ms,
            result.number_iterations,
        ) {
            Ok(_) => continue,
            Err(e) => panic!("Failed to write to README. Error: {}", e),
        }
    }

    let readme_path = "README.md";
    let content = match fs::read_to_string(readme_path)  {
        Ok(content) => content,
        Err(e) => panic!("Failed to read README. Error: {}", e),
    };

    let start_marker = "<!-- START_TIMINGS_TABLE -->";
    let end_marker = "<!-- END_TIMINGS_TABLE -->";

    // Find the start and end positions
    let start_idx = content.find(start_marker).expect("Start marker not found") + start_marker.len();
    let end_idx = content.find(end_marker).expect("End marker not found");

    // Build the new content
    let new_content = format!(
        "{}\n{}\n{}",
        &content[..start_idx],
        md_table.trim(),
        &content[end_idx..]
    );

    match fs::write(readme_path, new_content)  {
        Ok(_) => println!("README updated!"),
        Err(e) => panic!("Failed to updated README: Error: {}", e),
    }
}


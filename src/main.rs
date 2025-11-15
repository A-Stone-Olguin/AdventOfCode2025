use advent_of_code_2025::traits::timing_repository::TimingRepository;
use clap::Parser;
use advent_of_code_2025::days::day_factory::day_factory;
use advent_of_code_2025::db::sqlite::connection::Sqlite;
use advent_of_code_2025::utils::get_input_data::get_input_data;
use advent_of_code_2025::timings::{time_day::time_day, record_timings::write_timings_to_readme};

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

    let mut timing_repository: Box<dyn TimingRepository<_>> = match Sqlite::new() {
        Ok(repo) => Box::new(repo),
        Err(e) => panic!("Could not start database. Error: {e}"),
    };

    match time_day(args.number_iterations, args.day, &data, &day, &mut timing_repository) {
        Ok(_) => println!("Successfully recorded timings for day {}", args.day),
        Err(e) => panic!("Could not record timings for day {}. Error: {}", args.day, e),
    }

    // Write to README
    let results = match timing_repository.get_timings() {
        Ok(results) => results,
        Err(e) => panic!("Failed to read timings. Error: {}", e),
    };
    match write_timings_to_readme(results) {
        Ok(_) => println!("Finished writing timings to README"),
        Err(_) => panic!("Failed to write timings to README"),
    }
}

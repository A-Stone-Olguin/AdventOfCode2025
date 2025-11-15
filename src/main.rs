use advent_of_code_2025::traits::timing_repository::TimingRepository;
use clap::Parser;
use advent_of_code_2025::days::day_factory::day_factory;
use advent_of_code_2025::db::sqlite::connection::Sqlite;
use advent_of_code_2025::db::record::write_timings_to_readme;
use advent_of_code_2025::utils::get_input_data::get_input_data;
use advent_of_code_2025::db::record_timings::record_timings;

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

    let timings_part1 = record_timings(args.number_iterations, &data, |input| day.part1(input));
    let timings_part2 = record_timings(args.number_iterations, &data, |input| day.part2(input));

    for (i, vector) in [timings_part1, timings_part2].iter().enumerate() {
        let day_id: i64 = (2*(args.day-1) + i as u8) as i64;

        match timing_repository.delete_day_timings(day_id) {
            Ok(_) => println!("Finished deleting old timing data for Day {} Part {}", args.day, i+1),
            Err(e) => println!("Failed to insert timing. Error: {}", e),
        }
        match timing_repository.insert_timings(day_id, vector) {
            Ok(_) => println!("Finished recording timing data for Day {} Part {}\n", args.day, i+1),
            Err(e) => println!("Failed to insert timing. Error: {}", e),
        }
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

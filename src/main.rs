use advent_of_code_rust::cli::arguments;
use advent_of_code_rust::db::sqlite::connection::Sqlite;
use advent_of_code_rust::timings::{record_timings::write_timings_to_readme, time_day::time_day};
use advent_of_code_rust::traits::timing_repository::TimingRepository;
use advent_of_code_rust::utils::get_input_data::get_input_data;
use advent_of_code_rust::years::year_factory::year_factory;

fn main() {
    let args = arguments::parse_arguments();

    let mut timing_repository: Box<dyn TimingRepository<_>> = match Sqlite::new() {
        Ok(repo) => Box::new(repo),
        Err(e) => panic!("Could not start database. Error: {e}"),
    };

    if args.record_timings {
        println!("Recording timings");
        let results = match timing_repository.get_timings() {
            Ok(results) => results,
            Err(e) => panic!("Failed to read timings. Error: {}", e),
        };
        match write_timings_to_readme(results) {
            Ok(_) => println!("Finished writing timings to README"),
            Err(_) => panic!("Failed to write timings to README"),
        }
        return;
    }

    let year = year_factory(args.year.clone());
    let day = year.day_factory(args.day);

    let data = match get_input_data(args.year.clone().into(), args.day) {
        Ok(data) => data,
        Err(e) => panic!("{e}"),
    };

    if args.number_iterations == 0 {
        println!(
            "Day {} Part 1 Result: Result: {}",
            args.day,
            day.part1(&data)
        );
        println!(
            "Day {} Part 2 Result: Result: {}",
            args.day,
            day.part2(&data)
        );
    } else {
        match time_day(
            args.number_iterations,
            args.day,
            &data,
            day.as_ref(),
            args.year.into(),
            &mut timing_repository,
        ) {
            Ok(_) => println!("Successfully recorded timings for day {}", args.day),
            Err(e) => panic!(
                "Could not record timings for day {}. Error: {}",
                args.day, e
            ),
        }
    }
}

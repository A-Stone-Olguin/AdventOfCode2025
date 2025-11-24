use std::fmt::Display;

use crate::{
    cli::arguments::ParsedArguments,
    timings::{record_timings::write_timings_to_readme, time_day::time_day},
    traits::{day::Day, timing_repository::TimingRepository},
    utils::get_input_data::get_input_data,
};

pub fn handle_record_timings<E: Display>(timing_repository: &mut Box<dyn TimingRepository<E>>) {
    println!("Recording timings");
    let results = match timing_repository.get_timings() {
        Ok(results) => results,
        Err(e) => panic!("Failed to read timings. Error: {}", e),
    };
    match write_timings_to_readme(results) {
        Ok(_) => println!("Finished writing timings to README"),
        Err(_) => panic!("Failed to write timings to README"),
    }
}

pub fn run_day<E: Display>(
    args: &ParsedArguments,
    year_num: u32,
    day_num: u8,
    day: &dyn Day,
    timing_repository: &mut Box<dyn TimingRepository<E>>,
) {
    let data = match get_input_data(year_num, day_num) {
        Ok(data) => data,
        Err(e) => {
            if args.run_all {
                return;
            }
            panic!("{e}");
        }
    };

    if args.number_iterations == 0 {
        println!(
            "Year {}, Day {} Part 1 Result: Result: {}",
            year_num,
            day_num,
            day.part1(&data)
        );
        println!(
            "Year {}, Day {} Part 2 Result: Result: {}\n",
            year_num,
            day_num,
            day.part2(&data)
        );
    } else {
        match time_day(
            args.number_iterations,
            day_num,
            &data,
            day,
            year_num,
            timing_repository,
        ) {
            Ok(_) => println!(
                "Successfully recorded timings for year {}, day {}",
                year_num, day_num
            ),
            Err(e) => panic!(
                "Could not record timings for year {}, day {}. Error: {}",
                year_num, day_num, e
            ),
        }
    }
}

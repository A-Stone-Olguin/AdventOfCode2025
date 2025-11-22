use advent_of_code_rust::cli::arguments;
use advent_of_code_rust::db::sqlite::connection::Sqlite;
use advent_of_code_rust::timings::{record_timings::write_timings_to_readme, time_day::time_day};
use advent_of_code_rust::traits::timing_repository::TimingRepository;
use advent_of_code_rust::traits::year::Year;
use advent_of_code_rust::utils::get_input_data::get_input_data;
use advent_of_code_rust::years::year::ValidYear;
use advent_of_code_rust::years::year_factory::year_factory;
use strum::IntoEnumIterator;

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

    let years: Vec<(ValidYear, Box<dyn Year>)> = if args.run_all {
        ValidYear::iter()
            .map(|y| (y.clone(), year_factory(y)))
            .collect()
    } else {
        vec![(args.year.clone(), year_factory(args.year.clone()))]
    };

    for (year_enum, year) in years {
        let days = if args.run_all {
            year.get_all_days()
        } else {
            vec![year.day_factory(args.day)]
        };
        let year_num: u32 = year_enum.into();

        for (i, day) in days.iter().enumerate() {
            let day_num = i as u8 + 1;
            let data = match get_input_data(year_num, day_num) {
                Ok(data) => data,
                Err(e) => {
                    if args.run_all {
                        continue;
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
                    day.as_ref(),
                    year_num,
                    &mut timing_repository,
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
    }
}

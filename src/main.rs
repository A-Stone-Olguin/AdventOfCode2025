use advent_of_code_rust::db::sqlite::connection::Sqlite;
use advent_of_code_rust::timings::{record_timings::write_timings_to_readme, time_day::time_day};
use advent_of_code_rust::traits::timing_repository::TimingRepository;
use advent_of_code_rust::utils::get_input_data::get_input_data;
use advent_of_code_rust::years::year::ValidYear;
use advent_of_code_rust::years::year_factory::year_factory;
use clap::Parser;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use strum::IntoEnumIterator;

#[derive(Parser)]
struct Arguments {
    day: u8,

    #[arg(short, long, value_enum)]
    year: Option<ValidYear>,

    #[arg(short, long)]
    number_iterations: Option<u32>,

    #[arg(short, long, default_value_t = false)]
    record_timings: bool,
}

fn main() {
    let args = Arguments::parse();

    let valid_year = match args.year {
        Some(year) => year,
        None => {
            let options: Vec<u32> = ValidYear::iter().map(|x| x.into()).collect();

            let idx = Select::new()
                .with_prompt("Choose a year")
                .items(&options)
                .default(0)
                .interact()
                .unwrap();
            options[idx].into()
        }
    };

    let num_iters = match args.number_iterations {
        Some(iters) => iters,
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to time a number of iterations? Enter '0' for no")
            .default(0)
            .allow_empty(true)
            .interact()
            .unwrap(),
    };

    let year = year_factory(valid_year.clone());
    let day = year.day_factory(args.day);

    let data = match get_input_data(valid_year.clone().into(), args.day) {
        Ok(data) => data,
        Err(e) => panic!("{e}"),
    };

    let mut timing_repository: Box<dyn TimingRepository<_>> = match Sqlite::new() {
        Ok(repo) => Box::new(repo),
        Err(e) => panic!("Could not start database. Error: {e}"),
    };

    if num_iters == 0 {
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
            num_iters,
            args.day,
            &data,
            day.as_ref(),
            valid_year.into(),
            &mut timing_repository,
        ) {
            Ok(_) => println!("Successfully recorded timings for day {}", args.day),
            Err(e) => panic!(
                "Could not record timings for day {}. Error: {}",
                args.day, e
            ),
        }
    }

    if args.record_timings {
        let results = match timing_repository.get_timings() {
            Ok(results) => results,
            Err(e) => panic!("Failed to read timings. Error: {}", e),
        };
        match write_timings_to_readme(results) {
            Ok(_) => println!("Finished writing timings to README"),
            Err(_) => panic!("Failed to write timings to README"),
        }
    }
}

use advent_of_code_2025::days::day_factory::day_factory;
use advent_of_code_2025::db::sqlite::connection::Sqlite;
use advent_of_code_2025::timings::{record_timings::write_timings_to_readme, time_day::time_day};
use advent_of_code_2025::traits::timing_repository::TimingRepository;
use advent_of_code_2025::utils::get_input_data::get_input_data;
use clap::Parser;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};

#[derive(clap::ValueEnum, Clone)]
enum ValidYear {
    TwentyFive,
    TwentyFour,
}

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
    let mut args = Arguments::parse();

    if args.year.is_none() {
        let options = [ValidYear::TwentyFive, ValidYear::TwentyFour];
        let idx = Select::new()
            .with_prompt("Choose a year")
            .items(["2025", "2024"])
            .default(0)
            .interact()
            .unwrap();
        args.year = Some(options[idx].clone());
    }

    if args.number_iterations.is_none() {
        let num_iters = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to time a number of iterations? Enter '0' for no")
            .default(0)
            .allow_empty(true)
            .interact()
            .unwrap();
        args.number_iterations = Some(num_iters)
    }

    let day = day_factory(args.day);

    let data = match get_input_data(args.day) {
        Ok(data) => data,
        Err(e) => panic!("{e}"),
    };

    let mut timing_repository: Box<dyn TimingRepository<_>> = match Sqlite::new() {
        Ok(repo) => Box::new(repo),
        Err(e) => panic!("Could not start database. Error: {e}"),
    };

    let num_iters = args.number_iterations.unwrap_or_default();

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

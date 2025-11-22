use crate::years::year::{ValidYear, valid_days};
use clap::Parser;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use strum::IntoEnumIterator;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    day: Option<u8>,

    #[arg(short, long, value_enum)]
    year: Option<ValidYear>,

    #[arg(short, long)]
    number_iterations: Option<u32>,

    #[arg(short, long, default_value_t = false)]
    record_timings: bool,

    #[arg(short, long, default_value_t = false)]
    all: bool,
}

pub struct ParsedArguments {
    pub day: u8,
    pub year: ValidYear,
    pub record_timings: bool,
    pub number_iterations: u32,
    pub run_all: bool,
}

pub fn parse_arguments() -> ParsedArguments {
    let args = Arguments::parse();

    if args.record_timings
        && (args.all
            || args.day.is_some()
            || args.year.is_some()
            || args.number_iterations.is_some())
    {
        panic!("--record-timings cannot be combined with other flags or values");
    }

    if args.all && (args.day.is_some() || args.year.is_some()) {
        panic!("--run-all cannot be used with --day or --year");
    }

    let skip_day_year_prompt = args.record_timings || args.all;

    let valid_year = if skip_day_year_prompt {
        ValidYear::TwentyFive
    } else {
        match args.year {
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
        }
    };

    let day = if skip_day_year_prompt {
        0
    } else {
        let max_day = valid_days(valid_year.clone());
        match args.day {
            Some(d) => d,
            None => Input::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "Which day would you like? Choose from 1 to {}",
                    max_day
                ))
                .allow_empty(false)
                .validate_with(move |input: &u8| -> Result<(), &str> {
                    if input >= &1 && input <= &max_day {
                        Ok(())
                    } else {
                        Err("The provided value is not in the range.")
                    }
                })
                .interact()
                .unwrap(),
        }
    };

    let num_iters = if args.record_timings {
        0
    } else {
        match args.number_iterations {
            Some(iters) => iters,
            None => Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to time a number of iterations? Enter '0' for no")
                .default(0)
                .allow_empty(true)
                .interact()
                .unwrap(),
        }
    };

    ParsedArguments {
        day,
        year: valid_year,
        record_timings: args.record_timings,
        number_iterations: num_iters,
        run_all: args.all,
    }
}

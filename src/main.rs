use advent_of_code_rust::cli::{
    arguments,
    helpers::{handle_record_timings, run_day},
};
use advent_of_code_rust::{
    db::sqlite::connection::Sqlite,
    traits::{timing_repository::TimingRepository, year::Year},
    years::{year::ValidYear, year_factory::year_factory},
};
use strum::IntoEnumIterator;

fn main() {
    let args = arguments::parse_arguments();

    let mut timing_repository: Box<dyn TimingRepository<_>> = match Sqlite::new() {
        Ok(repo) => Box::new(repo),
        Err(e) => panic!("Could not start database. Error: {e}"),
    };

    if args.record_timings {
        handle_record_timings(&mut timing_repository);
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
            run_day(
                &args,
                year_num,
                day_num,
                day.as_ref(),
                &mut timing_repository,
            );
        }
    }
}

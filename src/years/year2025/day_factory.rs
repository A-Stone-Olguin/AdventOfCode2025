use crate::traits::{day::Day, year::Year};
use crate::years::year2025::*;

pub struct Year2025 {}

impl Year for Year2025 {
    fn day_factory(&self, day_number: u8) -> Box<dyn Day> {
        match_days!(day_number,
            1 => Day1,
            2 => Day2,
            3 => Day3,
            4 => Day4,
            5 => Day5,
            6 => Day6,
            7 => Day7,
            8 => Day8,
            9 => Day9,
            10 => Day10,
            11 => Day11,
            12 => Day12
        )
    }

    fn get_all_days(&self) -> Vec<Box<dyn Day>> {
        list_days!(
            Day1, Day2, Day3, Day4, Day5, Day6, Day7, Day8, Day9, Day10, Day11, Day12
        )
    }
}

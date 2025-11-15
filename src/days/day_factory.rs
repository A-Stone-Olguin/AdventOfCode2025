use crate::traits::day::Day;
use crate::days::*;

pub fn day_factory(day_number: u8) -> Box<dyn Day> {
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

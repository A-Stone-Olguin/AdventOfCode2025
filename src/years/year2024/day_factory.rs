use crate::traits::{day::Day, year::Year};
use crate::years::year2024::*;

pub struct Year2024 {}

impl Year for Year2024 {
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
            12 => Day12,
            13 => Day13,
            14 => Day14,
            15 => Day15,
            16 => Day16,
            17 => Day17,
            18 => Day18,
            19 => Day19,
            20 => Day20,
            21 => Day21,
            22 => Day22,
            23 => Day23,
            24 => Day24,
            25 => Day25
        )
    }

    fn get_all_days(&self) -> Vec<Box<dyn Day>> {
        list_days!(
            Day1, Day2, Day3, Day4, Day5, Day6, Day7, Day8, Day9, Day10, Day11, Day12, Day13,
            Day14, Day15, Day16, Day17, Day18, Day19, Day20, Day21, Day22, Day23, Day24, Day25
        )
    }
}

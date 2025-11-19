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
}

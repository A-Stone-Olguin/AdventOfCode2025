use regex::Regex;

use crate::{
    traits::day::Day,
    utils::parse_input_data::{matches_per_row, parse_input_to_rows},
};

#[derive(Default)]
pub struct Day5;

fn parse_ranges_ingredients(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let lines = parse_input_to_rows(input);
    let regex = Regex::new(r"\d+").unwrap();
    let matches = matches_per_row(lines, &regex);

    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut ingredients: Vec<u64> = vec![];
    for m in matches {
        if m.len() == 0 {
            continue;
        } else if m.len() == 1 {
            ingredients.push(m[0].parse().unwrap());
        } else {
            ranges.push((m[0].parse().unwrap(), m[1].parse().unwrap()))
        }
    }

    (ranges, ingredients)
}

// NOTE: Very unoptimized currently
fn get_fresh_count((ranges, ingredients): &(Vec<(u64, u64)>, Vec<u64>)) -> u32 {
    let mut fresh_count = 0;
    for ingredient in ingredients {
        for (lo, hi) in ranges {
            if ingredient >= lo && ingredient <= hi {
                fresh_count += 1;
                break;
            }
        }
    }
    fresh_count
}

fn get_cleaned_ranges(ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_by(|(a1, _), (a2, _)| a1.cmp(a2));

    let mut cleaned_up_ranges: Vec<(u64, u64)> = vec![];

    for (_, (lo, hi)) in ranges.iter().enumerate() {
        if cleaned_up_ranges.is_empty() {
            cleaned_up_ranges.push((*lo, *hi));
            continue;
        }
        for (i, (lo2, hi2)) in cleaned_up_ranges.clone().iter().enumerate() {
            // Subsumed
            if lo >= lo2 && hi <= hi2 {
                break;
            }

            // Consumed
            if lo <= lo2 && hi >= hi2 {
                cleaned_up_ranges[i] = (*lo, *hi);
                break;
            }

            // Overlap left
            if lo < lo2 && lo2 <= hi {
                cleaned_up_ranges[i] = (*lo, *hi2);
                break;
            }

            // Overlap right
            if hi > hi2 && lo <= hi2 {
                cleaned_up_ranges[i] = (*lo2, *hi);
                break;
            }

            // No ranges fit, create new entry
            if i == cleaned_up_ranges.len() - 1 {
                cleaned_up_ranges.push((*lo, *hi));
            }
        }
    }
    cleaned_up_ranges
}

fn get_all_fresh_veggies(cleaned_ranges: &[(u64, u64)]) -> u64 {
    cleaned_ranges
        .iter()
        .fold(0, |prev, (lo, hi)| prev + *hi - *lo + 1)
}

impl Day for Day5 {
    fn part1(&self, input: &str) -> String {
        let range_ingredients = parse_ranges_ingredients(input);
        get_fresh_count(&range_ingredients).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (mut ranges, _) = parse_ranges_ingredients(input);
        let clean_ranges = get_cleaned_ranges(&mut ranges);
        get_all_fresh_veggies(&clean_ranges).to_string()
        // get_total_fresh_veggies(&ranges).to_string()
    }
}

#[cfg(test)]
mod day5_tests_2025 {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
    3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32
    "#;

    #[test]
    fn test_2025_day5_part1() {
        let day = Day5::default();
        assert_eq!(day.part1(EXAMPLE_INPUT), "3");
    }

    #[test]
    fn test_2025_day5_part2() {
        let day = Day5::default();
        assert_eq!(day.part2(EXAMPLE_INPUT), "14");
    }
}

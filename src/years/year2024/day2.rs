use regex::Regex;

use crate::{
    traits::day::Day,
    utils::parse_input_data::{self, matches_per_row},
};

#[derive(Default)]
pub struct Day2;

fn safe_constraints() -> impl FnMut(&&i32, &&i32) -> bool {
    move |a, b| a < b && **b - **a <= 3
}

fn safe_constraints_v2(prev: i32, curr: i32) -> bool {
    prev < curr && curr - prev <= 3
}

fn get_sequence(input: &str) -> Vec<Vec<i32>> {
    let lines = parse_input_data::parse_input_to_rows(input);
    let regex = Regex::new(r"\d+").unwrap();
    let matches = matches_per_row(lines, &regex);

    matches
        .iter()
        .map(|row| {
            row.iter()
                .map(|num| num.parse().unwrap_or_default())
                .collect()
        })
        .collect()
}

fn is_safe(sequence: &[i32]) -> bool {
    sequence.iter().is_sorted_by(safe_constraints())
        || sequence.iter().rev().is_sorted_by(safe_constraints())
}

fn is_safe_increasing(sequence: &[i32]) -> bool {
    sequence.iter().is_sorted_by(safe_constraints())
}

fn is_safe_with_dampening(sequence: &[i32]) -> bool {
    let has_failure = sequence
        .windows(2)
        .enumerate()
        .find(|(_, vals)| !safe_constraints_v2(vals[0], vals[1]))
        .map(|(i, _)| i + 1);

    match has_failure {
        Some(i) => {
            // We don't know if the first or second element was the one that caused the issue. We can try both
            is_safe_increasing(&[&sequence[..i], &sequence[i + 1..]].concat())
                || is_safe_increasing(&[&sequence[..i - 1], &sequence[i..]].concat())
        }
        None => true,
    }
}

fn is_safe_wrapper(sequence: &[i32]) -> bool {
    match is_safe_with_dampening(sequence) {
        true => true,
        false => {
            let mut clone = sequence.to_owned();
            clone.reverse();
            is_safe_with_dampening(&clone)
        }
    }
}

fn count_safe_levels(input: &str) -> i32 {
    let sequences = get_sequence(input);

    sequences
        .iter()
        .fold(0, |acc, level| acc + is_safe(level) as i32)
}

fn count_dampened_safe_levels(input: &str) -> i32 {
    let sequences = get_sequence(input);

    sequences
        .iter()
        .fold(0, |acc, level| acc + is_safe_wrapper(level) as i32)
}

impl Day for Day2 {
    fn part1(&self, input: &str) -> String {
        let safe_levels = count_safe_levels(input);
        format!("{}", safe_levels)
    }

    fn part2(&self, input: &str) -> String {
        format!("{}", count_dampened_safe_levels(input))
    }
}

#[cfg(test)]
mod day2_tests_2024 {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "#;

    #[test]
    fn test_2024_day2_part1() {
        let day = Day2::default();
        assert_eq!(day.part1(EXAMPLE_INPUT), "2");
    }

    #[test]
    fn test_2024_day2_part2() {
        let day = Day2::default();
        assert_eq!(day.part2(EXAMPLE_INPUT), "4");
    }
}

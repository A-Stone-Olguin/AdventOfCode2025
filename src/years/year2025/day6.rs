use ndarray::{Array2, Axis};
use regex::Regex;

use crate::{
    traits::day::Day,
    utils::parse_input_data::{matches_per_row, parse_input_to_grid, parse_input_to_rows},
};

#[derive(Default)]
pub struct Day6;

enum Operation {
    Multiply,
    Add,
}

fn parse_values(input: &str) -> Array2<String> {
    let rows = parse_input_to_rows(input);
    let regex = Regex::new(r"\S+").unwrap();
    let matches = matches_per_row(rows, &regex);

    let rows = matches.len();
    let cols = matches.first().map(|row| row.len()).unwrap_or(0);

    assert!(matches.iter().all(|row| row.len() == cols));

    let flat: Vec<String> = matches.into_iter().flatten().collect();
    Array2::from_shape_vec((rows, cols), flat).unwrap()
}

fn get_grand_total(problem_grid: &Array2<String>) -> u64 {
    let mut sum = 0;
    for col in problem_grid.axis_iter(Axis(1)) {
        let op = match col.last().unwrap().as_str() {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => panic!("Invalid operation"),
        };
        let mut temp = match op {
            Operation::Multiply => 1,
            Operation::Add => 0,
        };
        for (i, val) in col.iter().enumerate() {
            if i == col.len() - 1 {
                break;
            }
            match op {
                Operation::Multiply => temp *= val.parse::<u64>().unwrap(),
                Operation::Add => temp += val.parse::<u64>().unwrap(),
            };
        }
        sum += temp;
    }
    sum
}

fn parse_to_char_array(input: &str) -> Array2<char> {
    parse_input_to_grid(input)
}

fn get_grand_total_v2(problem_grid: &Array2<char>) -> u64 {
    let mut sum = 0;
    let mut current_operation: Option<Operation> = None;
    let mut temp_value = 0;

    for col in problem_grid.axis_iter(Axis(1)) {
        if col.iter().all(|v| *v == ' ') {
            sum += temp_value;
            current_operation = None;
            continue;
        }

        if current_operation.is_none() {
            current_operation = match col.last().unwrap() {
                '*' => Some(Operation::Multiply),
                '+' => Some(Operation::Add),
                _ => panic!("Shouldn't have happened!"),
            };
            temp_value = match current_operation {
                Some(Operation::Multiply) => 1,
                Some(Operation::Add) => 0,
                None => panic!("What happened in the previous?"),
            };
        }

        let col_str: String = col.iter().take(col.len() - 1).copied().collect();
        match current_operation {
            Some(Operation::Multiply) => temp_value *= col_str.trim().parse::<u64>().unwrap(),
            Some(Operation::Add) => temp_value += col_str.trim().parse::<u64>().unwrap(),
            None => panic!("Something went wrong in assigning operation"),
        };
    }
    // Add the last temp_value
    sum + temp_value
}

impl Day for Day6 {
    fn part1(&self, input: &str) -> String {
        let grid = parse_values(input);
        get_grand_total(&grid).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let grid = parse_to_char_array(input);
        get_grand_total_v2(&grid).to_string()
    }
}

#[cfg(test)]
mod day6_tests_2025 {
    use super::*;

    // NOTE: Kinda modified because leading spaces were trimmed.
    const EXAMPLE_INPUT: &str = r#"
    328  51 64  123
    64  387 23   45
    98  215 314   6
    +   *   +   *
    "#;

    #[test]
    fn test_2025_day6_part1() {
        let day = Day6::default();
        assert_eq!(day.part1(EXAMPLE_INPUT), "4277556");
    }

    #[test]
    fn test_2025_day6_part2() {
        let day = Day6::default();
        assert_eq!(day.part2(EXAMPLE_INPUT), "3263827");
    }
}

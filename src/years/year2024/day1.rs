use std::collections::HashMap;

use regex::Regex;

use crate::{traits::day::Day, utils::parse_input_data};

#[derive(Default)]
pub struct Day1;

fn get_sorted_pairs(input: &str) -> Vec<(u32, u32)> {
    let rows = parse_input_data::parse_input_to_rows(input);
    let regex = Regex::new(r"\d+").unwrap();
    let matches = parse_input_data::matches_per_row(rows, &regex);

    let mut vec1: Vec<u32> = vec![];
    let mut vec2: Vec<u32> = vec![];
    for row in matches {
        if row.len() != 2 {
            panic!("Incorrectly got too many matches")
        }
        vec1.push(row[0].parse().unwrap_or_default());
        vec2.push(row[1].parse().unwrap_or_default());
    }
    if vec1.len() != vec2.len() {
        panic!("The vectors are not the same length!")
    }
    vec1.sort();
    vec2.sort();

    let mut paired_vec: Vec<(u32, u32)> = vec![];
    for (i, _) in vec1.iter().enumerate() {
        paired_vec.push((vec1[i], vec2[i]));
    }
    paired_vec
}

fn get_counted_lists(input: &str) -> (Vec<u32>, HashMap<u32, u32>) {
    let rows = parse_input_data::parse_input_to_rows(input);
    let regex = Regex::new(r"\d+").unwrap();
    let matches = parse_input_data::matches_per_row(rows, &regex);

    let mut vec: Vec<u32> = vec![];
    let mut map: HashMap<u32, u32> = HashMap::new();
    for row in matches {
        if row.len() != 2 {
            panic!("Incorrectly got too many matches")
        }
        vec.push(row[0].parse().unwrap_or_default());

        let row_val: u32 = row[1].parse().unwrap_or_default();
        match map.get(&row_val) {
            Some(count) => map.insert(row_val, count + 1),
            None => map.insert(row_val, 1),
        };
    }
    (vec, map)
}

fn compute_total_distance(paired_vec: Vec<(u32, u32)>) -> u32 {
    paired_vec
        .iter()
        .fold(0, |acc, (left, right)| acc + left.abs_diff(*right))
}

fn compute_similarity_score((vec, map): (Vec<u32>, HashMap<u32, u32>)) -> u32 {
    vec.iter()
        .fold(0, |acc, val| acc + val * map.get(val).unwrap_or(&0))
}

impl Day for Day1 {
    fn part1(&self, input: &str) -> String {
        let pairs = get_sorted_pairs(input);
        compute_total_distance(pairs).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let counted_lists = get_counted_lists(input);
        compute_similarity_score(counted_lists).to_string()
    }
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    "#;

    #[test]
    fn test_2024_day1_part1() {
        let day = Day1::default();
        assert_eq!(day.part1(EXAMPLE_INPUT), "11");
    }

    #[test]
    fn test_2024_day1_part2() {
        let day = Day1::default();
        assert_eq!(day.part2(EXAMPLE_INPUT), "31");
    }
}

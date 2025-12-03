use std::collections::HashMap;

use crate::{traits::day::Day, utils::parse_input_data::parse_input_to_rows};

#[derive(Default)]
pub struct Day9;

fn uncompact_files(input: &str) -> (Vec<Option<usize>>, HashMap<usize, u32>) {
    let binding = parse_input_to_rows(input);
    let line = binding.first().unwrap();

    let mut uncompact: Vec<Option<usize>> = vec![];
    let mut map: HashMap<usize, u32> = HashMap::new();
    for (i, c) in line.char_indices() {
        let parsed_int = c.to_digit(10).unwrap();
        let value_to_add = match i % 2 == 0 {
            true => Some(i / 2),
            false => None,
        };

        for _ in 0..parsed_int {
            uncompact.push(value_to_add);
        }

        map.insert(i / 2, parsed_int);

        // uncompact.append(&mut (0..parsed_int).map(|_| value_to_add).collect());
    }
    (uncompact, map)
}

fn move_file_blocks(uncompact: &mut [Option<usize>]) -> &mut [Option<usize>] {
    let mut left = 0;
    let mut right = uncompact.len() - 1;

    while left < right {
        // Find first available 'None'
        while uncompact[left].is_some() {
            left += 1;
        }

        // Find first available 'Some'
        while uncompact[right].is_none() {
            right -= 1;
        }

        if left >= right {
            break;
        }

        uncompact.swap(left, right);
    }

    uncompact
}

fn move_grouped_blocks(
    uncompact: &mut [Option<usize>],
    value_to_move: usize,
) -> &mut [Option<usize>] {
    if value_to_move <= 1 {
        return uncompact;
    }

    let mut left = 0;
    let mut right = uncompact.len() - 1;

    // Find where the value starts
    while left < right {
        match uncompact[right] {
            Some(value) => {
                if value == value_to_move {
                    break;
                }
                right -= 1
            }
            None => right -= 1,
        }
    }

    // Record the indices to move
    let mut indices_to_move: Vec<usize> = vec![];
    while let Some(i) = uncompact[right] {
        if i != value_to_move {
            break;
        }
        indices_to_move.push(right);
        right -= 1;
    }

    // Determine eligible empty space
    let mut empty_count = 0;
    while left <= right {
        match uncompact[left] {
            Some(_) => {
                empty_count = 0;
                left += 1;
            }
            None => {
                empty_count += 1;
                // Swap when we have enough
                if empty_count >= indices_to_move.len() {
                    for (i, right_index) in indices_to_move.iter().enumerate() {
                        uncompact.swap(left - i, *right_index);
                    }
                    break;
                }
                left += 1;
            }
        }
    }

    move_grouped_blocks(uncompact, value_to_move - 1)
}

fn calculate_checksum(compacted_files: &[Option<usize>]) -> u64 {
    let mut checksum: u64 = 0;
    for (i, v) in compacted_files.iter().enumerate() {
        checksum += match v {
            Some(v) => (i * v) as u64,
            None => 0,
        }
    }

    checksum
}

impl Day for Day9 {
    fn part1(&self, input: &str) -> String {
        let (mut uncompacted_files, _) = uncompact_files(input);
        let compacted_files = move_file_blocks(&mut uncompacted_files);
        calculate_checksum(compacted_files).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (mut uncompacted_files, _) = uncompact_files(input);
        let highest_value = input.trim().len() / 2;
        let compacted_files = move_grouped_blocks(&mut uncompacted_files, highest_value);
        calculate_checksum(compacted_files).to_string()
    }
}

#[cfg(test)]
mod day9_tests_2024 {
    use super::*;

    const EXAMPLE_INPUT: &str = "2333133121414131402";
    const EXAMPLE_INPUT2: &str = "12344";

    #[test]
    fn test_2024_day9_part1() {
        let day = Day9::default();
        assert_eq!(day.part1(EXAMPLE_INPUT), "1928");
    }

    #[test]
    fn test_2024_day9_part2() {
        let day = Day9::default();
        assert_eq!(day.part2(EXAMPLE_INPUT), "2858");
        println!("{}", day.part2(EXAMPLE_INPUT2))
    }
}

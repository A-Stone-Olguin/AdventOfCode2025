use regex::Regex;

use crate::{traits::day::Day, utils::parse_input_data};

#[derive(Default)]
pub struct Day1;

enum Turn {
    Left,
    Right,
}

fn parse_turns(input: &str) -> Vec<(Turn, i32)> {
    let rows = parse_input_data::parse_input_to_rows(input);
    let regex = Regex::new(r"(L|R)(\d+)").unwrap();
    let captures = parse_input_data::captures_per_row(rows, &regex);

    let mut vec: Vec<(Turn, i32)> = vec![];
    for capture in captures {
        let inserted_value = match capture.get(0).unwrap().get(0).unwrap().as_str() {
            "L" => Turn::Left,
            "R" => Turn::Right,
            _ => panic!("Got invalid value"),
        };
        let turn_amount: i32 = capture.get(0).unwrap().get(1).unwrap().parse().unwrap();
        vec.push((inserted_value, turn_amount));
    }
    vec
}

fn calculate_turns(turns: &[(Turn, i32)], initial_value: i32) -> i32 {
    let mut zero_count = 0;
    let mut current_value = initial_value;
    for (direction, clicks) in turns {
        let change_value: i32 = match direction {
            Turn::Left => -1 * clicks,
            Turn::Right => *clicks,
        };
        current_value = (current_value + change_value) % 100;
        if current_value == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

fn count_zero_occurrences(turns: &[(Turn, i32)], initial_value: i32) -> i32 {
    let mut zero_count = 0;
    let mut current_value = initial_value;
    for (direction, clicks) in turns {
        let count_threshold: i32 = match direction {
            Turn::Left => current_value,
            Turn::Right => 100 - current_value,
        };

        // Any "cycles" around
        let cycles = clicks / 100;
        zero_count += cycles;

        // Get any "intermediate clicks", don't count 0 twice
        if *clicks - cycles * 100 >= count_threshold && count_threshold != 0 {
            zero_count += 1;
        }

        // TODO CLeanup and remove
        let change_value: i32 = match direction {
            Turn::Left => -1 * clicks,
            Turn::Right => *clicks,
        };
        current_value = (current_value + change_value).rem_euclid(100);
    }

    zero_count
}

fn turn_amount(current_value: i32, (direction, clicks): &(Turn, i32)) -> i32 {
    let change_value: i32 = match direction {
        Turn::Left => -1 * clicks,
        Turn::Right => *clicks,
    };
    (current_value + change_value).rem_euclid(100)
}

fn calculate_zeroes(turns: &[(Turn, i32)], initial_value: i32) -> i32 {
    turns
        .iter()
        .fold((initial_value, 0), |(location, count), turn| {
            let new_position = turn_amount(location, turn);
            (new_position, count + (new_position == 0) as i32)
        })
        .1
}

fn number_zero_clicks_in_turn((direction, clicks): &(Turn, i32), current_value: i32) -> i32 {
    let mut zero_count = 0;
    let count_threshold: i32 = match direction {
        Turn::Left => current_value,
        Turn::Right => 100 - current_value,
    };

    // Any "cycles" around
    let cycles = clicks / 100;
    zero_count += cycles;

    // Get any "intermediate clicks", don't count 0 twice
    if *clicks - cycles * 100 >= count_threshold && count_threshold != 0 {
        zero_count += 1;
    }
    zero_count
}

fn count_zero_clicks(turns: &[(Turn, i32)], initial_value: i32) -> i32 {
    turns
        .iter()
        .fold((initial_value, 0), |(location, count), turn| {
            (
                turn_amount(location, turn),
                count + number_zero_clicks_in_turn(turn, location),
            )
        })
        .1
}

impl Day for Day1 {
    fn part1(&self, input: &str) -> String {
        let turns = parse_turns(input);
        calculate_zeroes(&turns, 50).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let turns = parse_turns(input);
        count_zero_clicks(&turns, 50).to_string()
    }
}

#[cfg(test)]
mod day1_2025_tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
    L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82
    "#;

    const EXAMPLE_INPUT2: &str = r#"
    R1050
    "#;

    const EXAMPLE_INPUT3: &str = r#"
    R1000
    "#;

    #[test]
    fn test_2025_day1_part1() {
        let day = Day1::default();
        assert_eq!(day.part1(EXAMPLE_INPUT), "3")
    }

    #[test]
    fn test_2025_day1_part2() {
        let day = Day1::default();
        assert_eq!(day.part2(EXAMPLE_INPUT), "6");
        assert_eq!(day.part2(EXAMPLE_INPUT2), "11");
        assert_eq!(day.part2(EXAMPLE_INPUT3), "10")
    }
}

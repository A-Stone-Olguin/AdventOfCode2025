use std::collections::HashSet;

use ndarray::Array2;

use crate::{
    traits::day::Day,
    utils::{parse_input_data::parse_input_to_grid, point::Point},
};

#[derive(Default)]
pub struct Day7;

fn get_starting_position<'a>(grid: &'a Array2<char>) -> Point<'a> {
    let ((row, col), _) = grid.indexed_iter().find(|&(_, &c)| c == 'S').unwrap();
    Point::new(grid, col, row)
}

fn mark_visited_splits<'a>(start_point: &Point<'a>, visited_splits: &mut HashSet<(usize, usize)>) {
    let mut previous_point = *start_point;
    let split_char = &'^';
    while let Some(point) = previous_point.down() {
        if point.value().unwrap() == split_char {
            visited_splits.insert((point.y, point.x));
            if let Some(left_split) = point.left() {
                mark_visited_splits(&left_split, visited_splits);
            }
            if let Some(right_split) = point.right() {
                mark_visited_splits(&right_split, visited_splits);
            }
            break;
        } else {
            previous_point = point;
        }
    }
}

impl Day for Day7 {
    fn part1(&self, input: &str) -> String {
        let grid = parse_input_to_grid(input);
        let start_point = get_starting_position(&grid);
        let mut visited_points = HashSet::<(usize, usize)>::new();
        mark_visited_splits(&start_point, &mut visited_points);
        visited_points.len().to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "Unimplemented part2.".to_string()
    }
}

#[cfg(test)]
mod day7_tests_2025 {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
    .......S.......
    ...............
    .......^.......
    ...............
    ......^.^......
    ...............
    .....^.^.^.....
    ...............
    ....^.^...^....
    ...............
    ...^.^...^.^...
    ...............
    ..^...^.....^..
    ...............
    .^.^.^.^.^...^.
    ...............
    "#;

    #[test]
    fn test_2025_day7_part1() {
        let day = Day7::default();
        assert_eq!(day.part1(EXAMPLE_INPUT), "21");
    }

    #[test]
    fn test_2025_day7_part2() {
        let day = Day7::default();
        assert_eq!(day.part2(EXAMPLE_INPUT), "3263827");
    }
}

use ndarray::Array2;

use crate::{
    traits::day::Day,
    utils::{parse_input_data::parse_input_to_grid, point::Point},
};

#[derive(Default)]
pub struct Day4;

fn is_accessible<'a>(point: &Point<'a>, value: &char) -> bool {
    if *value != '@' {
        return false;
    }

    let mut neighbor_count = 0;
    for (i, neighbor) in point.neighbors().iter().enumerate() {
        // Don't count self
        if i == 4 {
            continue;
        }
        match neighbor {
            Some(n) => neighbor_count += (**n == '@') as i32,
            None => continue,
        }
    }
    neighbor_count < 4
}

fn count_accessible(grid: &Array2<char>) -> u64 {
    let mut accessible_count = 0;
    for ((row, col), value) in grid.indexed_iter() {
        let point = Point::new(grid, row, col);
        if is_accessible(&point, value) {
            accessible_count += 1;
        }
    }
    accessible_count
}

fn count_removable(grid: &mut Array2<char>) -> u64 {
    let mut removed_count = 0;

    let mut roll_idxs: Vec<(usize, usize)> = vec![];
    for (idx, value) in grid.indexed_iter() {
        if *value == '@' {
            roll_idxs.push(idx);
        }
    }

    loop {
        let mut count = 0;
        for (row, col) in &roll_idxs {
            let point = Point::new(grid, *row, *col);
            let value = point.value().unwrap();
            if is_accessible(&point, value) {
                grid[(*row, *col)] = '.';
                count += 1;
            }
        }
        if count == 0 {
            break;
        }
        removed_count += count;
    }

    removed_count
}

impl Day for Day4 {
    fn part1(&self, input: &str) -> String {
        let grid = parse_input_to_grid(input);
        count_accessible(&grid).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut grid = parse_input_to_grid(input);
        count_removable(&mut grid).to_string()
    }
}

#[cfg(test)]
mod day4_tests_2025 {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.
    "#;

    #[test]
    fn test_2025_day4_part1() {
        let day = Day4::default();
        assert_eq!(day.part1(EXAMPLE_INPUT), "13");
    }

    #[test]
    fn test_2025_day4_part2() {
        let day = Day4::default();
        assert_eq!(day.part2(EXAMPLE_INPUT), "43");
    }
}

use regex::Regex;

use crate::{
    traits::day::Day,
    utils::parse_input_data::{matches_per_row, parse_input_to_rows},
};

#[derive(Default)]
pub struct Day9;

fn parse_points(input: &str) -> Vec<(u64, u64)> {
    let rows = parse_input_to_rows(input);
    let regex = Regex::new(r"\d+").unwrap();
    let matches = matches_per_row(rows, &regex);
    matches
        .iter()
        .map(|pair_vec| (pair_vec[0].parse().unwrap(), pair_vec[1].parse().unwrap()))
        .collect()
}

fn calculate_areas(points: &[(u64, u64)]) -> Vec<u64> {
    let mut areas: Vec<u64> = vec![];
    // O(n^2), also bad will clean up another time
    for (i, (x1, y1)) in points.iter().enumerate() {
        for (x2, y2) in points.iter().skip(i + 1) {
            let y_length = y1.abs_diff(*y2) + 1;
            let x_length = x1.abs_diff(*x2) + 1;
            areas.push(y_length * x_length);
        }
    }
    areas.sort_by(|a, b| b.cmp(a));
    areas
}

fn make_boundary_lines(red_tiles: &Vec<(u64, u64)>) -> Vec<((u64, u64), (u64, u64))> {
    let mut v = vec![];
    for window in red_tiles.windows(2).into_iter() {
        let (x1, y1) = window[0];
        let (x2, y2) = window[1];
        if x1 == x2 {
            let (first, last) = match y1 <= y2 {
                true => (y1, y2),
                false => (y2, y1),
            };
            v.push(((x1, first), (x2, last)));
        } else {
            let (first, last) = match x1 <= x2 {
                true => (x1, x2),
                false => (x2, x1),
            };
            v.push(((first, y1), (last, y2)));
        }
    }

    // Do first and last
    let &(x1, y1) = red_tiles.first().unwrap();
    let &(x2, y2) = red_tiles.last().unwrap();
    if x1 == x2 {
        let (first, last) = match y1 <= y2 {
            true => (y1, y2),
            false => (y2, y1),
        };
        v.push(((x1, first), (x2, last)));
    } else {
        let (first, last) = match x1 <= x2 {
            true => (x1, x2),
            false => (x2, x1),
        };
        v.push(((first, y1), (last, y2)));
    }

    v
}

fn is_valid_area_v4(
    (x1, y1): &(u64, u64),
    (x2, y2): &(u64, u64),
    boundary_lines: &[((u64, u64), (u64, u64))],
) -> bool {
    // VERY INEFFICIENT RIGHT NOW
    let (&min_x, &max_x) = match x1 <= x2 {
        true => (x1, x2),
        false => (x2, x1),
    };
    let (&min_y, &max_y) = match y1 <= y2 {
        true => (y1, y2),
        false => (y2, y1),
    };

    for ((min_lin_x, min_lin_y), (max_lin_x, max_lin_y)) in boundary_lines {
        if max_lin_x == min_lin_x {
            if min_x < *max_lin_x && *max_lin_x < max_x {
                // Check line intersects
                if (*min_lin_y <= min_y && *max_lin_y > min_y)
                    || (*min_lin_y < max_y && *max_lin_y >= max_y)
                {
                    return false;
                }
            }
        } else {
            if min_y < *max_lin_y && *max_lin_y < max_y {
                // Check line intersects
                if (*min_lin_x <= min_x && *max_lin_x > min_x)
                    || (*min_lin_x < max_x && *max_lin_x >= max_x)
                {
                    return false;
                }
            }
        }
    }

    true
}

fn calculate_red_green_areas_v4(
    red_points: &[(u64, u64)],
    lines: &[((u64, u64), (u64, u64))],
) -> u64 {
    let mut max_area = 0;
    for (i, (x1, y1)) in red_points.iter().enumerate() {
        for (x2, y2) in red_points.iter().skip(i + 1) {
            let y_length = y1.abs_diff(*y2) + 1;
            let x_length = x1.abs_diff(*x2) + 1;
            let proposed_area = x_length * y_length;
            if proposed_area > max_area && is_valid_area_v4(&(*x1, *y1), &(*x2, *y2), lines) {
                max_area = proposed_area;
            }
        }
    }
    max_area
}

impl Day for Day9 {
    fn part1(&self, input: &str) -> String {
        let points = parse_points(input);
        calculate_areas(&points).first().unwrap().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let points = parse_points(input);

        let boundary_lines = make_boundary_lines(&points);
        calculate_red_green_areas_v4(&points, &boundary_lines).to_string()
    }
}

#[cfg(test)]
mod day9_tests_2025 {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
    7,1
    11,1
    11,7
    9,7
    9,5
    2,5
    2,3
    7,3
    "#;

    #[test]
    fn test_2025_day9_part1() {
        let day = Day9::default();
        assert_eq!(day.part1(EXAMPLE_INPUT), "50");
    }

    #[test]
    fn test_2025_day9_part2() {
        let day = Day9::default();
        assert_eq!(day.part2(EXAMPLE_INPUT), "24");
    }
}

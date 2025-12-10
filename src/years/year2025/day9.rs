use std::collections::{HashMap, HashSet};

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

// TODO: RENAME TO BOUNDARY
fn get_green_tile_boundary(red_tiles: &Vec<(u64, u64)>) -> HashSet<(u64, u64)> {
    let mut green_boundary_points: HashSet<(u64, u64)> = HashSet::new();

    for window in red_tiles.windows(2).into_iter() {
        let (x1, y1) = window[0];
        let (x2, y2) = window[1];
        if x1 == x2 {
            let (first, last) = match y1 <= y2 {
                true => (y1, y2),
                false => (y2, y1),
            };
            for i in first..last + 1 {
                green_boundary_points.insert((x1, i));
            }
        } else {
            let (first, last) = match x1 <= x2 {
                true => (x1, x2),
                false => (x2, x1),
            };
            for i in first..last + 1 {
                green_boundary_points.insert((i, y1));
            }
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
        for i in first..last + 1 {
            green_boundary_points.insert((x1, i));
        }
    } else {
        let (first, last) = match x1 <= x2 {
            true => (x1, x2),
            false => (x2, x1),
        };
        for i in first..last + 1 {
            green_boundary_points.insert((i, y1));
        }
    }
    green_boundary_points
}

fn make_green_tiles(
    boundary_points: &HashSet<(u64, u64)>,
    red_points: &[(u64, u64)],
) -> HashSet<(u64, u64)> {
    let mut max_x = &0;
    let mut max_y = &0;
    for (x, y) in red_points {
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }

    let mut green_points = HashSet::new();
    for x in 0..max_x + 1 {
        for y in 0..max_y + 1 {
            if boundary_points.get(&(x, y)).is_some() {
                green_points.insert((x, y));
            }
            if is_inside_boundary(&(x, y), boundary_points) {
                green_points.insert((x, y));
            }
        }
    }
    green_points
}

fn is_inside_boundary((x1, y1): &(u64, u64), boundary_points: &HashSet<(u64, u64)>) -> bool {
    if boundary_points.get(&(*x1, *y1)).is_some() {
        return true;
    }

    // 0,X is not in shape, we can count from there (count number of times we cross shape)
    //      even => outside
    //      odd  => inside
    boundary_points
        .iter()
        .filter(|(x2, y2)| y1 == y2 && x1 < x2)
        .count()
        % 2
        == 1
}

fn is_valid_area(
    (x1, y1): &(u64, u64),
    (x2, y2): &(u64, u64),
    green_points: &HashSet<(u64, u64)>,
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

    for i in min_y..max_y + 1 {
        if !is_inside_boundary(&(min_x, i), green_points)
            || !is_inside_boundary(&(max_x, i), green_points)
        {
            return false;
        }
    }

    for i in min_x..max_x + 1 {
        if !is_inside_boundary(&(i, min_y), green_points)
            || !is_inside_boundary(&(i, max_y), green_points)
        {
            return false;
        }
    }
    true
}

fn is_valid_area_v2(
    (x1, y1): &(u64, u64),
    (x2, y2): &(u64, u64),
    green_points: &HashSet<(u64, u64)>,
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

    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            if green_points.get(&(x, y)).is_none() {
                return false;
            }
        }
    }

    true
}

fn is_valid_area_v3(
    (x1, y1): &(u64, u64),
    (x2, y2): &(u64, u64),
    boundary_points: &HashSet<(u64, u64)>,
    is_inside: &mut HashMap<(u64, u64), bool>,
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

    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            if let Some(&exists) = is_inside.get(&(x, y)) {
                if !exists {
                    return false;
                }
            }
            if is_inside_boundary(&(x, y), boundary_points) {
                is_inside.insert((x, y), true);
            } else {
                is_inside.insert((x, y), false);
                return false;
            }
        }
    }

    true
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
                todo!()
            }
        }
    }

    true
}

fn calculate_red_green_areas(
    red_points: &[(u64, u64)],
    boundary_points: &HashSet<(u64, u64)>,
) -> Vec<u64> {
    let mut areas: Vec<u64> = vec![];
    // O(n^2), also bad will clean up another time
    for (i, (x1, y1)) in red_points.iter().enumerate() {
        println!("{i}/{}", red_points.len() - 1);
        for (x2, y2) in red_points.iter().skip(i + 1) {
            let y_length = y1.abs_diff(*y2) + 1;
            let x_length = x1.abs_diff(*x2) + 1;
            if !is_valid_area(&(*x1, *y1), &(*x2, *y2), boundary_points) {
                continue;
            }
            areas.push(y_length * x_length);
        }
    }
    areas.sort_by(|a, b| b.cmp(a));
    areas
}

fn calculate_red_green_areas_v2(
    red_points: &[(u64, u64)],
    green_points: &HashSet<(u64, u64)>,
) -> Vec<u64> {
    let mut areas: Vec<u64> = vec![];
    // O(n^2), also bad will clean up another time
    for (i, (x1, y1)) in red_points.iter().enumerate() {
        println!("{i}/{}", red_points.len() - 1);
        for (x2, y2) in red_points.iter().skip(i + 1) {
            if !is_valid_area_v2(&(*x1, *y1), &(*x2, *y2), green_points) {
                continue;
            }
            let y_length = y1.abs_diff(*y2) + 1;
            let x_length = x1.abs_diff(*x2) + 1;
            areas.push(y_length * x_length);
        }
    }
    areas.sort_by(|a, b| b.cmp(a));
    areas
}

fn calculate_red_green_areas_v3(
    red_points: &[(u64, u64)],
    green_points: &HashSet<(u64, u64)>,
) -> u64 {
    let mut is_inside: HashMap<(u64, u64), bool> = HashMap::new();
    let mut max_area = 0;

    // O(n^2), also bad will clean up another time
    for (i, (x1, y1)) in red_points.iter().enumerate() {
        println!("{i}/{}", red_points.len() - 1);
        for (x2, y2) in red_points.iter().skip(i + 1) {
            let y_length = y1.abs_diff(*y2) + 1;
            let x_length = x1.abs_diff(*x2) + 1;
            let proposed_area = x_length * y_length;
            if proposed_area > max_area
                && is_valid_area_v3(&(*x1, *y1), &(*x2, *y2), green_points, &mut is_inside)
            {
                max_area = proposed_area;
            }
        }
    }
    max_area
}

fn calculate_red_green_areas_v4(
    red_points: &[(u64, u64)],
    green_points: &HashSet<(u64, u64)>,
) -> u64 {
    todo!()
    // let mut max_area = 0;
    // // O(n^2), also bad will clean up another time
    // for (i, (x1, y1)) in red_points.iter().enumerate() {
    //     println!("{i}/{}", red_points.len()-1);
    //     for (x2, y2) in red_points.iter().skip(i+1) {
    //         let y_length = y1.abs_diff(*y2)+1;
    //         let x_length = x1.abs_diff(*x2)+1;
    //         let proposed_area = x_length*y_length;
    //         if proposed_area > max_area && is_valid_area_v4(&(*x1, *y1), &(*x2, *y2), re) {
    //             max_area = proposed_area;
    //         }
    //     }
    // }
    // max_area
}

impl Day for Day9 {
    fn part1(&self, input: &str) -> String {
        let points = parse_points(input);
        calculate_areas(&points).first().unwrap().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let points = parse_points(input);

        // let boundary_points = get_green_tile_boundary(&points);
        // calculate_red_green_areas(&points, &boundary_points).first().unwrap().to_string()

        // let boundary_points = get_green_tile_boundary(&points);
        // let green_points = make_green_tiles(&boundary_points, &points);
        // calculate_red_green_areas_v2(&points, &green_points).first().unwrap().to_string()

        let boundary_points = get_green_tile_boundary(&points);
        calculate_red_green_areas_v3(&points, &boundary_points).to_string()
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

use std::collections::{HashMap, HashSet};

use crate::{
    traits::day::Day,
    utils::parse_input_data::{matches_per_row, parse_input_to_rows},
};
use ndarray::{Array1, arr1};
use regex::Regex;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Point3D {
    vec: Array1<i64>,
}

impl Point3D {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Point3D {
            vec: arr1(&[x, y, z]),
        }
    }

    fn distance(self, other_point: Point3D) -> f64 {
        let diff = self.vec - other_point.vec;
        f64::sqrt(diff.dot(&diff) as f64)
    }
}

fn parse_points(input: &str) -> Vec<Point3D> {
    let rows = parse_input_to_rows(input);
    // No negatives
    let regex = Regex::new(r"\d+").unwrap();
    let matches = matches_per_row(rows, &regex);

    matches
        .iter()
        .map(|coords| {
            Point3D::new(
                coords[0].parse().unwrap(),
                coords[1].parse().unwrap(),
                coords[2].parse().unwrap(),
            )
        })
        .collect()
}

fn calculate_distances(points: &[Point3D]) -> Vec<((Point3D, Point3D), f64)> {
    // O(n^2), gonna try for better later
    let mut distances: Vec<((Point3D, Point3D), f64)> = vec![];
    for (i, first_point) in points.iter().enumerate() {
        for second_point in points.iter().skip(i + 1) {
            distances.push((
                (first_point.clone(), second_point.clone()),
                first_point.clone().distance(second_point.clone()),
            ));
        }
    }
    distances.sort_by(|(_, a), (_, b)| a.total_cmp(b));
    distances
}

fn make_circuits(sorted_distances: &[((Point3D, Point3D), f64)], max_runs: usize) -> usize {
    let ((first, second), _) = sorted_distances.first().unwrap();
    let mut circuits: Vec<HashSet<Point3D>> = vec![HashSet::from([first.clone(), second.clone()])];

    let mut circuit_map: HashMap<Point3D, usize> =
        HashMap::from([(first.clone(), 0_usize), (second.clone(), 0_usize)]);

    let mut i = 1;
    while i < max_runs {
        let ((first, second), _) = sorted_distances.get(i).unwrap();
        match (circuit_map.get(first), circuit_map.get(second)) {
            (None, None) => {
                let new_index = circuits.len();
                circuits.push(HashSet::from([first.clone(), second.clone()]));
                circuit_map.insert(first.clone(), new_index);
                circuit_map.insert(second.clone(), new_index);
            }
            (None, Some(&circuit_index)) => {
                circuits[circuit_index].insert(first.clone());
                circuit_map.insert(first.clone(), circuit_index);
            }
            (Some(&circuit_index), None) => {
                circuits[circuit_index].insert(second.clone());
                circuit_map.insert(second.clone(), circuit_index);
            }
            (Some(&c1), Some(&c2)) => {
                if c1 != c2 {
                    // Merge sets together
                    let (target, source) = if circuits[c1].len() >= circuits[c2].len() {
                        (c1, c2)
                    } else {
                        (c2, c1)
                    };

                    let moved_points: Vec<Point3D> = circuits[source].iter().cloned().collect();
                    for point in &moved_points {
                        circuit_map.insert(point.clone(), target);
                        circuits[target].insert(point.clone());
                    }
                    circuits[source].clear();
                }
            }
        }
        i += 1;
    }

    if circuits.len() < 3 {
        return 0;
    }
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    circuits[0].len() * circuits[1].len() * circuits[2].len()
}

fn is_one_circuit(circuits: &[HashSet<Point3D>], total_amount: usize) -> bool {
    circuits.iter().find(|&p| p.len() == total_amount).is_some()
}

fn make_circuits_v2(sorted_distances: &[((Point3D, Point3D), f64)], total_points: usize) -> i64 {
    let ((first, second), _) = sorted_distances.first().unwrap();
    let mut circuits: Vec<HashSet<Point3D>> = vec![HashSet::from([first.clone(), second.clone()])];

    let mut circuit_map: HashMap<Point3D, usize> =
        HashMap::from([(first.clone(), 0_usize), (second.clone(), 0_usize)]);

    let mut i = 1;

    let mut first: &Point3D = first;
    let mut second: &Point3D = second;
    while !is_one_circuit(&circuits, total_points) {
        let (points, _) = sorted_distances.get(i).unwrap();
        first = &points.0;
        second = &points.1;
        match (circuit_map.get(first), circuit_map.get(second)) {
            (None, None) => {
                let new_index = circuits.len();
                circuits.push(HashSet::from([first.clone(), second.clone()]));
                circuit_map.insert(first.clone(), new_index);
                circuit_map.insert(second.clone(), new_index);
            }
            (None, Some(&circuit_index)) => {
                circuits[circuit_index].insert(first.clone());
                circuit_map.insert(first.clone(), circuit_index);
            }
            (Some(&circuit_index), None) => {
                circuits[circuit_index].insert(second.clone());
                circuit_map.insert(second.clone(), circuit_index);
            }
            (Some(&c1), Some(&c2)) => {
                if c1 != c2 {
                    // Merge sets together
                    let (target, source) = if circuits[c1].len() >= circuits[c2].len() {
                        (c1, c2)
                    } else {
                        (c2, c1)
                    };

                    let moved_points: Vec<Point3D> = circuits[source].iter().cloned().collect();
                    for point in &moved_points {
                        circuit_map.insert(point.clone(), target);
                        circuits[target].insert(point.clone());
                    }
                    circuits[source].clear();
                }
            }
        }
        i += 1;
    }
    first.vec[0] * second.vec[0]
}

#[derive(Default)]
pub struct Day8;

impl Day for Day8 {
    fn part1(&self, input: &str) -> String {
        let points = parse_points(input);
        let sorted_distances = &calculate_distances(&points);
        let max_runs: usize;
        if points.len() >= 1000 {
            max_runs = 1000;
        } else {
            max_runs = 10;
        }
        make_circuits(sorted_distances, max_runs).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let points = parse_points(input);
        let sorted_distances = &calculate_distances(&points);
        make_circuits_v2(sorted_distances, points.len()).to_string()
    }
}

#[cfg(test)]
mod day8_tests_2025 {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
    162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689
    "#;

    #[test]
    fn test_2025_day8_part1() {
        let day = Day8::default();
        assert_eq!(day.part1(EXAMPLE_INPUT), "40");
    }

    #[test]
    fn test_2025_day8_part2() {
        let day = Day8::default();
        assert_eq!(day.part2(EXAMPLE_INPUT), "25272");
    }
}

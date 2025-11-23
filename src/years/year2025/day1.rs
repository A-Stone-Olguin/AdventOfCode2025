use crate::traits::day::Day;

#[derive(Default)]
pub struct Day1;

impl Day for Day1 {
    fn part1(&self, _input: &str) -> String {
        format!("Unimplemented part1.")
    }

    fn part2(&self, _input: &str) -> String {
        format!("Unimplemented part2.")
    }
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "INPUT";

    #[test]
    fn test_day1_part1() {
        let day = Day1::default();
        assert_ne!(day.part1(EXAMPLE_INPUT), "tmp")
    }
}
